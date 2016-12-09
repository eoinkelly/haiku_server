# Building
I had to set the following on macOS to get the openssl crate to build:

```sh
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include;
export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include;
```
