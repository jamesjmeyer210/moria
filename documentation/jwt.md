# JWT Equation

## The Variables and Operators

 - Let `t` be some token.
 - Let `h` be some header.
 - Let `m` be some message digest.
 - Let `s` be some secret.
 - Let `+` be a string concationation operator.
 - Let `H(x)` be any hash function.
 - Let `t` be the token result.

## The Equation

`t = h + '.' + m + '.' + H(h + '.' + m + '.' + s)`
