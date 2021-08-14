#!/usr/bin/env python3

# Compile and Run (with rust_bundler_cp
# MIT LICENSE. Zhenbo Li

import subprocess
import os
import sys
import time

# As of 2021-08-06, Codeforces.com is using rust 1.49
RUST_VERSION = "1.49.0"
BUNDLER = "rust_bundler_cp"
TEMP_DIRECTORY = "/dev/shm"


def get_time_str():
    return time.strftime("%H_%M_%S", time.localtime())


BUNDLING_TIME = get_time_str()


def check_rust_toolkit():
    subprocess.run(["rustup", "default", RUST_VERSION])


def check_valid_cargo_directory():
    x = os.listdir()
    if 'Cargo.toml' not in x:
        print("Not a cargo project. Aborting")
        exit(1)


def bundle(binary) -> str:
    output_path = TEMP_DIRECTORY + "/problem_" + binary + "_" + BUNDLING_TIME
    subprocess.run([BUNDLER, "--input", ".", "--binary", binary, "--output", output_path + ".rs"])
    return output_path


def compile_rs(rs_file):
    subprocess.run(["rustc", rs_file + ".rs", "-o", rs_file])


def main():
    check_rust_toolkit()
    check_valid_cargo_directory()
    binary = "rust_codeforce_template"
    if len(sys.argv) >= 2:
        binary = sys.argv[1]
    rs_file = bundle(binary)
    compile_rs(rs_file)
    subprocess.run([rs_file])


main()
