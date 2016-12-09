# Silly Haiku server

A silly little HTTP server I wrote to learn some rust. It will serve you a
(very pseudo) pseudorandom Haiku.

# Building
I had to set the following on macOS to get the openssl crate to build:

```sh
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include;
export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include;
```
