# tlc
A small utility program that gets the **t**otal **l**ine **c**ount all files in the current directory (and sub-directories).

## Building
Assuming you installed [Rust](https://www.rust-lang.org/tools/install), run the following in your CLI:
```
cargo build --release
```
The release executable will be put in `target/release`.

## Setup
This assumes a Windows OS. Steps may differ for Linux or Mac.

1. Make sure you get the release build of this executable. See the previous section for more information.
2. Put the executable (from step 1) into a folder (preferably containing other executables/utilities). Then, in your
   user environmental variables, put the path to this folder under the variable `PATH`.
3. You should be able to access `lc` from the CLI.
