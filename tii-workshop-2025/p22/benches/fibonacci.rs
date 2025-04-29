use p22::calc;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(args = [1, 2, 4, 8, 16, 32, 64, 100])]
fn fibonacci(n: u64) -> u64 {
    calc::fib_loop(n as u32)
}


#[divan::bench(args = [1, 2, 4, 8, 16, 32, 64, 100])]
fn fibonacci_rec(n: u64) -> u64 {
    calc::fib_rec(n as u32)
}
