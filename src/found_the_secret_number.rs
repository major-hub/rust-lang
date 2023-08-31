use std::cmp::Ordering;
use std::io;
use rand::Rng;

#[warn(dead_code)]
pub fn found() {
    println!("Raqamni topish o'yini!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Yashirin raqam: {secret_number}");


    loop {
        let mut my_number = String::new();
        println!("Iltimos, taxminingizni kiriting.");

        io::stdin()
            .read_line(&mut my_number)
            .expect("Satrni o‘qib bo‘lmadi");

        println!("Sizning taxminingiz: {my_number}");

        let my_number: i32 = match my_number.trim().parse() {
            Ok(son) => son,
            Err(_) => continue,
        };

        match my_number.cmp(&secret_number) {
            Ordering::Greater => println!("Kattaroq"),
            Ordering::Less => println!("Kichikroq"),
            Ordering::Equal => {
                println!("Teng!");
                break;
            }
        }
    }
}