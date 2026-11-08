use rand::Rng;
use std::cmp::Ordering;
use std::io; //prelude 

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("神秘数字：{} ", secret_number);

    println!("猜测一个数字");

    // let mut foo = 1; // mut 可变变量
    // let  bar = foo;
    // foo =2;

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取");

    println!("你猜测的数字是：{} ", guess);

    println!("testing .......");

    let guess: u32 = guess.trim().parse().expect("please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => {
            println!("you win");
        }
    }
}
