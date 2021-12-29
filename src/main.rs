use md5::{Md5, Digest};
use std::{fs, io, path};
use std::env;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    let cli_path = &args[1];
    
    if !path::Path::new(&cli_path).exists() {
        println!("{:?} does not exist", &cli_path);
        panic!(" {} does not exist", &cli_path)
    }
   
    // Ready Directory where duplicates should be checked
    let path = fs::read_dir(cli_path).unwrap();


    // Vecs are dynamic arrays in Rust 
    let mut arr : Vec<String> = vec![];
    
    for file in path {
        
        // gets File
        let mut file = fs::File::open(file.unwrap().path())?;
        
        //Initializes MD5 Hash
        let mut hasher = Md5::new();
    
        // Hashes file
        let _n = io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();
  
        // MD5 Hash from bytes to Hex String
        let str_hash = format!("{:x}", hash);
        
        // Check if file is already in Vector
        if arr.contains(&str_hash) {
            println!("{} duplicated", &str_hash)
        } else {
            arr.push(str_hash);
        }

    }
    
    Ok(())
}
