extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main(){
    let input = "Text to encrypt";

    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
}