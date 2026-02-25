use memmap2::Mmap;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, Read};

fn main() {
    println!("Memory Mapped File Example");

    // Open the file
    let mut file = File::open("README.md").expect("Failed to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Failed to read file");

    // for line in contents {
    //     print!("{}", line as char);
    // }

    // Memory map the file
    let map = unsafe { Mmap::map(&file).expect("Failed to map file") };

    // compare the contents read from the file with the memory-mapped contents
    assert_eq!(contents, &*map);
}
