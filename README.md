# column ![Rust](https://github.com/YMatoi/column/workflows/Rust/badge.svg)

## Usage

```
USAGE:
    column [OPTIONS] <columns>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s <separator>        

ARGS:
    <columns>...   
```

```
$ cat hoge.txt
A B C D
E F G H
I J K L
M N O P

$ cat hoge.txt | column 3 1
D B
H F
L J
P N
```
