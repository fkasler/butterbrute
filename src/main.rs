use std::env;
use std::fs;
use std::{num::NonZeroU32};
use ring::{pbkdf2, hmac};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::atomic::{AtomicBool, Ordering};
use rayon::prelude::*;

fn gen_key(password: &str, salt: &str, iterations: u32, key_size: usize) -> String {
   let mut pbkdf2_key = vec![0u8; key_size];
    let algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

    pbkdf2::derive(
        algorithm,
        NonZeroU32::new(iterations).unwrap(),
        salt.as_bytes(),
        password.as_bytes(),
        &mut pbkdf2_key,
    );

    let result = hex::encode(pbkdf2_key);
    let len = result.len();
    let second_half = &result[len / 2..];
    second_half.to_string()
}

fn calculate_hmac(input: &str, key: &str) -> String {
    let key_bytes = hex::decode(key).expect("Invalid key format");
    let input_bytes = input.as_bytes();
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_bytes);
    let tag = hmac::sign(&key, input_bytes);
    hex::encode(tag.as_ref())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Useage: ./butterbrute vault.bcup /path/to/password/dictionary/passwords.txt");
        return;
    }
    let vault_file = &args[1];
    let vault_content = fs::read_to_string(vault_file).expect("Failed to read the vault file.");
    let sections: Vec<&str> = vault_content.strip_prefix("b~>buttercup/").unwrap().split_at(1).1.split('$').collect();
    let base64_encoded_content = sections[0];
    let iv = sections[1];
    let salt = sections[2];
    let auth = sections[3];
    let iterations: u32 = sections[4].parse().unwrap();
    let password_file = &args[2];
    let input = format!("{}{}{}", base64_encoded_content, iv, salt);
    let key_size = 512 / 8;
    let file = File::open(password_file).expect("Failed to open password file");
    let reader = BufReader::new(file);
    let should_stop = AtomicBool::new(false);
    reader.lines().par_bridge().for_each(|line| {
        if should_stop.load(Ordering::Relaxed) {
           return
        }
        let line = line.expect("Failed to read line in password file");
        let line = line.trim();
        let guess = gen_key(line, salt, iterations, key_size); 
        let hmac = calculate_hmac(&input, &guess);
        match hmac == auth {
          true => {
            println!("Found Password:{}", line);
            should_stop.store(true, Ordering::Relaxed);
            return
          },
          false => ()
        }
    })
}
