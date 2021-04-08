# Rust portscanner

This is my try at making a portscanner with rust, it's very basic at the moment but it will be improved upon.

## Usage
```
USAGE:
    portscan [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --mode <mode>        Choose one of the available modes [default: default]  [possible values: default, t1000]
    -t, --target <target>    The target
```

## Modes
Currently the scanner is able to perform port scans using 2 modes:
* Default: scans ports 1-65535
* t1000: scans the top 1000 most popular ports

