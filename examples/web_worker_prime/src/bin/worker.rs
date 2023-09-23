use yew_agent::Registrable;
use yew_worker_prime::agent::PrimeReactor;

fn main() {
    PrimeReactor::registrar().register();
}
