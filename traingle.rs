fn main(){
    let mut val = String::new();
    std::io::stdin().read_line (&mut val).expect("Failed to read line");
    let mut substr_iter = val.split_whitespace();
    let mut next_num = || -> usize {
        substr_iter.next().expect("Not enough input numbers")
                   .parse().expect("Input is not a number")
    };
    let val1 = next_num();
    let val2 = next_num();
    let val3 = next_num();

    
    if val1 == val2 && val2 == val3{
        println!("Triângulo equilátero");
    } else if val1 == val2 || val2 == val3 || val1 == val3{
        println!("Triângulo isóceles");
    } else {
        println!("Triângulo escaleno");
    }
}