# Intro

e**x**tended **st**andar**d** library for rust.

# Why?

Things which i miss in rust's standard library.

# Features

Currently not much.

API doc: https://j-keck.github.io/xstd-rs/

# Usage

Add `xstd` to the dependencies section in your `Cargo.toml`

    [dependencies]
    xstd = "0.2"

Import everything:

    use xstd::prelude::*;

Or select only what you need:

    use xstd::vec::Unlines;
