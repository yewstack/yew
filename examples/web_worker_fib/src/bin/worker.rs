use yew_agent::task::Task;
use yew_worker_fib::agent::FibonacciAgent;

fn main() {
    FibonacciAgent::registrar().register();
}
