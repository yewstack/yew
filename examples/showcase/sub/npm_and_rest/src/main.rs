extern crate yew;
extern crate npm_and_rest;

use yew::prelude::*;
use npm_and_rest::Model;
use npm_and_rest::gravatar::GravatarService;
use npm_and_rest::ccxt::CcxtService;

struct Context {
    gravatar: GravatarService,
    ccxt: CcxtService,
}

impl AsMut<GravatarService> for Context {
    fn as_mut(&mut self) -> &mut GravatarService {
        &mut self.gravatar
    }
}

impl AsMut<CcxtService> for Context {
    fn as_mut(&mut self) -> &mut CcxtService {
        &mut self.ccxt
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        gravatar: GravatarService::new(),
        ccxt: CcxtService::new(),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
