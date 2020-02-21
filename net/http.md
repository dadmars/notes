Http The Definitive Guide

URL
    scheme://server location/path
        URLs structure

    <scheme>://<user>:<password>@<host>:<port>/<path>;<params>?<query>#<frag>
        params: a list of name/value pairs, separated from the rest of the URL (and from each other) by “;”
        frag: The frag field is not passed to the server when referencing the object; it is used internally by the client.

    Encoding Mechanisms
        The encoding simply represents the unsafe character by % followed by two hexadecimal digits

        ~       126 (0x7E)  http://www.joes-hardware.com/%7Ejoe
        SPACE   32 (0x20)   http://www.joes-hardware.com/more%20tools.html
        %       37 (0x25)   http://www.joes-hardware.com/100%25satisfaction.html

    It is best for client applications to convert any unsafe or restricted characters before sending any URL to any other application.

The start line and headers are just ASCII text, broken up by lines. CRLF
The body is simply an optional chunk of data. The body can contain text or binary data or can be empty.

request message:
    <method> <request-URL> <version>
    <headers>
    <entity-body>

response message
    <version> <status> <reason-phrase>
    <headers>
    <entity-body>

HTTP clients should not pipeline requests that have side effects (such as POSTs).
