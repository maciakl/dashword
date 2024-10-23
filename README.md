# DASHWORD

A command line utility that generates a *dashword* or arbitrary length.

### What is a dashword?

A *dashword* is a password type keyword composed of a dictionary word (of arbitrary length) followed by a dash and a number with an arbitrary number of digits.


### How to use this tool?

Simply run it on the command line:

    Usage: dashword [OPTIONS]

    Options:
    -l, --length <LENGTH>  Length of the word [default: 5]
    -d, --digits <DIGITS>  Number of digits after the dash [default: 2]
    -s, --simple           Use a word from a curated list
    -h, --help             Print help
    -V, --version          Print version

By default this tool will generate an 8 character *dashword* composed of a 5 character word, followed by a dash and 2 digit number.

 ### Installation

 Install via `cargo`:

     cargo install https://github.com/maciakl/dashword/ 
 
 On Windows, this tool is also distributed via `scoop` (see [scoop.sh](https://scoop.sh)).

 First, you need to add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket
    scoop update

 Next simply run:
 
    scoop install dashword
