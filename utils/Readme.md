# Miltr Utils

This is a small utility package to be used in the miltr implementations.

ATM it's only justification for existing is the `ByteParsing` trait on top of
the `bytes` crate.

I needed some safe wrappers around methods on `bytes::BytesMut` to check for
out of bounds. An alternative may be the
[`try_buf`](https://github.com/wheird-lee/try_buf) crate. Or, if `bytes` itself
gains something along the lines of
[try_get_* methods](https://github.com/tokio-rs/bytes/issues/254), that would be
suitable as well.
