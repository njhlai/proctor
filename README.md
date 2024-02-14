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
```json
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

### Setup
Then, set up the local dev environment based at `sol_dir` for `proctor` by running:
```sh
proctor setup
```

### Installing libraries
To install the language-specific libraries for coding problems, compile each language-specific libraries, which are under the `lib/` directory.
* `C++`:
    ```sh
    clang++ -std=c++20 -stdlib=libc++ -Wall -I${PWD}/lib/cpp/src -c -fPIC lib/cpp/src/[PROBLEM_SOURCE]/[DATA_STRUCTURE].cpp -o lib/cpp/build/[PROBLEM_SOURCE]_[DATA_STRUCTURE].o
    clang++ -std=c++20 -stdlib=libc++ -Wall -shared lib/cpp/build/*.o -o lib/cpp/build/libproctor.so
    ```
* `Python`:
    ```sh
    source [PATH_TO_SOLUTIONS_DIRECTORY]/venv/py311/bin/activate
    pip install -e lib/cpp/py
    ```
* `Rust`:
    ```sh
    cargo build --release --locked -p proctor
    ```

### Running
To compile and test a solution, run:
```sh
proctor run [PROBLEM_ID] [LANG]
```
`proctor` will then compile and test the solution at `[PATH_TO_SOLUTIONS_DIRECTORY]/leetcode/[PROBLEM_ID]/sol.[LANG]`.
