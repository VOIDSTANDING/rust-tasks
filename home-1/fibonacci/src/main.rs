fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("USAGE: {} <n-th_fibonacci_number>", args[0]);
        return;
    }

    let n: u128 = match args[1].trim().parse() {
        Ok(num) => num,
        _ => {
            println!("Negative is not allowed.");
            return;
        }
    };

    println!("The {}-th Fibonacci number is: {}", n, fib(n));
}


fn fib(n: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    
    a
}