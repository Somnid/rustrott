Rustrott
========

This is the B-side to [webrott](https://github.com/ndesmic/webrott).  A second implementation in a different language with very different pricipals.  There are many reasons to do this:

1) I get to use Rust more
2) There might be a wider audience that's not being served with a web-first exercise
3) Rust is much better suited to the binary data we need to deal with
4) Re-implementing in a different language is likely to bring out new ideas
5) Rust has much better performance
6) There is still reason to have native apps (at least on PC, I'm no so sure about mobile)
7) A WASM cross-over is a long term goal

To that end we'll probably start with the same stuff and see where it takes us.  This will be treated as a suppliment to webrott so I won't go over the algorithms again but will probably outline the differences when dealing with Rust.  I don't think I'll be implementing everything from scratch here, the tradeoffs of bloated 3rd party libraries is different and honestly I feel the quality of the Rust ecosystem is stronger because so much emphasis is on performance so I'm less concerned.  Also, you just get far less out of the box compared to the batteries included web platform and I can't implement every little thing.

Table of Contents
-----------------

- [Part 1 - The Wad Format](part1/wad.md)

Working with the code
---------------------

Like with webrott I'm just copying a folder forward to document the learning process until I find the file duplication so unsustainable I quit.  Not sure how well it will work but that's my thought.  You just need to run cargo in each folder.