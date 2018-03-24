# Assignment 5
##### Sam Rogers
##### Prof. Craig Tanis
##### 26 March 2018

For this assignment I have chosen [Rust](https://www.rust-lang.org/) as the
language in which to re-implement the Scheme functions from Assignment 4.
Instructions for use follow this paragraph, and further down I compare and
contrast the implementations in two different languages as specified.

## Usage

Run unit tests with `cargo test`.

## Observations

The jump from Scheme to Rust is a big one. I am taking algorithms designed
in a decades-old, primarily functional language and adapting them to a
low-level multiparadigm language whose first stable release appeared only
three years ago. Here are some of the challenges I encountered, in the same
order they reared their ugly heads:

1. static typing: Rust demands to know beforehand whether that integer is
32 or 64 bits. In contrast, Scheme won't complain about type issues
 until well after everything goes wrong. 
2. collections: Scheme does not have generics. Hardly surprising given that
it hardly enforces anything about data types. Rust, however, enforces
type homogeneity in collections! This complicated my "simple" math tricks,
leading ultimately to the use of external libraries.
3. language system: Rust has a richer and more tightly integrated build process
and ecosystem than Scheme. To be fair to the latter, though, Guile is not
the only Scheme ever hatched, not to mention other Lisps like Clojure.
4. execution: Rust runs on "bare metal". Scheme compiles to bytecode and
runs on a VM, like Java, Elixir, and .NET languages.
 
## Conclusions

1. Linguists often find that people have trouble imagining concepts for which
the languages they speak have no word. In software this translates not only
to "if you only have a hammer" but also to an inflexibility in the crucial 
pre-implementation phases due to having a limited vocabulary of computation
models to draw on in analyzing a problem.

2. In this case, the implementation was far easier in a functional language
like Scheme. However, the loose typing that allows this makes it far more 
difficult to find where it went wrong when it inevitably does. Rust was a
huge pain to work with in this case but offers reliability crucial to real-world applications.

3. The assignment help me gain a clearer appreciation of what it means to pick
"the right tool for the job". No language is objectively superior or inferior to
another. ~(Unless one is PHP, in which case the other is objectively superior.)~
