use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::env;

#[allow(dead_code)]
struct RandomByte {
    byte: u8,
}

#[allow(dead_code)]
impl RandomByte {
    fn new(b: u8) -> std::option::Option<RandomByte> {
        if b <= 0x20 { None }
        else { Some(RandomByte{ byte: b }) }
    }
    fn as_letter_lower(self: Self) -> char {
        let mut b = 'a' as u8;
        b = b + (self.byte % 26);
        b as char
    }
    fn as_letter_upper_or_digit(self: Self) -> char {
        let mut b: u8;
        if (self.byte % 36) < 26 {
            b = 'A' as u8;
            b = b + (self.byte % 36);
        } else {
            b = '0' as u8;
            b = b + ((self.byte % 36) - 26);
        }
        b as char
    }
}

fn is_uppercase(c: &char) -> bool {
    ['A','B','C','D','E','F','G',
     'H','I','J','K','L','M',
     'N','O','P','Q','R','S','T',
     'U','V','W','X','Y','Z'].contains(&c)
}

fn is_lowercase(c: &char) -> bool {
    ['a','b','c','d','e','f','g',
     'h','i','j','k','l','m',
     'n','o','p','q','r','s','t',
     'u','v','w','x','y','z'].contains(&c)
}

fn is_digit(c: &char) -> bool {
    ['0','1','2','3','4','5','6','7','8','9'].contains(&c)
}

fn is_symbol(c: &char) -> bool {
    ['~','!','@','#','$','%','^','&','*','(',
     ')','?','<','>','-','=','_','+',',','.',
     '?','/',';','[',']','{','}'].contains(&c)
}


// checks for >0 uppercase, >0 lowercase, >0 numbers, >0 symbols
// and starts with a letter
fn check_password(s: &str) -> bool {
    let mut b_starts_letter = false;
    let mut b_uppercase = false;
    let mut b_lowercase = false;
    let mut b_number = false;
    let mut b_symbol = false;

    let c1 = s.chars().nth(0).unwrap();
    if is_lowercase(&c1) || is_uppercase(&c1) {
        b_starts_letter = true;
    }
    for c in s.chars() {
        if is_uppercase(&c) {
            b_uppercase = true;
        }
        if is_lowercase(&c) {
            b_lowercase = true;
        }
        if is_digit(&c) {
            b_number = true;
        }
        if is_symbol(&c) {
            b_symbol = true;
        }
    }
    b_uppercase && b_lowercase && b_number && b_symbol && b_starts_letter
}

fn randbytes(len: usize) -> io::Result<String> {
    let f = fs::File::open(&Path::new("/dev/random"))?;
    let mut s = String::new();
    let excludes = vec!['l','I','|','\'','`','\\','"',':'];
    for byte in f.bytes() {
        match byte {
            Ok(byte) => {
                let mut b = byte;
                if b >= 0x7f { b = b % 0x7f; }
                if b >= 0x21 && b <= 0x7e {
                    let c = b as char;
                    if excludes.contains(&c) {
                        continue;
                    } else {
                        s.push(c);
                    }
                    if s.len() >= len { break; }
                } else {
                    //sprintln!("unusable byte {:x?}",b)
                }
            },
            Err(why) => { println!("! {:?}",why.kind()); },
        }
    }
    Ok(s)
}

#[allow(dead_code)]
#[allow(unused_variables)]
//#[allow(unused_mut)]
fn randbytes2(len: usize) -> io::Result<Vec<RandomByte>> {
    let f = fs::File::open(&Path::new("/dev/random"))?;
    let mut v = Vec::<RandomByte>::new();
    for byte in f.bytes() {
        match byte {
            Ok(byte) => {
                let rb = RandomByte::new(byte);
                match rb {
                    Some(b) => { v.push(b); },
                    None => {},
                }
                if v.len() >= len { break; }
            },
            Err(why) => { println!("Error(randbytes2): {:?}",why.kind()); },
        }
    }
    Ok(v)
}

fn rand_password(len: usize) -> String {
    match randbytes(len) {
        Err(why) => { println!("! {:?}",why.kind()); std::process::exit(1); },
        Ok(s) => s,
    }
}

fn contains_dash_h(s: &String) -> bool {
    s.contains("-h")
}

#[allow(unused_variables)]
fn main() {
    let mut entropy_bits: f32;
    let args: Vec<String> = env::args().collect();
    if (args.len() != 2) || 
        contains_dash_h(&args[1]) || 
        (&args[1].parse::<usize>().unwrap() < &1) {
        println!("Usage:  {} LEN\nwhere LEN must be > 0", &args[0]);
        std::process::exit(1);
    }
    let & len = &args[1].parse::<usize>().unwrap();

    // Emit len random characters (A-Z, a-z, 0-9, 28 symbols)
    let mut s;
    // Note: log2(27 symbols + 25 lcase (excludes l) + 25 ucase (excludes I) + 10 digits) = 6.44
    entropy_bits = 6.44 * len as f32; 
    loop {
        s = rand_password(len);
        if check_password(&s) { break; }
    }
    println!("  {} (entropy: {} bits)",s,entropy_bits);

    // Emit len random chars (A-Z, 0-9 only)
    // Note: log2(25 ucase + 10 digits) = 5.13
    entropy_bits = 5.13 * len as f32;
    let v: Vec<RandomByte>;
    print!("  ");
    match randbytes2(len) {
        Err(why) => { println!("! {:?}",why.kind()); },
        Ok(v) => {
            for b in v {
                print!("{}",b.as_letter_upper_or_digit())
            }
        },
    };
    println!(" (entropy: {} bits)",entropy_bits);

    // Emit len random chars (A-Z, 0-9) broken up by dashes
    let group_len = 5; // arbitrary choice
    if len >= group_len {
        let v: Vec<RandomByte>;
        let len_reduced = len - (len % group_len);
        entropy_bits = 5.17 * len_reduced as f32;
        print!("  ");
        match randbytes2(len_reduced) {
            Err(why) => { println!("! {:?}",why.kind()); },
            Ok(v) => {
                let mut cntr = 0;
                for b in v {
                    if (cntr>0) && (cntr % 5 == 0) {
                        print!("-")
                    }
                    print!("{}",b.as_letter_upper_or_digit());
                    cntr=cntr+1;
                }
            },
        };
        println!(" (entropy: {} bits)",entropy_bits);
    }
}
