## Calcamabob
 - A command line calculator
 - Is written in Rust
 - Is not blazing fast
 - Can do basic math

### Usage
Make the binary and execute it with the `eqn` command line argument followed by an expression, like so:

```sh
calcamabob --eqn="sin(pi)^2"
calcamabob --eqn="2+3*4"
```

### Motivation
I never tried to make a programming language before, so a calculator is a good excercise to do that.

### References:
I needed help getting a Pratt parser started and found a good tutorial for it: https://willspeak.me/2016/09/03/top-down-operator-precedence-parsing-in-rust.html