fn variables() {
    let mut x = 5;
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é: {x}");

    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}