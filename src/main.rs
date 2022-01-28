use std::collections::HashMap;

fn fibonacci(n: u128) -> u128 {
    if n == 0 || n == 1 {
        return 1
    }

    fibonacci(n-1) + fibonacci(n-2)
}

fn mem_fibonacci(n: u128, mem: &mut HashMap<u128, u128>) -> u128 {
    if !mem.contains_key(&n) {
        if n == 0 || n == 1 {
            mem.insert(n, 1);
            return mem[&n];
        }
        let lhs = mem_fibonacci(n-1, mem);
        let rhs = mem_fibonacci(n-2, mem);
        mem.insert(n, lhs+rhs);
    }

    mem[&n]
}

fn main() {
    let mut mem = HashMap::new();
    for i in 0..=150 {
        println!("{}: {}", i, mem_fibonacci(i, &mut mem));
    }
}
