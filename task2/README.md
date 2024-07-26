How to start the solution?
=========================
All commands should be executed from `task2` directory.

```
cd task2
```

Install wasm-pack

```
$ cargo install wasm-pack
```

Build locally

```
$ wasm-pack build --target web
```

You can build the example locally with:

```
npx http-server .
```

and then visiting http://localhost:8080 in a browser should run the example!


Echo server
===========
I found a good local echo server that you will need to run this example.
You can check out this example:

https://github.com/snapview/tokio-tungstenite/blob/master/examples/echo-server.rs

To run this example just clone the repository and got to its location and run:
```
cargo run --example echo-server 127.0.0.1:12345
```

Now use `ws://localhost:12345/` as connection url and verify if the provided solution works correctly.


Comments
========
I'm new to web assembly in rust and js environment in general. Fortunately it's so common and well known environment that it wasn't that hard to find good examples.
The test is missing as I did not wanted to go too deep with that due to lack of time.


Based on wasm-bindgen examples
