fn basic(){
    let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("{}",s1); desta forma o programa não vai compilar porque estou a imprimir um valor desatualizado
    let s2 = s1.clone(); // assim mantemos os dois valores
    println("s1 = {} , s2 = {}",s1,s2);

    let x = 5;
    let y = x; // no caso de integers podemos fazer desta forma porque normalmente eles têm um tamanho conhecido no tempo de compilação

    println!("x = {}, y = {}", x, y);
}

/* Here are some of the types that implement Copy:

    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
*/

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}