use std::{
    env,
    fs::{create_dir_all, remove_dir, remove_file}, path::{PathBuf, Path},
};
use duct::cmd;
use glob::glob;
use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    let task = env::args().nth(1);
    match task.as_ref().map(|it| it.as_str()) {
        Some("coverage") => coverage()?,
        _ => print_help(),
    }
    Ok(())
}

fn coverage() -> AnyResult<()> {
    let build_dir = project_root().join("target").join("debug").join("deps");
    let coverage_dir = project_root().join("coverage");
    let output = coverage_dir.join("tests.lcov");

    let _ = remove_dir(&coverage_dir); // Ignoring the error
    create_dir_all(&coverage_dir)?;

    println!("=== running coverage ===");
    cmd!("cargo", "test")
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTFLAGS", "-Cinstrument-coverage")
        .env("LLVM_PROFILE_FILE", "cargo-test-%p-%m.profraw")
        .run()?;
    println!("ok.");

    cmd!(
        "grcov",
        ".",
        "--binary-path",
        build_dir,
        "-s",
        ".",
        "-t",
        "lcov",
        "--branch",
        "--ignore-not-existing",
        "--ignore",
        "../*",
        "--ignore",
        "/*",
        "--ignore",
        "xtask/*",
        "--ignore",
        "*/src/tests/*",
        "-o",
        output,
    )
    .run()?;
    println!("ok.");

    println!("=== cleaning up ===");
    clean_files("**/*.profraw")?;
    println!("ok.");
    Ok(())
}

fn clean_files(pattern: &str) -> AnyResult<()> {
    let files: Result<Vec<PathBuf>, _> = glob(pattern)?.collect();
    files?.iter().try_for_each(remove_file)?;
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn print_help() {
    eprintln!("Tasks: coverage");    
}
