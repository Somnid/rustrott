The WAD Format, Side B
======================

This time we're not building a web app but a command-line program.  To do so we're going to use the crate [clap](https://crates.io/crates/clap).  I picked this because I've heard good things.  There is another popular crate for this called [structop](https://crates.io/crates/structopt) that seems to be built on top of `clap` but for no particular reason I decided not to go with it.  If you want to follow along you may choose differently.

`clap` is pretty nice in that you can use a builder patter to not only get the arguments but also to add help text and such automatically.  Since rust is much harder to debug, I started slower.  Instead of jumping into the wad, I read a text file to make sure everything was going well.  That's retained in the code but will probably be removed later.  The utility is called `wad` and will take different subcommands.  The only one aside from the aforementioned `txt`, is `ls` which will list out the contents of the wad.  It also has an argument `-f` or `--find` that does regex matching because as we saw in webrott there might be hundreds of files (and yes, rust needs an external dependency for regexs).

Rust does have some nicer built-in functionality for converting things to and from bytes. I started by using cursors but found that to just be too cumbersome so I reverted back to manual offsets.  Rust also has this concept of views of data versus actual data.  Basically we can be really efficent with reusing data in memory in a safe way. Unfortunately, this is also hard and we will need to have more fights with the borrow checker which is Rust's mechanism to make sure nothing bad happens with those references to memory.  I initially wanted to just read in the wad and have everything be a view into it.  I think I've learned enough that for sanity we won't strive for optimum efficency like that, we'll just clone data if we need to, it's still way faster than js.  This might also work out for WASM as we'll want an abstraction that isn't a file but rather a series of bytes.

Here's the meat of reading the wad:

```rust
match matches.subcommand(){
    ("ls", Some(fs_matches)) => {
        let filename = fs_matches.value_of("file").unwrap();
        let data = fs::read(filename).unwrap();

        let r#type = bytes::str_from_slice(&data, 0, 4);
        let num_lumps = bytes::u32_from_slice(&data, 4);
        let info_table_offset = bytes::u32_from_slice(&data, 8);

        let mut lumps = Vec::new();

        for lump_index in 0..num_lumps {
            lumps.push(LumpInfo {
                offset: bytes::u32_from_slice(&data, (info_table_offset + (lump_index * 16)) as usize),
                size: bytes::u32_from_slice(&data, ((info_table_offset + (lump_index * 16) + 4)) as usize),
                name: bytes::str_from_slice(&data, (info_table_offset + (lump_index * 16) + 8) as usize, 8)
            });
        }
```

To run the program you can compile it and run it as `wad ls {wadfile}`  You'll get a giant list of files.  You can filter like this `wad ls {wadfile} -f wall` to get a list of things with the string "wall" in the name.  Or while testing `cargo run -- ls {wadfile}`.