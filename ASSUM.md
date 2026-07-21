# production_source_correspondence

`verus/from_random_bytes_layout.rs` models the two byte assignments used by
`Builder::from_random_bytes` through `with_variant(Variant::RFC4122)` and
`with_version(Version::Random)`. The integration test calls the real public
constructor; source review and that test are the bridge from this model to the
crate implementation.

# caller_entropy

The caller supplies `from_random_bytes` input. This work proves deterministic
layout and byte preservation only; it makes no claim about input entropy or
UUID uniqueness.
