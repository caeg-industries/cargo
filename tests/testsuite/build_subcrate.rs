//! Tests for the `cargo build` command.

use cargo::{
    core::compiler::CompileMode,
    core::manifest::SUBCRATE_DELIMETER,
    core::{Shell, Workspace},
    ops::CompileOptions,
    Config,
};
use cargo_test_support::{main_file, named_bin_manifest, paths, project};
use std::env;

fn namespaced_name(name_parts: &[&str]) -> String {
    name_parts.join(SUBCRATE_DELIMETER)
}

#[cargo_test]
fn cargo_compile_simple() {
    let p = project()
        .file(
            "Cargo.toml",
            &named_bin_manifest(&namespaced_name(&["foo", "bar"]), "foo"),
        )
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.cargo("build").run();
    assert!(p.bin("foo").is_file());

    p.process(&p.bin("foo")).with_stdout("i am foo\n").run();
}

#[cargo_test]
fn cargo_fail_with_no_stderr() {
    let p = project()
        .file(
            "Cargo.toml",
            &named_bin_manifest(&namespaced_name(&["foo", "bar"]), "foo"),
        )
        .file("src/foo.rs", &String::from("refusal"))
        .build();
    p.cargo("build --message-format=json")
        .with_status(101)
        .with_stderr_does_not_contain("--- stderr")
        .run();
}

#[cargo_test]
fn cargo_compile_simple_binary_with_implicit_name() {
    let p = project()
        .file(
            "Cargo.toml",
            &format!(
                r#"
                [package]
                name = "{}"
                version = "0.1.0"
                authors = []

                [profile.dev]
                incremental = false

                [profile.release]
                incremental = true
            "#,
                namespaced_name(&["foo", "bar"])
            ),
        )
        .file("src/main.rs", "fn main() { println!(\"i am foo\");}")
        .build();

    p.cargo("build").run();
    assert!(p.bin("foo_bar").is_file());

    p.process(&p.bin("foo_bar")).with_stdout("i am foo\n").run();
}

#[cargo_test]
fn cargo_compile_manifest_path() {
    let p = project()
        .file(
            "Cargo.toml",
            &named_bin_manifest(&namespaced_name(&["foo", "bar"]), "foo"),
        )
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
            &format!(
                r#"
                [package]
                name = "{}"
                version = "0.0.1"
                authors = []

                [lib]
                name = "main"
                path = "src/main.rs"
                crate-type = ["dylib"]

                [dependencies]
            "#,
                namespaced_name(&["foo", "bar"])
            ),
        )
        .file("src/main.rs", "#![allow(warnings)] fn main() {}")
        .build();

    p.cargo("build")
        .with_stderr(&format!(
            "\
warning: file found to be present in multiple build targets: [..]main.rs
[COMPILING] {} v0.0.1 ([..])
[FINISHED] [..]
",
            namespaced_name(&["foo", "bar"])
        ))
        .run();
}

#[cargo_test]
fn cargo_compile_api_exposes_artifact_paths() {
    let p = project()
        .file(
            "Cargo.toml",
            &format!(
                r#"
                [package]
                name = "{}"
                authors = []
                version = "0.0.0"

                [[bin]]
                name = "{}"
                path = "src/bin.rs"

                [lib]
                name = "{}"
                path = "src/foo.rs"
                crate-type = ["cdylib", "rlib"]
            "#,
                namespaced_name(&["foo", "bar"]),
                namespaced_name(&["the_foo_bin", "bar"]),
                namespaced_name(&["the_foo_lib", "bar"])
            ),
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
        .file(
            "Cargo.toml",
            &named_bin_manifest(&namespaced_name(&["foo", "bar"]), "foo"),
        )
        .file("src/foo.rs", "invalid rust code!")
        .build();

    p.cargo("build")
        .with_status(101)
        .with_stderr_contains(&format!(
            "\
[ERROR] could not compile `{}`

To learn more, run the command again with --verbose.\n",
            namespaced_name(&["foo", "bar"])
        ))
        .run();
    assert!(p.root().join("Cargo.lock").is_file());
}

#[cargo_test]
fn cargo_compile_with_build_script_puts_output_in_correct_folder() {
    let p = project()
        .file(
            "Cargo.toml",
            &format!(
                r#"
                [package]
                name = "{}"
                authors = []
                version = "0.0.0"
            "#,
                namespaced_name(&["foo", "bar"])
            ),
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
            &format!(
                r#"
                [package]
                name = "foo"
                version = "0.0.1"
                edition = "2018"
                authors = []

                [dependencies]
                "{}" = {{ path = "../foo-bar" }}
            "#,
                namespaced_name(&["foo", "bar"])
            ),
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
            &format!(
                r#"
                [package]
                name = "{}"
                version = "0.0.1"
                edition = "2018"
                authors = []
            "#,
                namespaced_name(&["foo", "bar"])
            ),
        )
        .file("src/lib.rs", "pub fn answer() -> i32 { 42 }")
        .build();
    p.cargo("build").run();

    p.process(&p.bin("foo"))
        .with_stdout("The answer is 42\n")
        .run();
}
