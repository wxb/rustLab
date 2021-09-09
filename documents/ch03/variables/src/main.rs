fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let tup: (u32, f32, i8) = (500, 6.4, 1);
    let (_, b, _) = tup;
    println!("{}, {}", tup.0, b);

    // let arr: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let arr = [1, 2, 3, 4, 5, 6];
    println!("arr is: {}", arr[0]);
}
