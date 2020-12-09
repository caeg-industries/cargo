//! Tests for the `cargo build` command.

use cargo::{
    core::compiler::CompileMode,
    core::{Shell, Workspace},
    ops::CompileOptions,
    Config,
};
use cargo_test_support::{basic_manifest, main_file, named_bin_manifest, paths, project};
use std::env;

#[cargo_test]
fn cargo_compile_simple() {
    let p = project()
        .file("Cargo.toml", &named_bin_manifest("foo/bar", "foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.cargo("build").run();
    assert!(p.bin("foo").is_file());

    p.process(&p.bin("foo")).with_stdout("i am foo\n").run();
}

#[cargo_test]
fn cargo_fail_with_no_stderr() {
    let p = project()
        .file("Cargo.toml", &named_bin_manifest("foo/bar", "foo"))
        .file("src/foo.rs", &String::from("refusal"))
        .build();
    p.cargo("build --message-format=json")
        .with_status(101)
        .with_stderr_does_not_contain("--- stderr")
        .run();
}

/// Checks that the `CARGO_INCREMENTAL` environment variable results in
/// `rustc` getting `-C incremental` passed to it.
#[cargo_test]
fn cargo_compile_incremental() {
    let p = project()
        .file("Cargo.toml", &named_bin_manifest("foo/bar", "foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.cargo("build -v")
        .env("CARGO_INCREMENTAL", "1")
        .with_stderr_contains(
            "[RUNNING] `rustc [..] -C incremental=[..]/target/debug/incremental[..]`\n",
        )
        .run();

    p.cargo("test -v")
        .env("CARGO_INCREMENTAL", "1")
        .with_stderr_contains(
            "[RUNNING] `rustc [..] -C incremental=[..]/target/debug/incremental[..]`\n",
        )
        .run();
}

#[cargo_test]
fn cargo_compile_simple_binary_with_implicit_name() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo/bar"
                version = "0.1.0"
                authors = []

                [profile.dev]
                incremental = false

                [profile.release]
                incremental = true
            "#,
        )
        .file("src/main.rs", "fn main() { println!(\"i am foo\");}")
        .build();

    p.cargo("build").run();
    assert!(p.bin("foo_bar").is_file());

    p.process(&p.bin("foo_bar")).with_stdout("i am foo\n").run();
}

#[cargo_test]
fn incremental_profile() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo/bar"
                version = "0.1.0"
                authors = []

                [profile.dev]
                incremental = false

                [profile.release]
                incremental = true
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("build -v")
        .env_remove("CARGO_INCREMENTAL")
        .with_stderr_does_not_contain("[..]C incremental=[..]")
        .run();

    p.cargo("build -v")
        .env("CARGO_INCREMENTAL", "1")
        .with_stderr_contains("[..]C incremental=[..]")
        .run();

    p.cargo("build --release -v")
        .env_remove("CARGO_INCREMENTAL")
        .with_stderr_contains("[..]C incremental=[..]")
        .run();

    p.cargo("build --release -v")
        .env("CARGO_INCREMENTAL", "0")
        .with_stderr_does_not_contain("[..]C incremental=[..]")
        .run();
}

#[cargo_test]
fn incremental_config() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            ".cargo/config",
            r#"
                [build]
                incremental = false
            "#,
        )
        .build();

    p.cargo("build -v")
        .env_remove("CARGO_INCREMENTAL")
        .with_stderr_does_not_contain("[..]C incremental=[..]")
        .run();

    p.cargo("build -v")
        .env("CARGO_INCREMENTAL", "1")
        .with_stderr_contains("[..]C incremental=[..]")
        .run();
}

#[cargo_test]
fn cargo_compile_with_workspace_excluded() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.cargo("build --workspace --exclude foo")
        .with_stderr_does_not_contain("[..]virtual[..]")
        .with_stderr_contains("[..]no packages to compile")
        .with_status(101)
        .run();
}

#[cargo_test]
fn cargo_compile_manifest_path() {
    let p = project()
        .file("Cargo.toml", &named_bin_manifest("foo/bar", "foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.cargo("build --manifest-path foo/Cargo.toml")
        .cwd(p.root().parent().unwrap())
        .run();
    assert!(p.bin("foo").is_file());
}

#[cargo_test]
fn cargo_compile_duplicate_build_targets() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo/bar"
                version = "0.0.1"
                authors = []

                [lib]
                name = "main"
                path = "src/main.rs"
                crate-type = ["dylib"]

                [dependencies]
            "#,
        )
        .file("src/main.rs", "#![allow(warnings)] fn main() {}")
        .build();

    p.cargo("build")
        .with_stderr(
            "\
warning: file found to be present in multiple build targets: [..]main.rs
[COMPILING] foo/bar v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
}

#[cargo_test]
fn cargo_compile_with_invalid_version() {
    let p = project()
        .file("Cargo.toml", &basic_manifest("foo/bar", "1.0"))
        .build();

    p.cargo("build")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  Expected dot for key `package.version`
",
        )
        .run();
}

#[cargo_test]
fn cargo_compile_api_exposes_artifact_paths() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo/bar"
                authors = []
                version = "0.0.0"

                [[bin]]
                name = "the_foo_bin/bar"
                path = "src/bin.rs"

                [lib]
                name = "the_foo_lib/bar"
                path = "src/foo.rs"
                crate-type = ["cdylib", "rlib"]
            "#,
        )
        .file("src/foo.rs", "pub fn bar() {}")
        .file("src/bin.rs", "pub fn main() {}")
        .build();

    let shell = Shell::from_write(Box::new(Vec::new()));
    let config = Config::new(shell, env::current_dir().unwrap(), paths::home());
    let ws = Workspace::new(&p.root().join("Cargo.toml"), &config).unwrap();
    let compile_options = CompileOptions::new(ws.config(), CompileMode::Build).unwrap();

    let result = cargo::ops::compile(&ws, &compile_options).unwrap();

    assert_eq!(1, result.binaries.len());
    assert!(result.binaries[0].1.exists());
    assert!(result.binaries[0]
        .1
        .to_str()
        .unwrap()
        .contains("the_foo_bin_bar"));

    assert_eq!(1, result.cdylibs.len());
    // The exact library path varies by platform, but should certainly exist at least
    assert!(result.cdylibs[0].1.exists());
    assert!(result.cdylibs[0]
        .1
        .to_str()
        .unwrap()
        .contains("the_foo_lib_bar"));
}

#[cargo_test]
fn cargo_compile_with_invalid_code() {
    let p = project()
        .file("Cargo.toml", &named_bin_manifest("foo/bar", "foo"))
        .file("src/foo.rs", "invalid rust code!")
        .build();

    p.cargo("build")
        .with_status(101)
        .with_stderr_contains(
            "\
[ERROR] could not compile `foo/bar`

To learn more, run the command again with --verbose.\n",
        )
        .run();
    assert!(p.root().join("Cargo.lock").is_file());
}

#[cargo_test]
fn cargo_compile_with_build_script_puts_output_in_correct_folder() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo/bar"
                authors = []
                version = "0.0.0"
            "#,
        )
        .file("src/lib.rs", "pub fn bar() {}")
        .file("build.rs", "pub fn main() { println!(\"Hello world\"); }")
        .build();
    p.cargo("build").run();

    let build_script_dir = p.target_debug_dir().join("build");
    let matches: Vec<std::fs::DirEntry> = build_script_dir
        .read_dir()
        .expect("build dir did not exist")
        .map(|dir| dir.unwrap())
        .filter(|dir| dir.file_name().to_string_lossy().starts_with("foo_bar"))
        .filter(|dir| dir.path().join("output").exists())
        .collect();

    assert!(matches.len() >= 1);

    let output_contents = std::fs::read_to_string(matches[0].path().join("output"))
        .unwrap_or_else(|e| panic!("could not read file {}: {}", matches[0].path().display(), e));
    assert_eq!(output_contents, "Hello world\n");
}

#[cargo_test]
fn cargo_compile_with_subcrate_dependency() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                edition = "2018"
                authors = []

                [dependencies]
                "foo/bar" = { path = "../foo-bar" }
            "#,
        )
        .file(
            "src/main.rs",
            "fn main() {println!(\"The answer is {}\", foo_bar::answer());}",
        )
        .build();
    let _bar = project()
        .at("foo-bar")
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo/bar"
                version = "0.0.1"
                edition = "2018"
                authors = []
            "#,
        )
        .file("src/lib.rs", "pub fn answer() -> i32 { 42 }")
        .build();
    p.cargo("build").run();

    p.process(&p.bin("foo"))
        .with_stdout("The answer is 42\n")
        .run();
}
