// let a1 = 1000;
// let r1 = String::from("hello r401");
// let r2 = r1;

// println!("{a1}", a1);




fn add(a: i32, b : i32)-> i32{
    a + b  //表达式
}






fn main() {
    println!("hello less04");

    let a = 10;

    let r1 = String::from("hello world");
    println!("{r1} {a}");

    // let mut s = String::from("hi body");
    let s = String::from("hi body");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");


    let a2 =1;   // 默认不可变
    let mut b2 = 22222; // mut：可变
    b2  = 3333333;
    const MAX: i32 = 10000; //常量 

    println!("{a2}  {b2}");

    let num: i64 = 999;
    println!("{num}");

    let mut str1 = String::from("hello");
    str1.push_str("world????");

    println!("{str1}");



    let res = add(100, 300);
    println!("{}", res);


    let num = 10;
    if num > 10{
        println!("boolean value is :  yes ");
    }else{
        println!("boolean value is :  no ");
    }

    //loop
    let mut j = 0;
    loop{
        j += 1;
        println!("j value  println is : {}", j);
        if j > 5{break;}
    }

    // let k = 10;
    // while k > 0{

    //     println!("k value is  println: {}", k);
    //     k -= 1;
    // }

    println!("***********************************");

    for n in 1..5{
        println!("print n value is :{}", n);
    }

    println!("***********************************");

    let v = if true { 100 } else {200};

    println!("{} value is :", v);

    println!("i8/i32/i64/i128/u8/u16");
    println!("Hello Rust!!!");
    println!("Rust 最独特：所有权（Ownership）");
}
