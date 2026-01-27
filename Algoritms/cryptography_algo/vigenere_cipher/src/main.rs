//venegre substition cipher
use std::io;

fn take_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn encrypt(plain_text: String, key: &String) -> String {
    let mut cipher_text = String::new();
    let key_size = key.len();
    let key_iter: Vec<char> = key.chars().collect();
    let mut itrator = 0;
    for ch in plain_text.chars() {
        let p = ch as u8 - b'A';
        let k = key_iter[itrator % key_size] as u8 - b'A';
        let c = (p + k) % 26 + b'A';
        cipher_text.push(c as char);
        itrator += 1;
    }
    cipher_text.to_string()
}

fn decrypt(cipher_text: String, key: &String) -> String {
    let mut plain_text = String::new();
    let key_size = key.len();
    let key_iter: Vec<char> = key.chars().collect();
    let mut itrator = 0;
    for ch in cipher_text.chars() {
        let c = ch as u8 - b'A';
        let k = (key_iter[itrator % key_size]) as u8 - b'A';
        let p = (c + 26 - k) % 26 + b'A';
        plain_text.push(p as char);
        itrator += 1;
    }
    plain_text.to_string()
}

fn main() {
    println!("Enter the key");
    let key: String = take_input().to_uppercase();
    println!("Enter your message");
    let plain_text = take_input().to_uppercase();
    let cipher_text = encrypt(plain_text, &key);
    println!("The message after encryption is: {}", cipher_text);
    println!(
        "The message after decryption is: {}",
        decrypt(cipher_text, &key)
    );
}
