// Caesar cipher it is an substitution cipher
use std::io;

fn take_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn encrypt(plain_text: String, key: u8) -> String {
    let mut cipher_text = String::new();
    for ch in plain_text.chars() {
        if ch.is_ascii_uppercase() {
            cipher_text.push(((ch as u8 - b'A' + key) % 26 + b'A') as char);
        } else {
            cipher_text.push(ch);
        }
    }
    cipher_text
}
fn decrypt(cipher_text: String, key: u8) -> String {
    let mut plain_text = String::new();
    for ch in cipher_text.chars() {
        if ch.is_ascii_uppercase() {
            plain_text.push(((ch as u8 - b'A' + 26 - key) % 26 + b'A') as char);
        } else {
            plain_text.push(ch);
        }
    }
    plain_text
}

fn main() {
    println!("Enter the key");
    let key = take_input().parse().unwrap();
    println!("Enter your message:");
    let message: String = take_input().to_uppercase();
    let cipher_text = encrypt(message, key);
    println!("The cipher text of the message is: {}", cipher_text);
    println!(
        "After deciphering the cipher text the message is: {}",
        decrypt(cipher_text, key)
    );
}
