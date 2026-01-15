//this is a example and smaller version and working of diffie_hellman key exchange
use std::io;

struct Machine {
    private_key: u64,
    public_key: u64,
}

fn take_input_u64() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn mod_exp(mut base: u64, mut exp: u64, modules: u64) -> u64 {
    let mut result: u64 = 1;
    base %= modules;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modules;
        }
        base = (base * base) % modules;
        exp /= 2;
    }
    result
}

fn main() {
    let base: u64 = 3;
    let modules: u64 = 19;
    println!("Enter the machine A Private key");
    let mut temp_var = take_input_u64();
    let mut machine_a = Machine {
        private_key: temp_var,
        public_key: 0,
    };
    println!("Enter the Machine B's Private Key");
    temp_var = take_input_u64();
    let mut machine_b = Machine {
        private_key: temp_var,
        public_key: 0,
    };

    machine_a.public_key = mod_exp(base, machine_a.private_key, modules);
    machine_b.public_key = mod_exp(base, machine_b.private_key, modules);

    println!(
        "Machine A Public Key: {}\n Machine B Public Key: {}",
        machine_a.public_key, machine_b.public_key
    );

    let machine_a_symmetric_key = mod_exp(machine_b.public_key, machine_a.private_key, modules);
    let machine_b_symmetric_key = mod_exp(machine_a.public_key, machine_b.private_key, modules);

    println!("The Symmetric Key of Machine A is: {machine_a_symmetric_key}\nThe Symmetric Key of Machine B is: {machine_b_symmetric_key}");
}
//This Key exchange is amazing
//first we share the base and the modules in between machine A and Machine B which can Attacker
//access but the public key create by the each machine is also sent but that is not that much use
//full to decrypt the key attacker does not have the machine A and B private key
//Prime numbers are dope
