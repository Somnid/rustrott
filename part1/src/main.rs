extern crate clap;
extern crate regex;
mod bytes;
mod lump_info;

use clap::{App, Arg, SubCommand};
use regex::{Regex};
use std::fs;
use std::vec::Vec;
use std::path::{Path};

use crate::lump_info::{LumpInfo};

fn main() {
    let matches = App::new("WAD Reader Util")
                    .subcommand(SubCommand::with_name("ls")
                        .arg(Arg::with_name("file")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("find")
                            .required(false)
                            .short("f")
                            .long("find")
                            .takes_value(true)))
                    .subcommand(SubCommand::with_name("txt")
                        .arg(Arg::with_name("file")
                        .required(true)
                        .index(1)))
                .get_matches();

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

            println!("type: {}", r#type); 
            println!("# of lumps: {}", num_lumps); 
            println!("info table offset: {}", info_table_offset);
            println!("-------------------");

            let find = fs_matches.value_of("find");
            match find {
                Some(val) => {
                    let re = Regex::new(&val.to_uppercase()).unwrap();
                    for lump in lumps {
                        if re.is_match(&lump.name) {
                            println!("{}", lump);
                        }
                    }
                },
                None => {
                    for lump in lumps {
                        println!("{}", lump);
                    }
                }
            }
        }
        ("txt", Some(txt_matches)) => {
            let filename = txt_matches.value_of("file").unwrap();
            let contents = fs::read_to_string(Path::new(filename)).unwrap();
            println!("{}", contents);
        }
        _ => println!("Please supply a valid subcommand")
    }
}
