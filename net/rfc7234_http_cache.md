#Hypertext Transfer Protocol (HTTP/1.1): Caching
##Introduction
   An HTTP cache is a local store of response messages and the subsystem
   that controls storage, retrieval, and deletion of messages in it.  A
   cache stores cacheable responses in order to reduce the response time
   and network bandwidth consumption on future, equivalent requests.
   Any client or server MAY employ a cache, though a cache cannot be
   used by a server that is acting as a tunnel.

   A shared cache is a cache that stores responses to be reused by more
   than one user; shared caches are usually (but not always) deployed as
   a part of an intermediary.  A private cache, in contrast, is
   dedicated to a single user; often, they are deployed as a component
   of a user agent.

   The goal of caching in HTTP/1.1 is to significantly improve
   performance by reusing a prior response message to satisfy a current
   request.  A stored response is considered "fresh", as defined in
   Section 4.2, if the response can be reused without "validation"
   (checking with the origin server to see if the cached response
   remains valid for this request).  A fresh response can therefore
   reduce both latency and network overhead each time it is reused.
   When a cached response is not fresh, it might still be reusable if it
   can be freshened by validation (Section 4.3) or if the origin is
   unavailable (Section 4.2.4).

1.1.  Conformance and Error Handling

   The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
   "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this
   document are to be interpreted as described in [RFC2119].

   Conformance criteria and considerations regarding error handling are
   defined in Section 2.5 of [RFC7230].

1.2.  Syntax Notation

   This specification uses the Augmented Backus-Naur Form (ABNF)
   notation of [RFC5234] with a list extension, defined in Section 7 of
   [RFC7230], that allows for compact definition of comma-separated
   lists using a '#' operator (similar to how the '*' operator indicates
   repetition).  Appendix B describes rules imported from other
   documents.  Appendix C shows the collected grammar with all list
   operators expanded to standard ABNF notation.

1.2.1.  Delta Seconds

   The delta-seconds rule specifies a non-negative integer, representing
   time in seconds.

     delta-seconds  = 1*DIGIT

   A recipient parsing a delta-seconds value and converting it to binary
   form ought to use an arithmetic type of at least 31 bits of
   non-negative integer range.  If a cache receives a delta-seconds
   value greater than the greatest integer it can represent, or if any
   of its subsequent calculations overflows, the cache MUST consider the
   value to be either 2147483648 (2^31) or the greatest positive integer
   it can conveniently represent.

      Note: The value 2147483648 is here for historical reasons,
      effectively represents infinity (over 68 years), and does not need
      to be stored in binary form; an implementation could produce it as
      a canned string if any overflow occurs, even if the calculations
      are performed with an arithmetic type incapable of directly
      representing that number.  What matters here is that an overflow
      be detected and not treated as a negative value in later
      calculations.

2.  Overview of Cache Operation

   Proper cache operation preserves the semantics of HTTP transfers
   ([RFC7231]) while eliminating the transfer of information already
   held in the cache.  Although caching is an entirely OPTIONAL feature
   of HTTP, it can be assumed that reusing a cached response is
   desirable and that such reuse is the default behavior when no
   requirement or local configuration prevents it.  Therefore, HTTP
   cache requirements are focused on preventing a cache from either
   storing a non-reusable response or reusing a stored response
   inappropriately, rather than mandating that caches always store and
   reuse particular responses.

   Each cache entry consists of a cache key and one or more HTTP
   responses corresponding to prior requests that used the same key.
   The most common form of cache entry is a successful result of a
   retrieval request: i.e., a 200 (OK) response to a GET request, which
   contains a representation of the resource identified by the request
   target (Section 4.3.1 of [RFC7231]).  However, it is also possible to
   cache permanent redirects, negative results (e.g., 404 (Not Found)),



Fielding, et al.             Standards Track                    [Page 5]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   incomplete results (e.g., 206 (Partial Content)), and responses to
   methods other than GET if the method's definition allows such caching
   and defines something suitable for use as a cache key.

   The primary cache key consists of the request method and target URI.
   However, since HTTP caches in common use today are typically limited
   to caching responses to GET, many caches simply decline other methods
   and use only the URI as the primary cache key.

   If a request target is subject to content negotiation, its cache
   entry might consist of multiple stored responses, each differentiated
   by a secondary key for the values of the original request's selecting
   header fields (Section 4.1).

3.  Storing Responses in Caches

   A cache MUST NOT store a response to any request, unless:

   o  The request method is understood by the cache and defined as being
      cacheable, and

   o  the response status code is understood by the cache, and

   o  the "no-store" cache directive (see Section 5.2) does not appear
      in request or response header fields, and

   o  the "private" response directive (see Section 5.2.2.6) does not
      appear in the response, if the cache is shared, and

   o  the Authorization header field (see Section 4.2 of [RFC7235]) does
      not appear in the request, if the cache is shared, unless the
      response explicitly allows it (see Section 3.2), and

   o  the response either:

      *  contains an Expires header field (see Section 5.3), or

      *  contains a max-age response directive (see Section 5.2.2.8), or

      *  contains a s-maxage response directive (see Section 5.2.2.9)
         and the cache is shared, or

      *  contains a Cache Control Extension (see Section 5.2.3) that
         allows it to be cached, or

      *  has a status code that is defined as cacheable by default (see
         Section 4.2.2), or




Fielding, et al.             Standards Track                    [Page 6]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


      *  contains a public response directive (see Section 5.2.2.5).

   Note that any of the requirements listed above can be overridden by a
   cache-control extension; see Section 5.2.3.

   In this context, a cache has "understood" a request method or a
   response status code if it recognizes it and implements all specified
   caching-related behavior.

   Note that, in normal operation, some caches will not store a response
   that has neither a cache validator nor an explicit expiration time,
   as such responses are not usually useful to store.  However, caches
   are not prohibited from storing such responses.

3.1.  Storing Incomplete Responses

   A response message is considered complete when all of the octets
   indicated by the message framing ([RFC7230]) are received prior to
   the connection being closed.  If the request method is GET, the
   response status code is 200 (OK), and the entire response header
   section has been received, a cache MAY store an incomplete response
   message body if the cache entry is recorded as incomplete.  Likewise,
   a 206 (Partial Content) response MAY be stored as if it were an
   incomplete 200 (OK) cache entry.  However, a cache MUST NOT store
   incomplete or partial-content responses if it does not support the
   Range and Content-Range header fields or if it does not understand
   the range units used in those fields.

   A cache MAY complete a stored incomplete response by making a
   subsequent range request ([RFC7233]) and combining the successful
   response with the stored entry, as defined in Section 3.3.  A cache
   MUST NOT use an incomplete response to answer requests unless the
   response has been made complete or the request is partial and
   specifies a range that is wholly within the incomplete response.  A
   cache MUST NOT send a partial response to a client without explicitly
   marking it as such using the 206 (Partial Content) status code.

3.2.  Storing Responses to Authenticated Requests

   A shared cache MUST NOT use a cached response to a request with an
   Authorization header field (Section 4.2 of [RFC7235]) to satisfy any
   subsequent request unless a cache directive that allows such
   responses to be stored is present in the response.

   In this specification, the following Cache-Control response
   directives (Section 5.2.2) have such an effect: must-revalidate,
   public, and s-maxage.




Fielding, et al.             Standards Track                    [Page 7]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   Note that cached responses that contain the "must-revalidate" and/or
   "s-maxage" response directives are not allowed to be served stale
   (Section 4.2.4) by shared caches.  In particular, a response with
   either "max-age=0, must-revalidate" or "s-maxage=0" cannot be used to
   satisfy a subsequent request without revalidating it on the origin
   server.

3.3.  Combining Partial Content

   A response might transfer only a partial representation if the
   connection closed prematurely or if the request used one or more
   Range specifiers ([RFC7233]).  After several such transfers, a cache
   might have received several ranges of the same representation.  A
   cache MAY combine these ranges into a single stored response, and
   reuse that response to satisfy later requests, if they all share the
   same strong validator and the cache complies with the client
   requirements in Section 4.3 of [RFC7233].

   When combining the new response with one or more stored responses, a
   cache MUST:

   o  delete any Warning header fields in the stored response with
      warn-code 1xx (see Section 5.5);

   o  retain any Warning header fields in the stored response with
      warn-code 2xx; and,

   o  use other header fields provided in the new response, aside from
      Content-Range, to replace all instances of the corresponding
      header fields in the stored response.

4.  Constructing Responses from Caches

   When presented with a request, a cache MUST NOT reuse a stored
   response, unless:

   o  The presented effective request URI (Section 5.5 of [RFC7230]) and
      that of the stored response match, and

   o  the request method associated with the stored response allows it
      to be used for the presented request, and

   o  selecting header fields nominated by the stored response (if any)
      match those presented (see Section 4.1), and







Fielding, et al.             Standards Track                    [Page 8]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   o  the presented request does not contain the no-cache pragma
      (Section 5.4), nor the no-cache cache directive (Section 5.2.1),
      unless the stored response is successfully validated
      (Section 4.3), and

   o  the stored response does not contain the no-cache cache directive
      (Section 5.2.2.2), unless it is successfully validated
      (Section 4.3), and

   o  the stored response is either:

      *  fresh (see Section 4.2), or

      *  allowed to be served stale (see Section 4.2.4), or

      *  successfully validated (see Section 4.3).

   Note that any of the requirements listed above can be overridden by a
   cache-control extension; see Section 5.2.3.

   When a stored response is used to satisfy a request without
   validation, a cache MUST generate an Age header field (Section 5.1),
   replacing any present in the response with a value equal to the
   stored response's current_age; see Section 4.2.3.

   A cache MUST write through requests with methods that are unsafe
   (Section 4.2.1 of [RFC7231]) to the origin server; i.e., a cache is
   not allowed to generate a reply to such a request before having
   forwarded the request and having received a corresponding response.

   Also, note that unsafe requests might invalidate already-stored
   responses; see Section 4.4.

   When more than one suitable response is stored, a cache MUST use the
   most recent response (as determined by the Date header field).  It
   can also forward the request with "Cache-Control: max-age=0" or
   "Cache-Control: no-cache" to disambiguate which response to use.

   A cache that does not have a clock available MUST NOT use stored
   responses without revalidating them upon every use.

4.1.  Calculating Secondary Keys with Vary

   When a cache receives a request that can be satisfied by a stored
   response that has a Vary header field (Section 7.1.4 of [RFC7231]),
   it MUST NOT use that response unless all of the selecting header





Fielding, et al.             Standards Track                    [Page 9]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   fields nominated by the Vary header field match in both the original
   request (i.e., that associated with the stored response), and the
   presented request.

   The selecting header fields from two requests are defined to match if
   and only if those in the first request can be transformed to those in
   the second request by applying any of the following:

   o  adding or removing whitespace, where allowed in the header field's
      syntax

   o  combining multiple header fields with the same field name (see
      Section 3.2 of [RFC7230])

   o  normalizing both header field values in a way that is known to
      have identical semantics, according to the header field's
      specification (e.g., reordering field values when order is not
      significant; case-normalization, where values are defined to be
      case-insensitive)

   If (after any normalization that might take place) a header field is
   absent from a request, it can only match another request if it is
   also absent there.

   A Vary header field-value of "*" always fails to match.

   The stored response with matching selecting header fields is known as
   the selected response.

   If multiple selected responses are available (potentially including
   responses without a Vary header field), the cache will need to choose
   one to use.  When a selecting header field has a known mechanism for
   doing so (e.g., qvalues on Accept and similar request header fields),
   that mechanism MAY be used to select preferred responses; of the
   remainder, the most recent response (as determined by the Date header
   field) is used, as per Section 4.

   If no selected response is available, the cache cannot satisfy the
   presented request.  Typically, it is forwarded to the origin server
   in a (possibly conditional; see Section 4.3) request.











Fielding, et al.             Standards Track                   [Page 10]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


4.2.  Freshness

   A fresh response is one whose age has not yet exceeded its freshness
   lifetime.  Conversely, a stale response is one where it has.

   A response's freshness lifetime is the length of time between its
   generation by the origin server and its expiration time.  An explicit
   expiration time is the time at which the origin server intends that a
   stored response can no longer be used by a cache without further
   validation, whereas a heuristic expiration time is assigned by a
   cache when no explicit expiration time is available.

   A response's age is the time that has passed since it was generated
   by, or successfully validated with, the origin server.

   When a response is "fresh" in the cache, it can be used to satisfy
   subsequent requests without contacting the origin server, thereby
   improving efficiency.

   The primary mechanism for determining freshness is for an origin
   server to provide an explicit expiration time in the future, using
   either the Expires header field (Section 5.3) or the max-age response
   directive (Section 5.2.2.8).  Generally, origin servers will assign
   future explicit expiration times to responses in the belief that the
   representation is not likely to change in a semantically significant
   way before the expiration time is reached.

   If an origin server wishes to force a cache to validate every
   request, it can assign an explicit expiration time in the past to
   indicate that the response is already stale.  Compliant caches will
   normally validate a stale cached response before reusing it for
   subsequent requests (see Section 4.2.4).

   Since origin servers do not always provide explicit expiration times,
   caches are also allowed to use a heuristic to determine an expiration
   time under certain circumstances (see Section 4.2.2).

   The calculation to determine if a response is fresh is:

      response_is_fresh = (freshness_lifetime > current_age)

   freshness_lifetime is defined in Section 4.2.1; current_age is
   defined in Section 4.2.3.

   Clients can send the max-age or min-fresh cache directives in a
   request to constrain or relax freshness calculations for the
   corresponding response (Section 5.2.1).




Fielding, et al.             Standards Track                   [Page 11]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   When calculating freshness, to avoid common problems in date parsing:

   o  Although all date formats are specified to be case-sensitive, a
      cache recipient SHOULD match day, week, and time-zone names
      case-insensitively.

   o  If a cache recipient's internal implementation of time has less
      resolution than the value of an HTTP-date, the recipient MUST
      internally represent a parsed Expires date as the nearest time
      equal to or earlier than the received value.

   o  A cache recipient MUST NOT allow local time zones to influence the
      calculation or comparison of an age or expiration time.

   o  A cache recipient SHOULD consider a date with a zone abbreviation
      other than GMT or UTC to be invalid for calculating expiration.

   Note that freshness applies only to cache operation; it cannot be
   used to force a user agent to refresh its display or reload a
   resource.  See Section 6 for an explanation of the difference between
   caches and history mechanisms.

4.2.1.  Calculating Freshness Lifetime

   A cache can calculate the freshness lifetime (denoted as
   freshness_lifetime) of a response by using the first match of the
   following:

   o  If the cache is shared and the s-maxage response directive
      (Section 5.2.2.9) is present, use its value, or

   o  If the max-age response directive (Section 5.2.2.8) is present,
      use its value, or

   o  If the Expires response header field (Section 5.3) is present, use
      its value minus the value of the Date response header field, or

   o  Otherwise, no explicit expiration time is present in the response.
      A heuristic freshness lifetime might be applicable; see
      Section 4.2.2.

   Note that this calculation is not vulnerable to clock skew, since all
   of the information comes from the origin server.








Fielding, et al.             Standards Track                   [Page 12]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   When there is more than one value present for a given directive
   (e.g., two Expires header fields, multiple Cache-Control: max-age
   directives), the directive's value is considered invalid.  Caches are
   encouraged to consider responses that have invalid freshness
   information to be stale.

4.2.2.  Calculating Heuristic Freshness

   Since origin servers do not always provide explicit expiration times,
   a cache MAY assign a heuristic expiration time when an explicit time
   is not specified, employing algorithms that use other header field
   values (such as the Last-Modified time) to estimate a plausible
   expiration time.  This specification does not provide specific
   algorithms, but does impose worst-case constraints on their results.

   A cache MUST NOT use heuristics to determine freshness when an
   explicit expiration time is present in the stored response.  Because
   of the requirements in Section 3, this means that, effectively,
   heuristics can only be used on responses without explicit freshness
   whose status codes are defined as cacheable by default (see Section
   6.1 of [RFC7231]), and those responses without explicit freshness
   that have been marked as explicitly cacheable (e.g., with a "public"
   response directive).

   If the response has a Last-Modified header field (Section 2.2 of
   [RFC7232]), caches are encouraged to use a heuristic expiration value
   that is no more than some fraction of the interval since that time.
   A typical setting of this fraction might be 10%.

   When a heuristic is used to calculate freshness lifetime, a cache
   SHOULD generate a Warning header field with a 113 warn-code (see
   Section 5.5.4) in the response if its current_age is more than 24
   hours and such a warning is not already present.

      Note: Section 13.9 of [RFC2616] prohibited caches from calculating
      heuristic freshness for URIs with query components (i.e., those
      containing '?').  In practice, this has not been widely
      implemented.  Therefore, origin servers are encouraged to send
      explicit directives (e.g., Cache-Control: no-cache) if they wish
      to preclude caching.

4.2.3.  Calculating Age

   The Age header field is used to convey an estimated age of the
   response message when obtained from a cache.  The Age field value is
   the cache's estimate of the number of seconds since the response was
   generated or validated by the origin server.  In essence, the Age




Fielding, et al.             Standards Track                   [Page 13]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   value is the sum of the time that the response has been resident in
   each of the caches along the path from the origin server, plus the
   amount of time it has been in transit along network paths.

   The following data is used for the age calculation:

   age_value

      The term "age_value" denotes the value of the Age header field
      (Section 5.1), in a form appropriate for arithmetic operation; or
      0, if not available.

   date_value

      The term "date_value" denotes the value of the Date header field,
      in a form appropriate for arithmetic operations.  See Section
      7.1.1.2 of [RFC7231] for the definition of the Date header field,
      and for requirements regarding responses without it.

   now

      The term "now" means "the current value of the clock at the host
      performing the calculation".  A host ought to use NTP ([RFC5905])
      or some similar protocol to synchronize its clocks to Coordinated
      Universal Time.

   request_time

      The current value of the clock at the host at the time the request
      resulting in the stored response was made.

   response_time

      The current value of the clock at the host at the time the
      response was received.

   A response's age can be calculated in two entirely independent ways:

   1.  the "apparent_age": response_time minus date_value, if the local
       clock is reasonably well synchronized to the origin server's
       clock.  If the result is negative, the result is replaced by
       zero.

   2.  the "corrected_age_value", if all of the caches along the
       response path implement HTTP/1.1.  A cache MUST interpret this
       value relative to the time the request was initiated, not the
       time that the response was received.




Fielding, et al.             Standards Track                   [Page 14]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


     apparent_age = max(0, response_time - date_value);

     response_delay = response_time - request_time;
     corrected_age_value = age_value + response_delay;

   These are combined as

     corrected_initial_age = max(apparent_age, corrected_age_value);

   unless the cache is confident in the value of the Age header field
   (e.g., because there are no HTTP/1.0 hops in the Via header field),
   in which case the corrected_age_value MAY be used as the
   corrected_initial_age.

   The current_age of a stored response can then be calculated by adding
   the amount of time (in seconds) since the stored response was last
   validated by the origin server to the corrected_initial_age.

     resident_time = now - response_time;
     current_age = corrected_initial_age + resident_time;

4.2.4.  Serving Stale Responses

   A "stale" response is one that either has explicit expiry information
   or is allowed to have heuristic expiry calculated, but is not fresh
   according to the calculations in Section 4.2.

   A cache MUST NOT generate a stale response if it is prohibited by an
   explicit in-protocol directive (e.g., by a "no-store" or "no-cache"
   cache directive, a "must-revalidate" cache-response-directive, or an
   applicable "s-maxage" or "proxy-revalidate" cache-response-directive;
   see Section 5.2.2).

   A cache MUST NOT send stale responses unless it is disconnected
   (i.e., it cannot contact the origin server or otherwise find a
   forward path) or doing so is explicitly allowed (e.g., by the
   max-stale request directive; see Section 5.2.1).

   A cache SHOULD generate a Warning header field with the 110 warn-code
   (see Section 5.5.1) in stale responses.  Likewise, a cache SHOULD
   generate a 112 warn-code (see Section 5.5.3) in stale responses if
   the cache is disconnected.

   A cache SHOULD NOT generate a new Warning header field when
   forwarding a response that does not have an Age header field, even if
   the response is already stale.  A cache need not validate a response
   that merely became stale in transit.




Fielding, et al.             Standards Track                   [Page 15]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


4.3.  Validation

   When a cache has one or more stored responses for a requested URI,
   but cannot serve any of them (e.g., because they are not fresh, or
   one cannot be selected; see Section 4.1), it can use the conditional
   request mechanism [RFC7232] in the forwarded request to give the next
   inbound server an opportunity to select a valid stored response to
   use, updating the stored metadata in the process, or to replace the
   stored response(s) with a new response.  This process is known as
   "validating" or "revalidating" the stored response.

4.3.1.  Sending a Validation Request

   When sending a conditional request for cache validation, a cache
   sends one or more precondition header fields containing validator
   metadata from its stored response(s), which is then compared by
   recipients to determine whether a stored response is equivalent to a
   current representation of the resource.

   One such validator is the timestamp given in a Last-Modified header
   field (Section 2.2 of [RFC7232]), which can be used in an
   If-Modified-Since header field for response validation, or in an
   If-Unmodified-Since or If-Range header field for representation
   selection (i.e., the client is referring specifically to a previously
   obtained representation with that timestamp).

   Another validator is the entity-tag given in an ETag header field
   (Section 2.3 of [RFC7232]).  One or more entity-tags, indicating one
   or more stored responses, can be used in an If-None-Match header
   field for response validation, or in an If-Match or If-Range header
   field for representation selection (i.e., the client is referring
   specifically to one or more previously obtained representations with
   the listed entity-tags).

4.3.2.  Handling a Received Validation Request

   Each client in the request chain may have its own cache, so it is
   common for a cache at an intermediary to receive conditional requests
   from other (outbound) caches.  Likewise, some user agents make use of
   conditional requests to limit data transfers to recently modified
   representations or to complete the transfer of a partially retrieved
   representation.

   If a cache receives a request that can be satisfied by reusing one of
   its stored 200 (OK) or 206 (Partial Content) responses, the cache
   SHOULD evaluate any applicable conditional header field preconditions
   received in that request with respect to the corresponding validators
   contained within the selected response.  A cache MUST NOT evaluate



Fielding, et al.             Standards Track                   [Page 16]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   conditional header fields that are only applicable to an origin
   server, found in a request with semantics that cannot be satisfied
   with a cached response, or applied to a target resource for which it
   has no stored responses; such preconditions are likely intended for
   some other (inbound) server.

   The proper evaluation of conditional requests by a cache depends on
   the received precondition header fields and their precedence, as
   defined in Section 6 of [RFC7232].  The If-Match and
   If-Unmodified-Since conditional header fields are not applicable to a
   cache.

   A request containing an If-None-Match header field (Section 3.2 of
   [RFC7232]) indicates that the client wants to validate one or more of
   its own stored responses in comparison to whichever stored response
   is selected by the cache.  If the field-value is "*", or if the
   field-value is a list of entity-tags and at least one of them matches
   the entity-tag of the selected stored response, a cache recipient
   SHOULD generate a 304 (Not Modified) response (using the metadata of
   the selected stored response) instead of sending that stored
   response.

   When a cache decides to revalidate its own stored responses for a
   request that contains an If-None-Match list of entity-tags, the cache
   MAY combine the received list with a list of entity-tags from its own
   stored set of responses (fresh or stale) and send the union of the
   two lists as a replacement If-None-Match header field value in the
   forwarded request.  If a stored response contains only partial
   content, the cache MUST NOT include its entity-tag in the union
   unless the request is for a range that would be fully satisfied by
   that partial stored response.  If the response to the forwarded
   request is 304 (Not Modified) and has an ETag header field value with
   an entity-tag that is not in the client's list, the cache MUST
   generate a 200 (OK) response for the client by reusing its
   corresponding stored response, as updated by the 304 response
   metadata (Section 4.3.4).

   If an If-None-Match header field is not present, a request containing
   an If-Modified-Since header field (Section 3.3 of [RFC7232])
   indicates that the client wants to validate one or more of its own
   stored responses by modification date.  A cache recipient SHOULD
   generate a 304 (Not Modified) response (using the metadata of the
   selected stored response) if one of the following cases is true: 1)
   the selected stored response has a Last-Modified field-value that is
   earlier than or equal to the conditional timestamp; 2) no
   Last-Modified field is present in the selected stored response, but
   it has a Date field-value that is earlier than or equal to the
   conditional timestamp; or, 3) neither Last-Modified nor Date is



Fielding, et al.             Standards Track                   [Page 17]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   present in the selected stored response, but the cache recorded it as
   having been received at a time earlier than or equal to the
   conditional timestamp.

   A cache that implements partial responses to range requests, as
   defined in [RFC7233], also needs to evaluate a received If-Range
   header field (Section 3.2 of [RFC7233]) with respect to its selected
   stored response.

4.3.3.  Handling a Validation Response

   Cache handling of a response to a conditional request is dependent
   upon its status code:

   o  A 304 (Not Modified) response status code indicates that the
      stored response can be updated and reused; see Section 4.3.4.

   o  A full response (i.e., one with a payload body) indicates that
      none of the stored responses nominated in the conditional request
      is suitable.  Instead, the cache MUST use the full response to
      satisfy the request and MAY replace the stored response(s).

   o  However, if a cache receives a 5xx (Server Error) response while
      attempting to validate a response, it can either forward this
      response to the requesting client, or act as if the server failed
      to respond.  In the latter case, the cache MAY send a previously
      stored response (see Section 4.2.4).

4.3.4.  Freshening Stored Responses upon Validation

   When a cache receives a 304 (Not Modified) response and already has
   one or more stored 200 (OK) responses for the same cache key, the
   cache needs to identify which of the stored responses are updated by
   this new response and then update the stored response(s) with the new
   information provided in the 304 response.

   The stored response to update is identified by using the first match
   (if any) of the following:

   o  If the new response contains a strong validator (see Section 2.1
      of [RFC7232]), then that strong validator identifies the selected
      representation for update.  All of the stored responses with the
      same strong validator are selected.  If none of the stored
      responses contain the same strong validator, then the cache MUST
      NOT use the new response to update any stored responses.






Fielding, et al.             Standards Track                   [Page 18]

 
RFC 7234                    HTTP/1.1 Caching                   June 2014


   o  If the new response contains a weak validator and that validator
      corresponds to one of the cache's stored responses, then the most
      recent of those matching stored responses is selected for update.

   o  If the new response does not include any form of validator (such
      as in the case where a client generates an If-Modified-Since
      request from a source other than the Last-Modified response header
      field), and there is only one stored response, and that stored
      response also lacks a validator, then that stored response is
      selected for update.

   If a stored response is selected for update, the cache MUST:

   o  delete any Warning header fields in the stored response with
      warn-code 1xx (see Section 5.5);

   o  retain any Warning header fields in the stored response with
      warn-code 2xx; and,

   o  use other header fields provided in the 304 (Not Modified)
      response to replace all instances of the corresponding header
      fields in the stored response.