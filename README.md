# NVIDIA Monitor

Simple system monitor reporting CPU, memory, and GPU metrics of an NVIDIA card.


## Installation

- You need [rust](https://www.rust-lang.org/) installed.
- You need the library `libnvidia-ml.so` (NVIDIA Management Library) which typically comes with the NVIDIA driver.
- Then install the program with `cargo install --path .`

Tip: use [watchexec](https://github.com/watchexec/watchexec) to recompile automatically.


## License

MIT
