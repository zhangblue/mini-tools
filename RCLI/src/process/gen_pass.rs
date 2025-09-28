use rand::Rng;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;
use crate::opts::gen_pass::GenPassOpts;

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";

pub fn process_gen_pass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::<u8>::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPERCASE);
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWERCASE);
    }

    if opts.numbers {
        chars.extend_from_slice(NUMBERS);
    }

    if opts.symbols {
        chars.extend_from_slice(SYMBOLS);
    }

    chars.shuffle(&mut rng);

    for _ in 0..opts.length {
        let idx = rng.random_range(0..chars.len());
        password.push(chars[idx] as char);
    }

    password.shuffle(&mut rng);

    let string_pass = String::from_iter(password);

    // 密码强度估计
    let estimate = zxcvbn(&string_pass, &[]);

    println!("当前密码：{}  强度: {}", string_pass, estimate.score());

    Ok(())
}
