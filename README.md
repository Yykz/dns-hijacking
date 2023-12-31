# DNS Hijacking

An asynchronous Fake DNS server that responds with a specified IP address to DNS queries for a domain matching a given regex pattern.

## Functioning

- All DNS queries associated with domain names matching the specified regex are intercepted.
- Intercepted queries will receive a fake answer containing the target IP address with empty Authority and Additional fields.
- Fake responses have a default TTL (Time To Live) of 0 to prevent lingering in the DNS cache after the program stops.
- The DNS queries that don't match the regex and rtype are sent to a Google DNS server (8.8.8.8).
- The answer of the Google DNS server is sent back to the origin of the query.

## Limitations

- Only support rtype A/AAAA
- Works only with UDP requests.
- Maximum request size 1500 bytes.
  
## Usage

```bash
Usage: dns-hijacking [OPTIONS] [ENTRIES]...

Arguments:
  [ENTRIES]...
          List of entries that you want to redirect. They must be comma-separated, and each entry consists of a domain, rtype, and IP (which is local if you leave blank), separated by ';'.
          
          Example: "google.com;A","example.com;AAAA;::1" redirects domains that match 'google.com' with IPv4 and domains that match 'example.com' with IPv6 to local.

Options:
  -v, --verbose...
          Increase verbosity, and can be used multiple times

  -t, --ttl <TTL>
          Time To Live of fake answer
          
          [default: 0]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Disclaimer

**This project is provided solely for educational purposes. The author assumes no responsibility for any misuse or unauthorized use of the code present in this repository. The use of this technique outside of a testing environment or without explicit permission from the parties involved may be against the law.**
