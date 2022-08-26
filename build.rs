use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut stdout = File::create("log_stdout.txt").unwrap();
    let mut stderr = File::create("log_stderr.txt").unwrap();

    if !Path::new("c_wrapper/Makefile").exists() {
        execute_cmd("cd c_wrapper && cmake .", &mut stdout, &mut stderr);
    }
    execute_cmd("cd c_wrapper && make", &mut stdout, &mut stderr);

    execute_cmd(
        "cp c_wrapper/ogdf/libOGDF.a c_wrapper",
        &mut stdout,
        &mut stderr,
    );
    execute_cmd(
        "cp c_wrapper/ogdf/libCOIN.a c_wrapper",
        &mut stdout,
        &mut stderr,
    );

    println!("cargo:rustc-link-search=c_wrapper");
    println!("cargo:rustc-link-lib=static=c_wrapper");
    println!("cargo:rustc-link-lib=static=OGDF");
    println!("cargo:rustc-link-lib=static=COIN");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

fn execute_cmd(cmd: &str, stdout: &mut File, stderr: &mut File) {
    stdout
        .write_all(format!("executing: '{}'\n", cmd.clone()).as_bytes())
        .unwrap();
    stderr
        .write_all(format!("executing: '{}'\n", cmd.clone()).as_bytes())
        .unwrap();

    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    stdout.write_all(&output.stdout).unwrap();
    stderr.write_all(&output.stderr).unwrap();
}
