# An example of concurrent SQLite writes in Rust

```
$ cargo run
```

I have a SQLite database which is updated a few times per day by different
microservices - I don't want to go through the hassle of setting up a RDBMS since
I have very little data and very little write access requirements.

This repo was made to experiment with the `busy_handler` callback to serialize
concurrent transactions. The findings are that as long as a transaction from one
process doesn't last forever, the lock can eventually be acquired by simply
sleeping for a few milliseconds in the `busy_handler`. 16ms were found to yield the
fastest execution while containing the number of fn calls.