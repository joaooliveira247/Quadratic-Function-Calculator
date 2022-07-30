use std::io;

fn main() {
    let mut expoents = String::new();

    io::stdin()
        .read_line(&mut expoents)
        .expect("Error in function");


    let expoents = expoents.split(" ");

    for expoent in expoents {
        println!("{}", expoent)
    }
}
