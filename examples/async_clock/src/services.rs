use std::time::Duration;

use chrono::{DateTime, Local};
use futures::{Stream, StreamExt};
use gloo_net::http::Request;
use yew::platform::pinned::mpsc::UnboundedSender;
use yew::platform::spawn_local;
use yew::platform::time::{interval, sleep};
use yew::{AttrValue, Callback};

const ONE_SEC: Duration = Duration::from_secs(1);
const TEN_SECS: Duration = Duration::from_secs(10);

/// Demonstration code to show how to use async code in a yew component.
pub async fn initialize_atomic_clocks() {
    // aligning with atomic clocks :-)
    sleep(ONE_SEC).await;
}

/// Returns a stream of time updates.
pub fn stream_time() -> impl Stream<Item = DateTime<Local>> {
    interval(ONE_SEC).map(|_| Local::now())
}

/// Emit entertaining jokes every 10 seconds.
pub fn emit_jokes(joke_cb: Callback<AttrValue>) {
    // Spawn a background task that will fetch a joke and send it to the component.
    spawn_local(async move {
        loop {
            // Fetch the online joke
            let fun_fact = Request::get("https://v2.jokeapi.dev/joke/Programming?format=txt")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            // Emit it to the component
            joke_cb.emit(AttrValue::from(fun_fact));
            sleep(TEN_SECS).await;
        }
    });
}

/// Background task that computes the fun score from jokes that are delivered on the channel.
pub fn compute_fun_score(fun_score_cb: Callback<i16>) -> UnboundedSender<AttrValue> {
    let (tx, mut rx) = yew::platform::pinned::mpsc::unbounded::<AttrValue>();

    // Read endlessly from the UnboundedReceiver and compute the fun score.
    spawn_local(async move {
        while let Some(joke) = rx.next().await {
            sleep(ONE_SEC).await;
            let score = joke.len() as i16;
            fun_score_cb.emit(score);
        }
    });

    tx
}
