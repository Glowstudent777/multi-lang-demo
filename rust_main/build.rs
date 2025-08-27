use std::env;
use std::process::Command;
use std::path::PathBuf;

fn try_run(cmd: &str, args: &[&str], cwd: Option<&PathBuf>) {
    let mut command = Command::new(cmd);
    command.args(args);
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    match command.status() {
        Ok(status) => {
            println!("{} exited with {}", cmd, status);
        }
        Err(e) => {
            println!("Warning: failed to run {}: {:?}", cmd, e);
        }
    }
}

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("Building C library...");
    let c_src = root.join("../c_add/add.c");
    let c_out = out_dir.join("libc_add.dll");
    try_run(
        "gcc",
        &["-shared", "-o", c_out.to_str().unwrap(), c_src.to_str().unwrap()],
        None,
    );

    println!("Building C# library...");
    let cs_proj_dir = root.join("../cs_sub");
    try_run(
        "dotnet",
        &[
            "publish",
            "-c",
            "Release",
            "-r",
            "win-x64",
            "-p:PublishAot=true",
            "--self-contained",
            "false",
        ],
        Some(&cs_proj_dir),
    );

    let cs_dll_src = cs_proj_dir
        .join("bin/Release/net8.0/win-x64/native/Sub.dll");
    let cs_dll_dst = out_dir.join("Sub.dll");
    if cs_dll_src.exists() {
        std::fs::copy(&cs_dll_src, &cs_dll_dst).expect("Failed to copy C# DLL");
    }

    println!("Building Java...");
    let java_src_dir = root.join("../java_mul");
    let java_src = java_src_dir.join("Main.java");

    let java_bin_dir = java_src_dir.join("bin");
    std::fs::create_dir_all(&java_bin_dir).unwrap();

    let status = Command::new("javac")
        .args(&["-d", java_bin_dir.to_str().unwrap(), java_src.to_str().unwrap()])
        .status()
        .expect("Failed to run javac");
    if !status.success() {
        panic!("javac failed");
    }

    let jar_out = out_dir.join("java_mul.jar");

    let status = Command::new("jar")
        .args(&[
            "cf",
            jar_out.to_str().unwrap(),
            "-C",
            java_bin_dir.to_str().unwrap(),
            ".",
        ])
        .status()
        .expect("Failed to run jar");
    if !status.success() {
        panic!("jar command failed");
    }

    println!("cargo:rerun-if-changed=../c_add/add.c");
    println!("cargo:rerun-if-changed=../cs_sub/Sub.cs");
    println!("cargo:rerun-if-changed=../java_mul/Main.java");
}
