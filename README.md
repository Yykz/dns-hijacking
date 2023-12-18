# DNS Hijacking

An asynchronous Fake DNS server that responds with a specified IP address to DNS queries for a domain matching a given regex pattern for IPv4 (rtype A).

## Functioning

- All DNS queries associated with domain names matching the specified regex are intercepted.
- Intercepted queries will receive a fake answer containing the target IP address with empty Authority and Additional fields.
- Fake responses have a default TTL (Time To Live) of 0 to prevent lingering in the DNS cache after the program stops.
- The DNS queries that don't match the regex or don't concern IPv4 are sent to a Google DNS server (8.8.8.8).
- The answer of the Google DNS server is sent back to the origin of the first query.

## Limitations

- Only answering with target IP for IPv4 DNS request (rtype A).
- Works only with UDP requests.
- Maximum request size 1500 bytes.
  
## Usage

```bash
dns-hijacking [OPTIONS] <REGEX> <IP>

Arguments:
    <REGEX>: Regex that matches which domains are redirected.
    <IP>: IP where the targeted domains are redirected.

Options:
    -v, --verbose...: Increase verbosity, and can be used multiple times.
    -h, --help: Print help.
    -V, --version: Print version.
```

## Disclaimer

**This project is provided solely for educational purposes. The author assumes no responsibility for any misuse or unauthorized use of the code present in this repository. The use of this technique outside of a testing environment or without explicit permission from the parties involved may be against the law.**
