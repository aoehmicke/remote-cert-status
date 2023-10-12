# Remaining Cert Days
## Description
This script will check the remaining days of a certificate and prints them to the shell.
The output will be ordered by remaining days & hostname.

## Usage
```bash
# Using a piped input
./remaining-cert-days.sh < <file of hostnames>
# Using an argument
./remaining-cert-days.sh <hostname>
```

```bash
# Example
./remaining-cert-days.sh < hosts.txt

# Output
HOST                 ORGANISATION                    DAYS 
some.service.tld     "Let's Encrypt"                 73   
another.service.tld  "Starfield Technologies, Inc."  359 

# Example
./remaining-cert-days.sh some.service.tld

# Output
HOST              ORGANISATION     DAYS
some.service.tld  "Let's Encrypt"  73
```

## Requirements
### For running the application
- openssl

### For building the application
- rustc
- cargo

## Building
```bash
cargo build --release

# The binary will be located at ./target/release/remaining-cert-days

# Example
echo "some.service.tld" | ./target/release/remaining-cert-days

# Output
HOST                 ORGANISATION                    DAYS
some.service.tld     "Let's Encrypt"                 73
```

## License
[MIT](https://choosealicense.com/licenses/mit/)

## Author
[André Oehmicke](https://ao.oe-so.de)

## Version
1.0.0

## Changelog
### 1.0.0
- Initial release

## TODO
- [ ] Add support for multiple hostnames as arguments
- [ ] Add support for multiple hostnames as input
- [x] Add support for a file with hostnames as input
- [ ] Add option to sort by days
- [ ] Add option to sort by hostname
- [ ] Add option to sort by organisation
