# scheme-rs

A quick and dirty port of [lidrisp](https://github.com/ebenpack/lidrisp/) and/or [hascheme](https://github.com/ebenpack/hascheme) into rust.

TL;DR it's an (almost certainly buggy and incomplete) implementation of scheme, written in rust.

Very much based on Write Yourself a Scheme in 48 Hours, with a few notable differences. E.g. more complete-ish number parsing and fuller numeric tower, let/let*/letrec expressions, a slightly more sophisticated environment model, etc.