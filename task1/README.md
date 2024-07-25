To run test use
```
cargo test --package task1
```

Tu ran the example binary execute:
```
cargo run --package task1
```

A couple of notes and tradeoffs:
- Used just a String as error types while the best way to introduce a error to library like that is to provide custom enum implementing Error trait with text decryption for each error. Potentially also wrapping errors from higher layers using #[from].
- Documentation lacks of examples but I think good unit tests are usually enough for an example.
- I don't like comments for each line in the code as long as the code is understandable. Rust magic always should be documented, though.
- I used BigInt initially but then realized that Scalar is the way to go. Struggled a little bit as it's API is slightly... complex but I fund some good use cases in the RustCrypto test cases.