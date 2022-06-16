use yew_agent::task;

#[task(FibonacciTask)]
pub async fn calculate_fibonacci(n: u32) -> u32 {
    fn fib(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    fib(n)
}
