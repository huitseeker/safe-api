# An exploration of type-level SAFE.

This software is in a very early stage of development. We do not advise depending on it, yet.

The goal of this crate is to interface with any implementation of the SAFE
API and make its errors appear at compile-time rather than at runtime.

## The SAFE API for sponge functions

 The Safe API is an interface for the use of a sponge, a frequent building
 block for hash functions. It is more specifically a variant of the duplex
 model, by default specified with field elements as the arguments-which
 makes it zkProof-firendly. The aim of this API is to help eliminate
 common painpoints and security bugs found in implementations.

- Security flaws, such as domain separation failures
- Inconsistent APIs that are hard to use securely

The SAFE API is documented at http://safe-hash.dev.
See also this talk at the ZKSummit 8:
https://www.youtube.com/watch?v=w-4fzHpd4dk

## EXTRA-SAFE

This crate applies [typestate-oriented programming techniques](https://dl.acm.org/doi/10.1145/1639950.1640073) in Rust, along with [type-level programming](https://www.cambridge.org/core/journals/journal-of-functional-programming/article/faking-it-simulating-dependent-types-in-haskell/) in order of make [SAFE APIs](https://hackmd.io/@7dpNYqjKQGeYC7wMlPxHtQ/ByIbpfX9c) even more safe.

At a basic level, the SAFE API is meant to return errors at runtime when the user makes a mistake in its usage. This crate aims to lift most of those errors at compile time.

## License

MIT
