# `from_random_bytes` layout proof

The proof models the two assignments made by the public v4 builder path. It
proves that byte 6 receives the version-4 nibble, byte 8 receives the RFC 4122
variant prefix, their retained low bits are preserved, and every other byte is
unchanged. `tests/from_random_bytes_bridge.rs` invokes the actual public API
against the same contract over a deterministic bounded corpus.

Run the staged Verus executable with the matching Rust 1.94 library path; the
task artifact record contains the exact verifier provenance and command.
