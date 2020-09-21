// This file is included in `bindings.rs`

export function getPayload() {
  return new Date().toString();
}

export function getPayloadLater(callback) {
  setTimeout(() => {
    callback(getPayload());
  }, 1000);
}
