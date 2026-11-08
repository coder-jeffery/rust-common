// ownership

fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 100;

    makes_copy(x);

    let s2 = String::from("to be or not to be, this is a question");

    let (s3, len) = calculate_length(s2);

    println!("The length of '{s3}' is {len}.");

    let s5 = String::from("citibank");
    let len = calculate_length_brow(&s5);

    println!("The length of '{s5}' is {len}.");

    testcase_str();
    testcase_str2();
    let refreence_to_nothing = dangle();
    println!("{}", refreence_to_nothing);
}

fn takes_ownership(some_thing: String) {
    println!("{some_thing}");
}

fn makes_copy(number: i32) {
    println!("{number}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_brow(s: &String) -> usize {
    s.len()
}

fn testcase_str() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 =  &mut s;
    // println!("r1 =  {r1}  r2 =  {r2}");
    // {
    //     let r2 =  &mut s;
    //     println!("r2 =  {r2}");
    // }
    println!("r1 =  {r1}  ");
}

fn testcase_str2() {
    let mut s = String::from("hello");
    s.push_str(", world");

    let r1 = &s;
    let r2 = &s;
    println!("value is {s} r1 value is {r1}, r2 value is {r2}");

    let r3 = &mut s;
    println!(" r3 value is {r3}");
}

// 使用引用（&）借用值（不转移所有权）

fn dangle() -> String {
    let s = String::from("hello");
    s
}

//切片类型  Slice Type
