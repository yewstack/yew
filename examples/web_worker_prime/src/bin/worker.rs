use yew_agent::Registrable;
use yew_worker_prime::agent::Prime;

fn main() {
    Prime::registrar().register();
}
