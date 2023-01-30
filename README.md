# papillon
Papillon is command line HTTP stress tester.

Basicly WIP 1-to-1 rewrite of [Pewpew](https://github.com/bengadbois/pewpew).

```
Usage: papillon [OPTIONS] [BODY] [BODY_FILE] [COOKIES] [BASIC_AUTH] [OUTPUT_JSON] [OUTPUT_CSV] [OUTPUT_XML] [CPU] <COMMAND>

Commands:
  benchmark  Run benchmark tests
  stress     
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [BODY]         String to use as request body e.g. POST body.
  [BODY_FILE]    Path to file to use as request body. Will overwrite --body if both are present.
  [COOKIES]      Add request cookies, eg. 'data=123; session=456'
  [BASIC_AUTH]   Add HTTP basic authentication, eg. 'user123:password456'.
  [OUTPUT_JSON]  Path to file to write full data as JSON
  [OUTPUT_CSV]   Path to file to write full data as CSV
  [OUTPUT_XML]   Path to file to write full data as XML
  [CPU]          Number of CPUs to use. [default: 1]

Options:
  -r                       Interpret URLs as regular expressions.
      --dns-prefetch       Prefetch IP from hostname before making request, eliminating DNS fetching from timing.
  -t <TIMEOUT>             Maximum seconds to wait for response [default: 10]
  -X <REQUEST_METHOD>      Request type. GET, HEAD, POST, PUT, etc. [default: GET]
      --body-regex         Interpret Body as regular expressions.
  -H <HEADERS>             Add arbitrary header line, eg. 'Accept-Encoding:gzip, Content-Type:application/json'
  -A <USER_AGENT>          Add User-Agent header. Can also be done with the arbitrary header flag. [default: pillon]
  -C                       Add 'Accept-Encoding: gzip' header if Accept-Encoding is not already present.
  -k                       Enable HTTP KeepAlive.
      --follow-redirects   Follow HTTP redirects.
      --no-http2           Disable HTTP2.
      --enforce-ssl        Enfore SSL certificate correctness.
  -q                       Do not print while requests are running.
  -v                       Print extra troubleshooting info.
  -h, --help               Print help
  ```

# Notice

There's currently no actual functionality.
