# Iterating over indices of collections that implement `Index`

[![Build Status](https://travis-ci.org/joergbrech/graphtest.svg?branch=master)](https://travis-ci.org/joergbrech/graphtest)

[**Read the documentation**](https://joergbrech.github.io/graphtest/)

This is a code example to [this question on stack overflow](https://stackoverflow.com/questions/58260663/how-can-i-iterate-over-the-indices-of-a-generic-collection-that-implements-index).

 - [`src/ops.rs`](src/ops.rs) implements a supertrait for iterating over indices, see also [this example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=033fab2186cf7ca204ad7fa8978489fe).
 - [`src/lib.rs`](src/lib.rs) implements a minimalistic graph trait that uses the supertrait from `src/ops.rs`.