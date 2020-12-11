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

```sh
cp -rs ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/ ~/.rustup/toolchains/subcrate/
```

> Depending on your platform, you may need to change the source path to
> your platform's toolchain. Note that the `-s` flag is being passed to
> `cp`, so it will create symlinks to your existing toolchain rather
> than copying the files. This is done to save hard drive space.


After this, you can use `cargo install` to download, compile, and
install the fork into your custom toolchain. This will overwrite the
`cargo` binary in your new toolchain with the fork.

```sh
cargo install --force --git https://github.com/caeg-industries/cargo.git --branch subcrates --root ~/.rustup/toolchains/subcrate/ -- cargo
```

After running this, you'll be able to run your usual version of Cargo
and the fork interchangably by adding `+subcrate` after `cargo`. For
example, here is how you would check that the installation worked
correctly by inspecting the version number.

```sh
> cargo --version
1.48.0

> cargo +subcrate --version
1.50.0-namespace-fork
```

## Windows Setup

You first need to clone a Rustup toolchain that you already have
installed.

```sh
xcopy /E /I %USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc %USERPROFILE%\.rustup\toolchains\subcrate
```

> Depending on which channel you use, you may need to change the source path to
> your platform's toolchain.

After this, you can use `cargo install` to download, compile, and
install the fork into your custom toolchain. This will overwrite the
`cargo` binary in your new toolchain with the fork.

```sh
cargo install --force --git https://github.com/caeg-industries/cargo.git --branch subcrates --root %USERPROFILE%\.rustup\toolchains\subcrate -- cargo
```

After running this, you'll be able to run your usual version of Cargo
and the fork interchangably by adding `+subcrate` after `cargo`. For
example, here is how you would check that the installation worked
correctly by inspecting the version number.

```sh
> cargo --version
1.48.0

> cargo +subcrate --version
1.50.0-namespace-fork
```

## Installing Without Rustup

See the [Cargo Contributor Guide](https://doc.crates.io/contrib/process/working-on-cargo.html)
for other methods of building and using a fork of Cargo.

# Publishing Your First Subcrate

Namespaced crates start with a regular crate to reserve the
namespace. This is just a regular crate, no changes from normal Cargo
behaviour.

```toml
# parent/Cargo.toml

[package]
name = "parent"
description = "the parent crate!"
version = "0.1.0"
authors = ["You"]
edition = "2018"
license = "MIT"
```

```rust
// parent/src/main.rs

fn main() {
    println!("Hello world!");
}
```

## Creating Subcrates

However, with this fork, you can now create a new subcrate which is in
the `parent` namespace.

When compiling any of these examples, make sure to add `+subcrate` to
any `cargo` commands to use the fork of Cargo. For example, built with
`cargo +subcrate build`.

```toml
# parent-foo/Cargo.toml

[package]
name = "parent/foo"
description = "this is a subcrate"
version = "0.1.0"
authors = ["You"]
edition = "2018"
license = "MIT"
```

```rust
// parent-foo/src/lib.rs

pub fn do_foo() {
    println!("Hello world from parent/foo!");
}
```

Let's say we want to create another subcrate that depends on
`parent/foo`. You do it the same way you have any other
dependency. Note that, to be valid TOML, you need to quote namespaced
package names.

In Rust code, the namespace separator is replaced with a `_`.

## Depending on Subcrates

```toml
# parent-bar/Cargo.toml

[package]
name = "parent/bar"
description = "this is a subcrate"
version = "0.1.0"
authors = ["You"]
edition = "2018"
license = "MIT"

[dependencies]
"parent/foo" = { path = "../parent-foo" }
```

```rust
// parent-bar/src/lib.rs

use parent_foo::do_foo;

pub fn do_bar() {
    do_foo();
    println!("Hello world from parent/bar!");
}
```

## Resolving Name Conflicts

Suppose that another crate called `parent_foo` already exists. As you
migrate from using `parent_foo` to `parent/foo`, you might want to
depend on both at the same time! To do this, you need to rename one of
them in your Cargo.toml.

```toml
# parent-baz/Cargo.toml

[package]
name = "parent/baz"
description = "this is a subcrate"
version = "0.1.0"
authors = ["You"]
edition = "2018"
license = "MIT"

[dependencies]
"parent-foo" = "0.1.0"
"new-parent-foo" = { path = "../parent-foo", package = "parent/foo" }
```

```rust
// parent-baz/src/lib.rs

use parent_foo::old_lib_do_foo;
use new_parent_foo::do_foo;

pub fn do_baz() {
    old_lib_do_foo();
    do_foo();
    println!("Hello world from parent/baz!");
}
```

## Publishing to the Crates.io Fork

We have a fork of crates.io that supports crates with namespaces. To
use it, go to <TODO: fork URL here>. Login using Github and generate
an API token as usual.

When you're ready to publish, you need to point your publish at the
'index' of the forked crates.io
(https://github.com/caeg-industries/crates.io-namespace-fork-index.git).

For example,
```shell
cargo +subcrate publish --index https://github.com/caeg-industries/crates.io-namespace-fork-index.git --token "$YOUR_TOKEN"
```

## Depending on the Crates.io Fork

Up to now, we've shown path dependencies to subcrates. To depend on a
package in the fork of crates.io that we just published to, we first
need to add the registry the .cargo/config.toml file.

```toml
# parent-bar/.cargo/config.toml

[registries]
namespace-fork = { index = "https://github.com/caeg-industries/crates.io-namespace-fork-index.git" }
```

Then, when you depend on the package in your Cargo.toml, add the
"registry" key to your dependency.

```toml
# parent-bar/Cargo.toml

[package]
name = "parent/bar"
description = "this is a subcrate"
version = "0.1.0"
authors = ["You"]
edition = "2018"
license = "MIT"

[dependencies]
"parent/foo" = { version = "0.1.0", registry = "namespace-fork" }
```

For more information, the full docs on how to use an alternative
registry is available in [Cargo's docs](https://doc.rust-lang.org/cargo/reference/registries.html#using-an-alternate-registry).

# Reporting issues

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

