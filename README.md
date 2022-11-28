# Aho Corasick algorithm

## Getting started

In the algorithmic course (SA4T) of 4 MIN at ECAM.

We implemented the Aho Corasick algorithm. It is used for pattern searching with a time complexity of O(n + m + z) where :

* n: Size of the text
* m: Total character in all words (patterns)
* z: Total numbers of occurences in the text

## Setup

To setup the application, you must have installed [Cargo][cargo].


When it's done, go into the project directory.

```bash
cd aho-corasick
```

Install the dependencies using the following command:

```bash
cargo update
```

## Launch 

You can launch example that are in the [example folder][example-folder] with the following command:

```bash
cargo run --example <example>
```

As an illustration, if you want to run the basic example.

```bash
cargo run --example basic
```

<!--- Local links -->
[example-folder]: ./aho-corasick/examples/

<!--- Links -->
[cargo]: https://crates.io/