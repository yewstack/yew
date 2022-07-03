use yew_agent::Registrable;
use yew_worker_fib::agent::{FibonacciTask, Postcard};

fn main() {
    FibonacciTask::registrar().encoding::<Postcard>().register();
}
