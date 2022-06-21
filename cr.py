#!/usr/bin/env python3

# Compile and Run (with rust_bundler_cp)
# MIT LICENSE. Zhenbo Li

import subprocess
import os
import sys
import time
import platform

# As of 2021-08-06, Codeforces.com is using rust 1.49
RUST_VERSION = "1.49.0"
BUNDLER = "rust_bundler_cp"


def check_temporary_path():
    if 'WSL' in platform.uname()[2]: return "../../../Downloads"
    else: return "/dev/shm"


TEMP_DIRECTORY = check_temporary_path()

BACKUP_DIRECTORY = "backup"
RS_FILE_DIRECTORY = "src/bin/"
TEMPLATE_RS_FILE_NAME = "_template.rs"


def get_time_str():
    return time.strftime("%H_%M_%S", time.localtime())


BUNDLING_TIME = get_time_str()


def check_rust_toolkit():
    if 'WSL' in platform.uname()[2]:
        # Use WSL to skip it is a bit hacky
        print("Skipped rust version enforce")
        return
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


def reset_workspace():
    backup_dir = BACKUP_DIRECTORY + "/" + BUNDLING_TIME + "/"
    subprocess.run(["mkdir", "-p", BACKUP_DIRECTORY])
    subprocess.run(["mkdir", "-p", backup_dir])
    for filename in os.listdir(RS_FILE_DIRECTORY):
        if not filename.endswith("rs"):
            continue
        if filename == TEMPLATE_RS_FILE_NAME:
            continue
        subprocess.run(["mv", RS_FILE_DIRECTORY+filename, backup_dir + filename])
        subprocess.run(["cp", RS_FILE_DIRECTORY + TEMPLATE_RS_FILE_NAME, RS_FILE_DIRECTORY+filename])
    print("Previous result code backed up tp " + backup_dir)
    exit(0)


def check_bleeding_edge_bundler():
    global BUNDLER
    bleed = "./" + BUNDLER
    if os.path.exists(bleed):
        print("Using bleeding edge bundler")
        BUNDLER = bleed


def main():
    check_rust_toolkit()
    check_valid_cargo_directory()
    check_bleeding_edge_bundler()


    binary = "rust_codeforce_template"

    if "--reset" in sys.argv:
        reset_workspace()

    if len(sys.argv) >= 2:
        binary = sys.argv[1]
    rs_file = bundle(binary)
    compile_rs(rs_file)
    subprocess.run([rs_file])


main()
