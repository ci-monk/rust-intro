use std::mem;

fn main() {
    let tuple_a = (1, 2, 'a');
    println!("{:?}", tuple_a);

    let tuple_b = (21.2, tuple_a);
    println!("{:?}", tuple_b);
    println!("{:#?}", tuple_b.1);

    let other = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("{:?}", other);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", xs);
    println!("{}", xs[4]);
    println!("{}", xs.len());
    // mem::size_of_val(&xs) -> how big this is array is inside memory -> 20 bytes in size
    println!("{} {} {}", xs[1], xs.len(), mem::size_of_val(&xs));

    let ys = &xs[2..4];
    println!("{:?}", ys);
    println!("{}", ys[1]);

    // Slice of string
    let s = "String".to_string();
    // Strings aren't a literal type in rust.
    // Strings are more like arrays or tuples in the sense that they are compound types.
    let ss = String::from("String");
    println!("{}", s);
    println!("{}", ss);

    let slice = &ss[0..4];
    println!("{}", slice);

    let abc = String::from("abc - ");
    let def = String::from("def");
    let concat = abc + &def;
    println!("{}", concat);

    // Empty functions in rust return empty tuples
}
