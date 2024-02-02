# Simple Rust Compiler

Simple parser/lexer/optimizer writtin in standard rust.

Supports 4 instructions:
```
MEASURE
CZ
RZ
RX
```

## Build and Run

This compiler is built using cargo and the rust std library, so build and execution is simple.

### Build:
```
cargo build --release
```
_executable located at target/release/dustinw-qc_

Supply the inq file to be compiled as an argument:
```
dustinw-qc examples/testdata/sample_1.inq
```

### Tests:
```
cargo test
```
