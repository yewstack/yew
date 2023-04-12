use stdweb::Value;
use yew::prelude::*;

use super::Message;

pub struct Context {
    pubnub: PubnubService,
}

impl AsMut<PubnubService> for Context {
    fn as_mut(&mut self) -> &mut PubnubService {
        &mut self.pubnub
    }
}

#[derive(Default, PartialEq, Properties)]
pub struct PubnubService {
    lib: Option<Value>,
    chat: Option<Value>,
}

impl PubnubService {
    pub fn new(publish_key: &str, subscribe_key: &str) -> Self {
        info!("Creating new instance of pubnub service");
        let pubnub = js! {
            let pubnub = new PubNub({
                publishKey: @{publish_key},
                subscribeKey: @{subscribe_key},
                userId: "user_" + Math.random().toString(36).substring(20, 1)
            });
            console.log("PubNub instance created!");
            return pubnub;
        };
        PubnubService {
            lib: Some(pubnub),
            chat: None,
        }
    }

    pub fn send_message(&mut self, topic: &str, msg: &str) -> () {
        let lib = self.lib.as_ref().expect("No pubnub library!");
        js! {
            let pubnub = @{lib};
            pubnub.publish({
                message: @{msg},
                channel: @{topic},
                storeInHistory: true,
                ttl: 10,
            }).then((response) => {
                console.log(response);
            }).catch((error) => {
                console.log(error);
            });
        }
    }

    pub fn connect(
        &mut self,
        topic: &str,
        nickname: &str,
        onmessage: Callback<Message>,
        onoffline: Callback<String>,
        ononline: Callback<String>,
    ) -> () {
        let lib = self.lib.as_ref().expect("No pubnub library!");

        let chat_callback = move |text: String, source: String| {
            let msg = Message { text, from: source };
            onmessage.emit(msg);
        };

        let useroffline_callback = move |username: String| {
            onoffline.emit(username);
        };

        let useronline_callback = move |username: String| {
            ononline.emit(username);
        };

        let chat = js! {
            let pubnub = @{lib};
            let nickname = @{nickname};
            let chat_callback = @{chat_callback};
            let online_cb = @{useronline_callback};
            let offline_cb = @{useroffline_callback};

            console.log("PubNub Chat Engine Ready");

            // Subscribe to a channel and add listener
            pubnub.subscribe({channels: [@{topic}]});
            pubnub.addListener({
              message: function (m) {
                if (m.publisher != nickname) {
                    let user = nickname.length > 0 ? nickname : m.publisher;
                    chat_callback(m.message, user);
                }
              }
            });
            console.log("The chat is connected!");

            pubnub.hereNow({
                    channels: [@{topic}],
                    includeState: true,
            }).then((response) => {
                let occupants = response["channels"][@{topic}]["occupants"];
                let user = occupants[occupants.length - 1];
                user = nickname.length > 0 ? nickname : user["uuid"];
                let msg = "User " + user + " is Online.";
                console.log(msg);
                online_cb(user);
            }).catch((error) => {
                console.log(error)
            });

            console.log("pubnub connecting");
            return pubnub;
        };
        self.chat = Some(chat);
    }
}
