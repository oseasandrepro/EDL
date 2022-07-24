fn main() {
    let x = 0;
    
    {
        let x = 1;
        println!("dentro, x: {}", x);
    }
    
    println!("fora, x: {}", x);
    
    let x = "Rust shadowing";
    println!("fora, x: {}", x);
}
