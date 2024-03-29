# proctor

## About
proctor is a framework in [Rust](https://www.rust-lang.org/) that setups up a local environment for coding challenges, such as those from [LeetCode](https://leetcode.com), allowing one to attempt the coding challenge in the comfort of one's IDE/editor and dev environment.

Currently supported languages:
* `C++`
* `Python`
* `Rust`

## Requirements
### General Requirements
* [Rust](https://www.rust-lang.org/). Recommended to use Rust through [rustup](https://rustup.rs/).

### Language-Specific Requirements
* `C++`
    * [Clang](https://clang.llvm.org/): ver >= `17`.
    * [libc++](https://libcxx.llvm.org/): ver >= `17`.
* `Python`
    * [Python](https://www.python.org/): ver >= `3.11`.
    * [Pyenv](https://github.com/pyenv/pyenv).
* `Rust`
    * [Rust](https://www.rust-lang.org/): ver >= `1.74.1`.

## Usage
### Building from source
To build from source, clone this repository, and then run:
```sh
cargo build --release --locked -p runner
```
The built binary will be `target/release/proctor`.

### Configuration
Configure `proctor` by supplying a `config.json` file:
```
{
  "project_dir": "{{ PATH_TO_PROCTOR }}",
  "sol_dir": "{{ PATH_TO_SOLUTIONS_DIRECTORY }}",
  "lang": {
    // ...
    "{{ LANG_EXT }}": {
      "{{ LSP }}": {
        // {{ CONFIGURATION_DETAILS }}
      },
      // {{ OTHER_LANGUAGE_CONFIGURATIONS }}
    }
    // ...
  }
}
```
Refer to `example_config.json` for more details on configuration for different languages and LSPs.

### Setting up `sol_dir`
To set up the local dev environment based at `sol_dir` for `proctor`, run:
```sh
proctor setup
```

### Installing libraries
To install the language-specific libraries for coding problems, compile each language-specific libraries, which are under the `lib/` directory.
* `C++`:
    ```sh
    clang++ -std=c++20 -stdlib=libc++ -Wall -I${PWD}/lib/cpp/src -c -fPIC lib/cpp/src/[SOURCE]/[DATA_STRUCTURE].cpp -o lib/cpp/build/[SOURCE]_[DATA_STRUCTURE].o
    clang++ -std=c++20 -stdlib=libc++ -Wall -shared lib/cpp/build/*.o -o lib/cpp/build/libproctor.so
    ```
* `Python`:
    ```sh
    source [PATH_TO_SOLUTIONS_DIRECTORY]/venv/py311/bin/activate
    pip install -e lib/py
    ```
* `Rust`:
    ```sh
    cargo build --release --locked -p proctor
    ```

### Running
#### Fetching question
To fetch a question, run:
```sh
proctor fetch ID LANG [SOURCE]
```
`proctor` will fetch data related to the question specified and render it as `[PATH_TO_SOLUTIONS_DIRECTORY]/[SOURCE]/[ID]/sol.[LANG]`.

#### Compile and test solution
To compile and test a solution, run:
```sh
proctor run ID LANG [SOURCE]
```
`proctor` will compile and test the solution at `[PATH_TO_SOLUTIONS_DIRECTORY]/[SOURCE]/[ID]/sol.[LANG]`.
