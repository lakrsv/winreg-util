use bytes::Bytes;
use std::fs;
use std::path::Path;
use std::time::Instant;
use winreg_common::hive::parse_registry;

pub fn test() {
    for _ in 0..100 {
        let start = Instant::now();
        run_parse_registry();
        let duration = start.elapsed();
        println!("Time elapsed in parse_registry(): {:?}", duration);
    }
}

fn run_parse_registry() {
    let bytes = Bytes::from(fs::read(Path::new("C://Users/lakrs/Documents/CreativeWork/RustProjects/winreg_util/export-test/HKEY_LOCAL_MACHINE-SOFTWARE.dat")).unwrap());
    let hive_primary_file = parse_registry(bytes).unwrap();
    println!("Parsed the entire registry");
}
