fn main() {
    println!("Hello, world!");
    let mut x = 10;
    println!("{}", x);
    x = 5;
    println!("{}", x);
    let y: u32 = 21;
    println!("{}", y);

    // i8, u8, i16, u16, i32, u32, i64, u64 -> bits in memory
    // isuze, usize -> depends your arch
    // f32, f64
    let a = 1 + 2;
    println!("{}", a);

    let b = 123 - 4322;
    println!("{}", b);

    let c = 320 * 999;
    println!("{}", c);

    let d = 4/6;
    println!("{}", d);

    let e = 43 % 2;
    println!("{}", e);

    // bool
    let f = true;
    println!("{}", f);

    let g = false;
    println!("{}", g);

    // char
    let h = 'c';
    println!("{}", h);

    let i: char = 'z';
    println!("{}", i);

    // tuples
    let j: (i32, f32, char) = (1, 23.332, 'f');
    println!("{:?}", j);
    let (ab, _, ef) = j;
    println!("{}", ab);
    println!("{}", ef);
    println!("{}", j.0);

    // arrays
    let array = [1, 2, 3];
    println!("{:?}", array);
    println!("{}", array[2]);
}
