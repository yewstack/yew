use rand::{Rng, RngCore};
use yew::prelude::*;

/// The formula used to approximate the value of pi is the Monte Carlo method for random points
/// inside a circle. This method involves generating a random set of points on a square enclosing
/// a circle, and then using the ratio of the number of points inside the circle to the total number
/// of points generated to estimate the value of pi. Specifically, the formula used is:
///
/// * pi ≈ 4 * Ncircle / Npoints
///
/// where Ncircle is the number of points generated that fall inside the circle, and Npoints is the
/// total number of points generated.
///
/// # Arguments
/// * rng (&mut RngCore): a reference to the random number generator function.
/// * n_points (u64): the total number of points generated.
/// * n_circle (&mut u64): a reference to the number of points generated that fall inside the
///   circle.
///
/// # Returns
/// (f64): A floating point number that represents a rough approximation for pi.
/// or throw an error.
fn compute_monte_carlo<T>(rng: &mut T, n_points: u64, n_circle: &mut u64) -> f64
where
    T: RngCore,
{
    for _ in 0..n_points {
        let x = rng.gen_range(-0.5..0.5);
        let y = rng.gen_range(-0.5..0.5);

        if x * x + y * y <= 0.25 {
            *n_circle += 1;
        }
    }
    4.0 * (*n_circle as f64) / (n_points as f64)
}

#[function_component(ModelApp)]
pub fn model_app() -> Html {
    let pi_approximation = use_state(|| 0.0);
    let calculate_pi = {
        let mut rng = rand::thread_rng();
        let n_points = 10_000; // Number of random points to generate
        let mut n_circle = 0;
        let pi = compute_monte_carlo(&mut rng, n_points, &mut n_circle);
        let pi_approximation = pi_approximation.clone();
        Callback::from(move |_: MouseEvent| pi_approximation.set(pi))
    };

    html! {
        <div class="container">
          <h1 class="title">{"Approximation of Pi using Monte Carlo method."}</h1>
          <p class="subtitle">{"Click the button to calculate Pi using Monte Carlo method"}</p>
          <button class="button is-primary" onclick={calculate_pi.clone()}>{"Calculate Pi"}</button>
          <div class="result" style="font-size: 5rem">{format!("{:.5}", *pi_approximation)}</div>
        </div>
    }
}

fn main() {
    yew::Renderer::<ModelApp>::new().render();
}
