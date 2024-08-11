fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("There is {} Seconds in three hours", THREE_HOURS_IN_SECONDS);

    let y = 10;

    let y = y + 12;

    {
        let y = y * 23;
        println!("The value of y in the inner scope is: {y}");
    }


    print!("The value of y is: {y}");
    
    let spaces = "string";
    let spaces = spaces.len();

    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{}", guess);
}
