# occrs

A CLI utility for counting occurences in a list.

---

## Usage

Pass a newline-separated list to stdin.

e.g. `cat testfile | occrs`

## Sample output

The following output is the result of processing `testfile` in the root of this repository.

```
18 (47.37%) y
20 (52.63%) n
Total items: 38
```
## Compiling

To compile `occrs`, run `cargo build --release`. You may want to strip debug symbols to reduce executable size with `strip`, e.g. `strip target/release/occrs`. 

---

Copyright 2020 Amado Wilkins. *This project is licensed under the Apache License 2.0; see `LICENSE` for details.*