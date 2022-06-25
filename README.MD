## Calcamabob
 - A command line calculator
 - Is written in Rust
 - Is not blazing fast, and could be faster.
 - Can do basic math and handles operator precedence

### Usage
Make the binary and execute it with the `eqn` command line argument followed by an expression, like so:

```sh
calcamabob --eqn="sin(pi)^2"
calcamabob --eqn="2+3*4"
```

Calcamabob can interpret operator precedence, for example:
```calcamabob --eqn="2+3*4"```
Will return 14(`2+(3*4)`), and not 20(`(2+3)*4`)

### Motivation
I never tried to make a programming language before, so a calculator is a good excercise to do that.

### References:
I needed help getting a Pratt parser started and found a good tutorial for it: https://willspeak.me/2016/09/03/top-down-operator-precedence-parsing-in-rust.html