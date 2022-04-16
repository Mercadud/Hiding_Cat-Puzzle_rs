use std::io;

use rand;
use rand::Rng;

fn main() {
    let box_low = 1;
    let box_high = {
        let mut found_num = false;
        let mut e = -1;
        while !found_num {
            e = read_i32(String::from("Enter amount of boxes:"));
            if e >= 2 {
                found_num = true;
            } else {
                println!("there can't be less than 2 boxes!");
            }
        }
        e
    };
    let mut is_found = false;
    let mut rng = rand::thread_rng();
    let mut cat_location = rng.gen_range(1..(box_high + 1));
    while !is_found {
        let input = {
            let mut found_num = false;
            let mut e = -1;
            while !found_num {
                e = read_i32(String::from("Enter where you think the cat is:"));
                if e >= box_low && e <= box_high {
                    found_num = true;
                } else {
                    println!("Reminder that the lowest box is: {} and the highest box is: {}", box_low, box_high);
                }
            }
            e
        };
        if input == cat_location {
            is_found = true;
            println!("You found the cat!");
        }
        if cat_location == box_high {
            cat_location -= 1;
        } else if cat_location == box_low {
            cat_location += 1;
        } else {
            let up_down: bool = rng.gen_bool(0.5);
            if up_down {
                cat_location += 1;
            } else {
                cat_location -= 1;
            }
        }
    }
}

fn read_i32(message: String) -> i32 {
    loop {
        let mut entered = String::new();
        println!("{}", message);
        match io::stdin().read_line(&mut entered) {
            Ok(e) => {
                let mut is_numeric = true;
                let entered = entered.trim();
                for c in entered.clone().chars() {
                    if !c.is_numeric() {
                        is_numeric = false;
                    }
                }
                if entered.is_empty() {
                    is_numeric = false;
                }
                if is_numeric {
                    return entered.parse().unwrap();
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}