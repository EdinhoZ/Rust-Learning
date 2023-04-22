/* fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
} 
Aqui usamos uma função auxiliar para imprimir uma segunda linha */

fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
// Aqui usamos uma função auxiliar e um parâmetro específico