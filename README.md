# An exploration of type-level SAFE.

This software is in a very early stage of development. We do not advise depending on it, yet.

## EXTRA-SAFE

This applies [typestate-oriented programming techniques](https://dl.acm.org/doi/10.1145/1639950.1640073) in Rust, along with [type-level programming](https://www.cambridge.org/core/journals/journal-of-functional-programming/article/faking-it-simulating-dependent-types-in-haskell/) in order of make [SAFE APIs](https://hackmd.io/@7dpNYqjKQGeYC7wMlPxHtQ/ByIbpfX9c) even more safe.

At a basic level, the SAFE API is meant to return errors at runtime when the user makes a mistake in its usage. This crate aims to lift most of those errors at compile time.

## License

MIT