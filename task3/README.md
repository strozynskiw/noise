About the solution
==================
It's a very simple service served by Rocked. Incoming connections are blocked via Notify behind select! macro, that can react on any incoming signals (futures) wrapped with it. In our case we await incoming
notification or timeout. IT's very common way to do such a timeout.

In case incoming connection uses new unique key it is stored in the Notify Hashmap wrapped with RwLock (although Mutex could be good enough in this scenario) wrapped with Arc co we can share the whole service between tasks (that can be executed in different threads by default).

When matching connection is received, a matching Notify is send so the other task is unblocked and response to the other connection is executed the earliest possible, matching Notify is removed from hashmap
and response for current connection is send. That concludes synchronization.


How to start the solution?
=========================
Execute
```
cargo run --package task3
```

How to make a connection?
=========================
Execute
```
curl -X POST http://127.0.0.1:8000/wait-for-second-party/test-id
```

And then do the same operation from another console. In this case `test-id` is our synchronization identifier.


Comments
========
In this example I simply return static string as an error. Should be good enough for that example.
I only added unit tests. Integration tests would be great as well. Rocket provides nice guide on how to make them:
https://rocket.rs/guide/v0.5/testing/