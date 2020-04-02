Http The Definitive Guide

# Proxies Versus Gateways

* proxies connect two or more applications that speak the same protocol
* gateways hook up two or more parties that speak different protocols

# URL

## URLs structure

<scheme>://<user>:<password>@<host>:<port>/<path>;<params>?<query>#<frag>

* params: a list of name/value pairs, separated from the rest of the URL (and from each other) by “;”
* frag: The frag field is not passed to the server when referencing the object; it is used internally by the client.

## Encoding Mechanisms

The encoding simply represents the unsafe character by % followed by two hexadecimal digits

* ~       126 (0x7E)  http://www.joes-hardware.com/%7Ejoe
* SPACE   32 (0x20)   http://www.joes-hardware.com/more%20tools.html
* %       37 (0x25)   http://www.joes-hardware.com/100%25satisfaction.html

It is best for client applications to convert any unsafe or restricted characters before sending any URL to any other application.

# message

* The start line and headers are just ASCII text, broken up by lines. CRLF
* The body is simply an optional chunk of data. The body can contain text or binary data or can be empty.

## request message

```js
<method> <request-URL> <version>
<headers>
<entity-body>
```

## response message

```js
<version> <status> <reason-phrase>
<headers>
<entity-body>
```

# cache

document expiration(过期) and server revalidation(重新验证)

## Expiration Dates and Ages

### Cache-Control: max-age=484200

the maximum legal elapsed time (in seconds) from when a document is first generated

## Server Revalidation

A cached document has expired doesn’t mean it is actually different from server; it just means that it’s time to check. called “server revalidation”
* If content has changed, the cache gets a new copy
* the content has not changed, the cache only gets new headers, including a new expiration date, and updates the headers in the cache

## Revalidation with Conditional Methods

If-Modified-Since: <date>
* if the document has been modified since the specified date used in conjunction with the Last-Modified header
* If the document was not modified, 304 Not Modified response 

If-None-Match: "v2.4","v2.5","v2.6"
* entity tags (ETags)
* if the cached tags differ from the tags in the server’s document.

## Controlling Cachability

### Cache-Control: no-store

forbids a cache. A cache would forward a no-store response to the client, and then delete the object

### Cache-Control: no-cache

can actually be stored in the local cache storage. It just cannot be served from the cache to the client without first revalidating the freshness with the origin server. "do-not-serve-from-cache-without-revalidation."

### Cache-Control: max-age=3600

indicates the number of seconds since it came from the server for which a document can be considered fresh.

### Cache-Control: must-revalidate

tells caches they cannot serve a stale copy of this object without first revalidating with the origin server.