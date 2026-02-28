use std::io;

const REG_SIZE: usize = 32;

const BLOCK_SIZE: usize = 512;
const PADDING_START: u32 = 0b10000000000000000000000000000000; //stating with 10000
const PADDING: u32 = 0; // ususal 000000
const CONSTANT: u32 = 0b01010110111000111001110101110110; // totally random created by me heheheheheh
const SHIFT: u32 = 14;

fn take_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn convert_to_32_bits_vector(text: &str) -> Vec<u32> {
    let mut text_32_bits_vec: Vec<u32> = Vec::new();

    for t in text.chars() {
        text_32_bits_vec.push(t as u32);
    }

    text_32_bits_vec
}
fn cal_total_blocks(len: usize) -> usize {
    (((len as f64 * REG_SIZE as f64) + 64.0) / BLOCK_SIZE as f64).ceil() as usize
}

fn gen_512_bits_blocks(
    mut text_32_bits_vec: Vec<u32>,
    len: usize,
    total_blocks: usize,
) -> Vec<Vec<u32>> {
    let required_len = total_blocks * BLOCK_SIZE;

    if (len * REG_SIZE) < required_len - 64 {
        text_32_bits_vec.push(PADDING_START);

        if (len * REG_SIZE) < required_len - 64 {
            text_32_bits_vec.resize(required_len - 64, PADDING);
        }
    }

    // this converts the length of the message in vec of 32 bits
    let higher_32_bits = (len >> 32) as u32;
    let lower_32_bits = (len & 0xFFFF_FFFF) as u32;

    text_32_bits_vec.push(higher_32_bits);
    text_32_bits_vec.push(lower_32_bits);

    let mut blocks_of_512_bits: Vec<Vec<u32>> = Vec::new();

    for chunk in text_32_bits_vec.chunks(BLOCK_SIZE) {
        // 16 u32 per block
        blocks_of_512_bits.push(chunk.to_vec());
    }

    blocks_of_512_bits
}

fn process(round: u8, REG_B: u32, REG_C: u32, REG_D: u32) -> u32 {
    match round {
        0 => ((REG_B & REG_C) | ((!REG_B) & REG_D)),
        1 => ((REG_B & REG_D) | (REG_C & (!REG_D))),
        2 => (REG_B | REG_C | REG_D),
        3 => (REG_C | REG_B | (!REG_D)),
        _ => {
            panic!("Invalid Round");
        }
    }
}

fn create_hash(password: String) -> String {
    // Inital Registers
    let mut REG_A: u32 = 0x01234567; // 0x for hex value
    let mut REG_B: u32 = 0x89ABCDEF;
    let mut REG_C: u32 = 0xFEDCBA98;
    let mut REG_D: u32 = 0x76543210;
    let len = password.len();
    let total_blocks = cal_total_blocks(len);
    let text_32_bits_vec = convert_to_32_bits_vector(&password);
    let blocks_of_512_bits = gen_512_bits_blocks(text_32_bits_vec, len, total_blocks);

    for block in blocks_of_512_bits {
        for round in 0..4 {
            for reg_32_bits in &block {
                let process_32_bits = process(round, REG_B, REG_C, REG_D);
                let mut temp_reg_a = REG_A.wrapping_add(process_32_bits); // % 32 put this in 0 to 32 that why we need to use that funciotn
                temp_reg_a = temp_reg_a.wrapping_add(reg_32_bits.clone());
                temp_reg_a = temp_reg_a.wrapping_add(CONSTANT);
                temp_reg_a = temp_reg_a.rotate_left(SHIFT);

                let temp_b = REG_B;
                let temp_c = REG_C;
                let temp_d = REG_D;

                REG_A = temp_d;
                REG_B = temp_reg_a;
                REG_C = temp_b;
                REG_D = temp_c;
            }
        }
    }

    let hash_bits = vec![REG_A, REG_B, REG_C, REG_D];

    let bytes: Vec<u8> = hash_bits
        .into_iter()
        .flat_map(|word| word.to_be_bytes())
        .collect();

    let hash: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();

    hash
}

fn main() {
    loop {
        println!("Select Option");
        println!("1. Hash Password");
        println!("5. Exit");
        match take_input().parse().unwrap() {
            1 => {
                println!("Enter Your Password");
                let password = take_input();
                let hash = create_hash(password);
                println!("The hash of the password is: {}", hash);
            }
            5 => {
                println!("THANK YOU!");
                return;
            }
            _ => {
                println!("Wrong Input");
            }
        }
    }
}
