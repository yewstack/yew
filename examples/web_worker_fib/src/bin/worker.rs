use yew_agent::Registrable;
use yew_worker_fib::agent::FibonacciTask;

fn main() {
    FibonacciTask::registrar().register();
}
