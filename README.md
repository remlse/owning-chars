# Owning Chars

An alternative implementation of Rust's `str::chars` which works on owned strings and consumes them, meaning the resulting iterator is not tied to a lifetime.

This is an exercise in self referential data structures, inspired by [this fantastic article](https://morestina.net/blog/1868/self-referential-types-for-fun-and-profit).
