extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("开始猜数游戏");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("随机数是：{}", secret_number);

    loop {
        println!("请猜一个数字");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("错误的输入!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("刚才输入的不是数字! ");
                continue;
            }
        };

        println!("你猜的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("你猜对啦!");
                break;
            }
        }
    } // end of loop

}