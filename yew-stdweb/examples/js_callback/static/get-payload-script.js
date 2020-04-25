function get_payload() {
    return (new Date()).toString()
}

function get_payload_later(callback) {
    setTimeout(() => {
        callback(get_payload())
    }, 1000)
}
