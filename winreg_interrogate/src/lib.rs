use bytes::Bytes;
use rayon::ThreadPool;
use std::fs;
use std::path::Path;
use std::time::Instant;
use winreg_common::hive::parse_registry;

pub fn test() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().unwrap().get())
        .build()
        .unwrap();
    for _ in 0..100 {
        let start = Instant::now();
        run_parse_registry(&pool);
        let duration = start.elapsed();
        println!("Time elapsed in parse_registry(): {:?}", duration);
    }
}

fn run_parse_registry(pool: &ThreadPool) {
    let bytes = Bytes::from(fs::read(Path::new("C://Users/Lars/Documents/CreativeWork/Projects/Rust/winreg-util/hive/HKEY_LOCAL_MACHINE-SOFTWARE.dat")).unwrap());
    let hive_primary_file = parse_registry(bytes, pool).unwrap();
    println!("Parsed the entire registry");
}
