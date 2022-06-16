use yew_agent::task::Task;
use yew_worker_fib::agent::FibonacciTask;

fn main() {
    FibonacciTask::registrar().register();
}
