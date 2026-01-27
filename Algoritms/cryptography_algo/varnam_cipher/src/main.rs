use std::io;

fn take_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn encrypt(plain_text: String, key: &String) -> String {
    let key_size = key.len();
    let key_chars: Vec<char> = key.chars().collect();
    let mut cipher_text = String::new();
    let mut i = 0;
    for ch in plain_text.chars() {
        let p = ch as u8 - b'A';
        let k = key_chars[i % key_size] as u8 - b'A';
        let c = (p ^ k) + b'A';
        i += 1;
        cipher_text.push(c as char);
    }
    cipher_text.to_string()
}

fn decrypt(cipher_text: String, key: &String) -> String {
    let key_size = key.len();
    let key_chars: Vec<char> = key.chars().collect();
    let mut plain_text = String::new();
    let mut i = 0;
    for ch in cipher_text.chars() {
        let c = ch as u8 - b'A';
        let k = key_chars[i % key_size] as u8 - b'A';
        let p = (c ^ k) + b'A';
        i += 1;
        plain_text.push(p as char);
    }
    plain_text.to_string()
}

fn main() {
    println!("Enter the key");
    let key = take_input().to_uppercase();
    println!("Enter The Message");
    let plain_text = take_input().to_uppercase();
    let cipher_text = encrypt(plain_text, &key);
    println!("The message after Encryption is: {}", cipher_text);
    println!(
        "The cipher text after decryption is: {}",
        decrypt(cipher_text, &key)
    );
}
