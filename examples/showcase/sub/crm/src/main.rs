extern crate yew;
extern crate crm;

use yew::prelude::*;
use yew::services::dialog::DialogService;
use yew::services::storage::{StorageService, Area};
use crm::Model;

struct Context {
    storage: StorageService,
    dialog: DialogService,
}

impl AsMut<StorageService> for Context {
    fn as_mut(&mut self) -> &mut StorageService {
        &mut self.storage
    }
}

impl AsMut<DialogService> for Context {
    fn as_mut(&mut self) -> &mut DialogService {
        &mut self.dialog
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        storage: StorageService::new(Area::Local),
        dialog: DialogService,
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
