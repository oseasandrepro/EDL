fn main()
{
    let x = fib(9);
    println!("{}",x);
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }else{
        return fib(n-1) + fib(n-2);
    }
}