## Calcamabob
 - A command line calculator
 - Is written in Rust
 - Is not blazing fast, and could be faster.
 - Calcamabob can interpret operator precedence. For example: `calcamabob -e "2+3*4"` will return 14(`2+(3*4)`), and not 20(`(2+3)*4`).

### Usage
Make the binary and execute it with the `expression` command line argument followed by an expression, like so:

```sh
calcamabob --expression "sin(pi)^2"
calcamabob -e "2+3*4"
```

Alternatively, provide a path to a file which contains an arithmetic expression inside of it and call the `file` command line argument.

```sh
calcamabob --file "/path/to/expression.txt"
calcamabob -f "/path/to/expression.txt"
```

### Motivation
I never tried to make a programming language before, so a calculator is a good excercise to do that.

### References:
I needed help getting a Pratt parser started and found a good tutorial for it: https://willspeak.me/2016/09/03/top-down-operator-precedence-parsing-in-rust.html

### Warning:
This code hasn't be tested for every condition. Meaning, you might get an incorrect answer at some point.