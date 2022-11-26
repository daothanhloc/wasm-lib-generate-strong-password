extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate_password(length: u8,
    is_lowercase: bool,
    is_uppercase: bool,
    is_numeric: bool,
    is_symbol: bool) -> String {
    let lowercase: &str = "abcdefghijklmnopqrstuvwxyz";
    let uppercase: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numeric: &str = "0123456789";
    let symbol: &str = "!@#$%^&*()_+-=[]{};':,./<>?";

    let mut temp: String = String::new();

    if is_lowercase {
        temp = temp + lowercase;
    }
    if is_uppercase {
        temp = temp + uppercase;
    }
    if is_numeric {
        temp = temp + numeric;
    }
    if is_symbol {
        temp = temp + symbol;
    }

    print!("{}", temp);

    let mut rng = rand::thread_rng();
    let mut password = String::new();
    for _ in 0..length {
        let random_num: f64 = rng.gen();
        let random_num: usize = (random_num * temp.len() as f64) as usize; 
        password.push(temp.chars().nth(random_num).unwrap());
    }
    password
}