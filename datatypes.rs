fn datatypes(){
    let x: u32 = 3; // int
    let y: f32 = 3.0; // float
    let z: bool = true; //bool
    let w: char = 'z'; //char

    let tup: (i32, f64, u8) = (500, 6.4, 1); //tuple
    let (x, y, z) = tup; // para selecionar um valor do tuple podemos usar pattern matching para não dar erro
    
    let x: (i32, f64, u8) = (500, 6.4, 1);//tambem podemos acessar uma variável desta forma

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"]; //array

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // é o mesmo que let a = [3, 3, 3, 3, 3]; repete o valor da esquerda o número de vezes do número da direita

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("O primeiro valor do array é {first} e o segundo é {second}");
}

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}