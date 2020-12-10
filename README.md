# Cargo Crates-as-namespaces experiment

Cargo downloads your Rust project’s dependencies and compiles your project.

Learn more at https://doc.rust-lang.org/cargo/

This repo is a fork of Cargo for the purpose of experimenting with namespaces
as described in [this RFC](https://github.com/Manishearth/namespacing-rfc/blob/main/0000-packages-as-optional-namespaces.md).

The upstream repo for Cargo can be found [here](https://github.com/rust-lang/cargo).

# Namespace Support (AKA Subcrates)

While Rust crates are practically unlimited in size, it is a common
pattern for organizations to split their projects into many crates,
especially if they expect users to only need a fraction of their
crates.

This fork allows creating "namespaced" crate, which contain a `/` in
the name. For example, the crate `foo/bar` is in the `foo` namespace.

There is a corresponding [fork of
crates.io](https://github.com/caeg-industries/crates.io) which allows
publishing of namespaced crates.

If you own a crate `foo`, you may create a crate namespaced under it
as `foo/bar`. Only people who are owners of `foo` may _create_ a crate
`foo/bar` (and all owners of `foo` are implicitly owners of
`foo/bar`). After such a crate is created, additional per-crate
publishers may be added who will be able to publish subsequent
versions as usual.

# Installing and Using Cargo With Namespaces

The recommended way to use this experimental fork is to build it from
source and install it as a custom Rustup toolchain.

These steps require the following tools:

* `git`
* `curl` (on Unix)
* `pkg-config` (on Unix, used to figure out the `libssl` headers/libraries)
* OpenSSL headers (only for Unix, this is the `libssl-dev` package on ubuntu)
* `cargo` and `rustc` installed via `rustup`

## MacOS and Linux Setup

You first need to clone a Rustup toolchain that you already have
installed.

Depending on your platform, you may need to change the source path to
your platform's toolchain. Note that the `-s` flag is being passed to
`cp`, so it will create symlinks to your existing toolchain rather
than copying the files. This is done to save hard drive space.

```
cp -rs ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/ ~/.rustup/toolchains/subcrate/
```

After this, you can use `cargo install` to download, compile, and
install the fork into your custom toolchain. This will overwrite the
`cargo` binary in your new toolchain with the fork.

```
cargo install --force --git https://github.com/caeg-industries/cargo.git --branch subcrates --root ~/.rustup/toolchains/subcrate/ -- cargo
```

After running this, you'll be able to run your usual version of Cargo
and the fork interchangably by adding `+subcrate` after `cargo`. For
example, here is how you would check that the installation worked
correctly by inspecting the version number.

```
> cargo --version
1.48.0

> cargo +subcrate --version
1.50.0-namespace-fork
```

## Windows Setup

- TODO: Adapt the Linux steps for Windows

## Installing Without Rustup

See the [Cargo Contributor Guide](https://doc.crates.io/contrib/process/working-on-cargo.html)
for other methods of building and using a fork of Cargo.

# Publishing Your First Subcrate

- TODO: Create a parent crate and some subcrates with path dependencies
- TODO: Show a name conflict, and how to resolve it
- TODO: cargo publish to the fork
- TODO: Update dependencies to use crates.io fork

## Reporting issues

Found a bug? We'd love to know about it!

Please report all issues to do with namespaced crates on the GitHub
[issue tracker][issues].

[issues]: https://github.com/rust-lang/caeg-industries/issues

## Contributing

See the **[Cargo Contributor Guide]** for a complete introduction
to contributing to Cargo.

Note that this repository is only for the experimental namespace feature. For contributing to Cargo generally, you want to refer to the [upstream repo](https://github.com/rust-lang/cargo).

[Cargo Contributor Guide]: https://rust-lang.github.io/cargo/contrib/

## License

Cargo is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

### Third party software

This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (https://www.openssl.org/).

In binary form, this product includes software that is licensed under the
terms of the GNU General Public License, version 2, with a linking exception,
which can be obtained from the [upstream repository][1].

See [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY) for details.

[1]: https://github.com/libgit2/libgit2

