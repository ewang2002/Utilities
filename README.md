# Utilities
A series of utility/random programs that I've made for, well, random reasons. 

These are designed to be *small* projects, usually not worthy of having its own repository. Thus, instead of creating a repository for each project, I've decided to condense them into one repository.

A description of each program can be found in the associated folder.

## Building & Setup
Most of the projects I have will follow a very similar building/setup process. They are explained below. For projects which require slightly more advanced setup, those instructions will be made available in the corresponding project's README.

### Building
Assuming you installed [Rust](https://www.rust-lang.org/tools/install), run the following in your CLI:
```
cargo build --release
```
The release executable will be put in `target/release`.

### Setup
This assumes a Windows OS. Steps may differ for Linux or Mac.

1. Make sure you get the release build of this executable. See the previous section for more information.
2. Put the executable (from step 1) into a folder (preferably containing other executables/utilities). Then, in your
   user environmental variables, put the path to this folder under the variable `PATH`.
3. You should be able to access the executable from the CLI.

## License
All projects here are made available under the MIT license.