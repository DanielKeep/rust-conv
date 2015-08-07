
# `conv`

This crate provides a number of conversion traits with more specific semantics than those provided by `as` or `From`/`Into`.

The goal with the traits provided here is to be more specific about what generic code can rely on, as well as provide reasonably self-describing alternatives to the standard `From`/`Into` traits.  For example, the although `T: From<U>` might be satisfied in generic code, this says nothing about what *kind* of conversion that represents.

In addition, `From`/`Into` provide no facility for a conversion failing, meaning that implementations may need to choose between conversions that may not be valid, or panicking; neither option is appealing in general.

([Documentation for the master branch](https://danielkeep.github.io/rust-conv/doc/conv/index.html).)
