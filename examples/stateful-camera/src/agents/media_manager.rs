use yew::agent::{AgentLink, Store, StoreWrapper};
use wasm_bindgen::prelude::*;
use web_sys::{MediaDevices, window, console, MediaStreamConstraints};
use wasm_bindgen_futures::{JsFuture, spawn_local};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  GetStream,
  GetDevices,
}

#[derive(Debug)]
pub enum Message {
  SetStream(JsValue),
  SetStreamError(JsValue),
  SetDevices(Vec<InputDeviceInfo>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputDeviceInfo {
  device_id: DeviceId,
  group_id: String,
  kind: String,
  label: String
}

pub struct MediaManager {
  pub known_devices: Vec<InputDeviceInfo>,
  pub media_stream: Option<JsValue>,
  pub media_devices: MediaDevices,
  pub set_stream_error: Option<JsValue>,
}

impl Store for MediaManager {
  type Message = Message;
  type Input = Request;

  fn new() -> Self {
    let window = window().unwrap();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();

    MediaManager {
      known_devices: Vec::new(),
      media_stream: None,
      media_devices,
      set_stream_error: None,
    }
  }

  fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
    match msg {
      Request::GetStream => {
        console::log_1(&"Continuing handling getstream".into());
        let mut media_constraints = MediaStreamConstraints::new();
        media_constraints.audio(&JsValue::TRUE)
                         .video(&JsValue::TRUE);

        let media_promise = MediaDevices::get_user_media_with_constraints(
            &self.media_devices,
            &media_constraints).unwrap();

        spawn_local(async move {
            match JsFuture::from(media_promise).await {
              Ok(media) => link.send_message(Message::SetStream(media)),
              Err(e) => link.send_message(Message::SetStreamError(e)),
            }
        });
      }
      Request::GetDevices => {
        let devices_promise = MediaDevices::enumerate_devices(&self.media_devices).unwrap();

        spawn_local(async move {
            let devices = JsFuture::from(devices_promise).await
                            .unwrap()
                            .into_serde::<Vec<InputDeviceInfo>>()
                            .unwrap();

            link.send_message(Message::SetDevices(devices));
        });
      }
    }
  }

  fn reduce(&mut self, msg: Self::Message) {
    match msg {
      Message::SetStream(stream) => {
        self.media_stream = Some(stream);
      },
      Message::SetStreamError(error) => {
        self.set_stream_error = Some(error);
      }
      Message::SetDevices(devices) => {
        self.known_devices = devices;
      }
    }
  }
}