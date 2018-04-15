extern crate yew;
extern crate todomvc;

use yew::prelude::*;
use yew::services::storage::{StorageService, Area};
use todomvc::Model;

struct Context {
    storage: StorageService,
}

impl AsMut<StorageService> for Context {
    fn as_mut(&mut self) -> &mut StorageService {
        &mut self.storage
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        storage: StorageService::new(Area::Local),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}

