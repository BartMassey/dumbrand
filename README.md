#fastrand: Not actually fast, but simple random number PRNG
Bart Massey 2022

*This is a work-in-progress.* Don't use this for anything
until it at least has its types fleshed out. Probably don't
use it ever.

This Rust library crate is a simple LCG PRNG, with a
reasonably Rustic interface that is deliberately a bit
different than that of the `rand` crate, and actually a bit
more gross. It is intended for applications that want a
simple, predictable PRNG for benchmarking or reasonably fast
generation. You will understand what is happening under the
hood with this one. You will be appalled by how the sausage
is made here.

The `rand_pcg` crate's `Mcg128Xsl64` is about 20% faster in
my bad micro benchmarks. It is also likely has much, much
better entropy; it certainly has a better interface. So use
that instead, I think.

## License

This work is available under the "MIT License". Please see
the file `LICENSE.txt` in this distribution for license
terms.
