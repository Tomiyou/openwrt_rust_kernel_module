extern crate bindgen;

use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const INCLUDED_TYPES: &[&str] = &[
    "spinlock_t",
    "mutex",
    "sk_buff",
    "net_device",
];
const INCLUDED_FUNCTIONS: &[&str] = &[
    "spin_lock",
    "_?printk",
    "krealloc",
    "kfree",
    "mutex_unlock",
    "rust_helper_.*",
];
const INCLUDED_VARS: &[&str] = &[
    "__this_module",
    "LINUX_VERSION_CODE",
    "THIS_MODULE"
];

const UNSUPPORTED_ARGS: &[&str] = &[
    // "-ftrivial-auto-var-init=zero",
    // "-Wimplicit-fallthrough=5",
    // "-Wno-maybe-uninitialized",
    // "-Wno-alloc-size-larger-than",
    // "-mno-thumb-interwork",
    // "-fno-caller-saves",
    // "-Wno-unused-but-set-variable",
    // "-fno-var-tracking-assignments",
    // "-Wno-format-truncation",
    // "-Wno-format-overflow",
    // "-Wno-dangling-pointer",
    // "-Wno-stringop-truncation",
    // "-Wno-stringop-overflow",
    // "-Wno-restrict",
    // "-Werror=designated-init",
    // "-Wno-packed-not-aligned",
];

fn handle_kernel_version_cfg(bindings_path: &PathBuf) {
    let f = BufReader::new(fs::File::open(bindings_path).unwrap());
    let mut version = None;
    for line in f.lines() {
        let line = line.unwrap();
        if let Some(type_and_value) = line.split("pub const LINUX_VERSION_CODE").nth(1) {
            if let Some(value) = type_and_value.split('=').nth(1) {
                let raw_version = value.split(';').next().unwrap();
                version = Some(raw_version.trim().parse::<u64>().unwrap());
                break;
            }
        }
    }
    let version = version.expect("Couldn't find kernel version");
    let (major, minor) = match version.to_be_bytes() {
        [0, 0, 0, 0, 0, major, minor, _patch] => (major, minor),
        _ => panic!("unable to parse LINUX_VERSION_CODE {:x}", version),
    };

    if major >= 6 {
        for x in 0..=if major > 6 { unimplemented!() } else { minor } {
            println!("cargo:rustc-cfg=kernel_6_{}_0_or_greater", x);
        }
    }
    if major >= 5 {
        for x in 0..=if major > 5 { 19 } else { minor } {
            println!("cargo:rustc-cfg=kernel_5_{}_0_or_greater", x);
        }
    }
    if major >= 4 {
        // We don't currently support anything older than 4.4
        for x in 4..=if major > 4 { 20 } else { minor } {
            println!("cargo:rustc-cfg=kernel_4_{}_0_or_greater", x);
        }
    }
    // panic!("omeglaul")
}

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("Target={}", target);

    let mut builder = bindgen::Builder::default()
        .use_core()
        .detect_include_paths(false)
        .ctypes_prefix("c_types")
        .size_t_is_usize(true)
        .no_copy(".*")
        .derive_default(true)
        .derive_debug(false)
        .opaque_type("xregs_state")
        .clang_arg(format!("--target={}", target));

    let flags = fs::read_to_string("c_flags").unwrap();
    let flags = flags.replace("\n", "");

    builder = builder.clang_arg("-Wno-unknown-warning-option");
    for flag in flags.split(" ") {
        if flag.is_empty() {
            continue;
        } else if !UNSUPPORTED_ARGS.contains(&flag) {
            println!("Adding supported flag {}", flag);
            builder = builder.clang_arg(flag.to_string());
        } else {
            println!("Ignoring unsupported flag {}", flag);
        }
    }

    println!("cargo:rerun-if-changed=rust_bindings.h");
    builder = builder.header("rust_bindings.h");

    for t in INCLUDED_TYPES {
        builder = builder.allowlist_type(t);
    }
    for f in INCLUDED_FUNCTIONS {
        builder = builder.allowlist_function(f);
    }
    for v in INCLUDED_VARS {
        builder = builder.allowlist_var(v);
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    /* Enable kernel versions */
    handle_kernel_version_cfg(&out_path.join("bindings.rs"));
}
