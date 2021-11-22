# LLVM Embedded Coverage

The start of some work to use LLVM source coverage with embedded targets with
Rust. Currently, this just uses the external llvm symbols required to not write
to a file so we can transfer coverage along some other medium. And then it just
writes it to a file anyway, just to make sure it works.

To run this simple example with a recent nightly compiler:

```
RUSTFLAGS="-Z instrument-coverage" cargo +nightly run
```

If we then take the default.profraw that is written and use the profdata binary
to parse it with the show command and `--all-functions` we can see the
following printout:

```
Counters:
  _RNvCsl3MTDMpqlDe_22llvm_embedded_coverage4main:
    Hash: 0x6311214092dfa158
    Counters: 3
    Function count: 1
Instrumentation level: Front-end
Functions shown: 1
Total functions: 1
Maximum function count: 1
Maximum internal block count: 0
```

So it roughly works.

## Roadmap

- [ ] Figure out if buffer size is constant to make it more user friendly for
targets without alloc.
- [ ] Integrate some of the embedded rust tooling to send the coverage data
down the wire.
