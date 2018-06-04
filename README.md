# serde_cbor-diag

An implementation of a [serde][] serializer outputting the [CBOR diagnostic
notation][].

## ⚠️  Abandoned ⚠️

This approach was found to be unsuitable for the [intended use case][]. The
current implementation is feature complete and exhaustively tested, so if this
does fit a use case of yours feel free to take over this library (a PR adding a
link to your fork here would be appreciated).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.


[serde]: https://serde.rs
[CBOR diagnostic notation]: https://tools.ietf.org/html/rfc7049#section-6
[intended use case]: https://github.com/Nemo157/coap-browse/issues/1
