ripfs (temporary name, to be changed)
=====

A stepping stone towards a rust implementation of an [IPFS][] client.

Examples
========

Since this is far from feature complete there is no real main yet and the
examples are all integrated into the main binary file. There are two special
cases:

```sh
$ cargo run -- listen
```

that will run this binary just listening for any incoming connections and

```sh
$ cargo run -- self
```

that will run this binary talking to a previously started listening copy of it.

You can also pass an address for an IPFS node to connect to

```sh
$ cargo run -- /ip4/127.0.0.1/tcp/4001/ipfs/QmcD3Pzo3kwvuZYNcxwEbefhmhR8s2ftd7zMkAWBwMhjax
```

[IPFS]: https://ipfs.io
