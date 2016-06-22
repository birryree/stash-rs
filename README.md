# stash-rs [![Master branch build status](https://app.wercker.com/status/daab2a54e88010b7b0e5906d855435e2/s/master "wercker status")](https://app.wercker.com/project/bykey/daab2a54e88010b7b0e5906d855435e2)

An (experimental) Rust library for interacting with the Atlassian Stash API.

## Documentation

Coming soon...

## Overview

Stash is a wrapper around the Atlassian Stash API, which provides typesafe builders and wrappers around the datatypes the API returns.

## Example Usage

TBD

## Supported APIs

API support is built into the library as I need it, so I may not personally implement every bit functionality that the Stash API supports.

The currently supported APIs are:

* Parts of [Stash core](https://developer.atlassian.com/static/rest/stash/latest/stash-rest.html)

## Information for Building Locally

- Continuous integration is currently performed on Debian Jessie using Rust 1.8-stable.
- Development is done using Rust 1.8 on Mac OSX El Capitan

Build dependencies are:

* OpenSSL headers (Debian users can install `libssl-dev` for this)

## License

Distributed under [Apache 2.0](https://raw.githubusercontent.com/birryree/stash-rs/master/LICENSE), because information wants to be free.

Copyright &copy; 2016 William Lee. All rights reserved.
