mod luhn;
use luhn::luhn_implementation;
use std::env;

fn main() {
    println!(
        "----> Luhn Algorithm <----\n
           result: \n"
    );
    let user_input = env::args().nth(1).unwrap();
    let res = luhn_implementation(&user_input);

    if res % 10 == 0 {
        println!("[+] Card number {user_input} is valid");
        println!("[+] Luhn Algorithm result : {}", res % 10);
    } else {
        // None-zero values are invalid
        println!("[-] Card number {user_input} is invalid");
        println!("[-] Luhn Algorithm result : {}", res % 10);
    }
}
