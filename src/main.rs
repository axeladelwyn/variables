fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("There is {} Seconds in three hours", THREE_HOURS_IN_SECONDS);

    let y = 1_000;

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

    let back_drop = 2.123;
    
    let asep_kargo: f32 = 3.12321; 

    println!("floating points f64 {}, and floating point f32 {}", back_drop, asep_kargo);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("sum is {}, difference is {}, product is {}, quotient is {}, truncated is {}, remainder is {}", sum, difference, product, quotient, truncated, remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation.

    println!("{},{}", t, f);

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation

    println!("Char symbol {}, {}", c, z);
    let heart_eyed_cat = '😻' ;

    println!("Today emoji is {} ", heart_eyed_cat);




}
