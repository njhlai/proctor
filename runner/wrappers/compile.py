import argparse
import py_compile
from py_compile import PyCompileError
import sys
from traceback import print_exc


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="A wrapper around py_compile module.", exit_on_error=False
    )
    parser.add_argument("file", help="File to compile.")
    parser.add_argument(
        "-o",
        "--output",
        type=str,
        action="store",
        help="The output file to compile to.",
        required=True,
    )

    try:
        args = parser.parse_args()
    except argparse.ArgumentError:
        print_exc()
        sys.exit(1)

    try:
        compiled = py_compile.compile(
            file=args.file, cfile=args.output, doraise=True, quiet=0
        )
        if compiled is None:
            print(
                f"ERROR: Something went wrong while compiling solution with `py_compile`"
            )
            sys.exit(1)
    except PyCompileError:
        print_exc()
        sys.exit(1)
    except FileExistsError:
        print_exc()
        sys.exit(1)
