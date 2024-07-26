How to start the solution
=========================

To run test use
```
cargo test --package task1
```

Tu run the example binary execute:
```
cargo run --package task1
```


Comments
========
A couple of notes and tradeoffs:
- Documentation lacks of examples but I think good unit tests are usually enough for an example.
- I don't like comments for each line in the code as long as the code is understandable. Rust magic always should be documented, though.
- I used BigInt initially but then realized that Scalar is the way to go. Struggled a little bit as it's API is slightly... complex but I fund some good use cases in the RustCrypto test cases.