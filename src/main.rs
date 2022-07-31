use std::io;
fn main() {
    let mut expoents = String::new();

    io::stdin()
        .read_line(&mut expoents)
        .expect("Error in function");

    let expoents: Vec<&str> = expoents.trim().split(" ").collect();

    let expoents: Vec<i32> = {
        expoents.into_iter().map(|x| x.parse().unwrap()).collect()
    };

    println!("{:?}", expoents);

    for expoent in expoents {
        println!("{}", expoent);
    }
}
