use bytes::{Buf, Bytes};
use std::{fs, io};
use std::io::stdin;
use std::path::Path;
use winreg_common::hive::{HiveBaseBlock, HiveBinCell, HiveBinHeader};

pub fn test() {
    let mut bytes = Bytes::from(fs::read(Path::new("C://Users/lakrs/Documents/CreativeWork/RustProjects/winreg_util/export-test/HKEY_LOCAL_MACHINE-SOFTWARE.dat")).unwrap());

    let start = bytes.remaining();

    let base_block = HiveBaseBlock::build(&mut bytes).unwrap();

    println!("Base Block");
    println!(
        "File Name: {}, data = {:?}",
        &base_block.file_name(),
        &base_block
    );

    let mut cells = vec![];

    loop {
        let hive_bin_start_pos = bytes.remaining();
        if hive_bin_start_pos == 0 {
            break;
        }
        let hive_bin_header = HiveBinHeader::build(&mut bytes).unwrap();

        // println!("");
        // println!("Hive Bin Header");
        // println!("{:?}", hive_bin_header);
        // println!("");
        println!("Bytes read: {}", start - bytes.remaining());
        while let Some(cell) =
            HiveBinCell::build(&mut bytes, hive_bin_start_pos, hive_bin_header.size())
        {
            cells.push(cell);
            let bytes_read = start - bytes.remaining();
            let hive_bin_bytes_read = hive_bin_start_pos - bytes.remaining();

            // println!("Bytes read: {}", bytes_read);
            // println!("Hive Bin Bytes read {}", hive_bin_bytes_read);
            //
            // println!("Hive Bin Cell");
            // println!("{:?}", cell);
            // println!("");

            // if let CellData::NamedKey(key) = cell.cell_data() {
            //     // Got Key Node
            //     println!("Key Node Name: {}", key.key_name());
            // }
        }
    }
    println!("Parsed the entire registry");
    println!("Got a total of {} cells", cells.len());
    // let mut buffer = String::new();
    // let stdin = io::stdin();
    // for cell in cells {
    //     println!("{:?}", cell);
    //     stdin.read_line(&mut buffer).unwrap();
    // }
}
