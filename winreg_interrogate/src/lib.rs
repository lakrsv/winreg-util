use bytes::{Buf, Bytes};
use rayon::prelude::*;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;
use std::{fs, io};
use winreg_common::hive::{HiveBaseBlock, HiveBinCell, HiveBinHeader};

pub fn test() {
    let start = Instant::now();
    parse_registry();
    let duration = start.elapsed();
    println!("Time elapsed in parse_registry(): {:?}", duration);
}

fn parse_registry() {
    let mut bytes = Bytes::from(fs::read(Path::new("C://Users/lakrs/Documents/CreativeWork/RustProjects/winreg_util/export-test/HKEY_LOCAL_MACHINE-SOFTWARE.dat")).unwrap());

    let start = bytes.remaining();

    let base_block = HiveBaseBlock::build(&mut bytes).unwrap();

    println!("Base Block");
    println!(
        "File Name: {}, data = {:?}",
        &base_block.file_name(),
        &base_block
    );

    let mut my_blocks = vec![];
    let mut offset = 0;
    while start > offset {
        let hive_bin_header = HiveBinHeader::build(&mut &bytes[offset..offset + 32]).unwrap();
        if offset + hive_bin_header.size() as usize >= bytes.len() {
            break;
        }
        my_blocks.push(&bytes[offset..offset + 32 + hive_bin_header.size() as usize]);
        offset += hive_bin_header.size() as usize;
    }
    let block_count = my_blocks.len();

    let mut my_cells = my_blocks
        .par_iter_mut()
        .map(|bin| {
            let start_pos = bin.remaining();
            let mut cells: Vec<HiveBinCell> = vec![];
            let header = HiveBinHeader::build(bin).unwrap();
            while let Some(cell) = HiveBinCell::build(bin, start_pos, header.size()) {
                cells.push(cell);
            }
            return cells;
        })
        .flatten()
        .collect::<Vec<HiveBinCell>>();

    println!("Parsed the entire registry");
    println!("Got a total of {} hive bin blocks", block_count);
    println!("Got a total of {} cells", &my_cells.len());
}
