# DASHWORD

A command line utility that generates a *dashword* or arbitrary length.

### What is a dashword?

A *dashword* is a password type keyword composed of a dictionary word (of arbitrary length) followed by a dash and a number with an arbitrary number of digits.


### How to use this tool?

Simply run it on the command line:

    Usage: dashword [OPTIONS]

    Options:
    -l, --length <LENGTH>  Length of the word [default: 12]
    -d, --digits <DIGITS>  Number of digits after the dash [default: 3]
    -s, --simple           Use a word from a curated list
    -h, --help             Print help
    -V, --version          Print version

By default this tool will generate an 16 character *dashword* composed of a 12 character word, followed by a dash and a 3 digit number. The first letter will be capitalized. This should be a sufficient length and complexity to use as an easy to remember, throwaway password in most systems:

- 16 characters
- Upper and lower case
- At least one number
- Non alphanumeric character (the eponymous dash `-`)

If needed you can override the lenght of the word and the number as needed (capping out at 15 character words and 10 digit numbers).

#### Output Examples

    Thingamabobs-046
    Denaturation-653
    Patchinesses-880
    Nonconcurred-205
    Astrological-170
    Misallotting-442
    Compulsively-302
    Sugarcoating-250
    Lentiviruses-728
    Beneficiated-250

 ### Installation

 #### Platform Intependent

 Install via `cargo`:

     cargo install https://github.com/maciakl/dashword/ 

#### Linux and macOS

You can install via [grab](https://github.com/maciakl/grab):

    grab maciakl/dashword

#### Windows
 
 On Windows, this tool is also distributed via `scoop` (see [scoop.sh](https://scoop.sh)).

 First, you need to add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket
    scoop update

 Next simply run:
 
    scoop install dashword
