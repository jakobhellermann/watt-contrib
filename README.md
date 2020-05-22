# watt-contrib

A collection of proc-macro crates compiled to use [watt](https://github.com/dtolnay/watt).
This has the advantage, that their only dependency is the `watt` crate, saving compile time over compiling `syn`, `quote` etc.

They were generated using [cargo watt](https://github.com/jakobhellermann/cargo-watt).

## Usage

In your `Cargo.toml`:

```toml
[package]
# the usual

[dependencies]
tokio = { version = "0.2", features = ["macros"] }
thiserror = "1.0"

[patch.crates-io]
tokio-macros = { git = "https://github.com/jakobhellermann/watt-contrib" }
thiserror-impl = { git = "https://github.com/jakobhellermann/watt-contrib" }
```

Your dependency tree will look like this:

```
your-crate v0.1.0
├── thiserror v1.0.19
│   └── thiserror-impl v1.0.19
│       └── watt v0.3.0
└── tokio v0.2.21
    ├── bytes v0.5.4
    ├── pin-project-lite v0.1.5
    └── tokio-macros v0.2.5
        └── watt v0.3.0 (*)
```

If you're on 1.44+ and want to know why you depend on `syn`, run `cargo tree -e no-dev -i syn`.

<br>

**But wait, how can I know you didn't insert some malicous code into these macros?**

Well, verify it yourself.

`$ cargo watt verify ctor/src/ctor.wasm --crate ctor` will download ctor from crates.io, compile it and check that the `.wasm` file it is the same bit for bit.

Just keep in mind that reproducibility does not work 100% yet, for example compiling a file on linux will result in a different one than on macOS.
See `cargo watt`'s [section on verification](https://github.com/jakobhellermann/cargo-watt#verifying-compilation-cargo-watt-verify) for more information.

---

If any of these crates is outdated or you want to add another one:

1. Clone this repository
2. `cargo watt build --crate $crate`
3. `mv $crate-watt $crate`
4. Create a pull request
