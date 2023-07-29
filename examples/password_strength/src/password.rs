const PASSWORD_LEN: i32 = 17;

pub fn generate_password() -> String {
    let mut space: Vec<char> = vec![];
    for c in 'a'..='z' {
        space.push(c);
    }
    for c in 'A'..='Z' {
        space.push(c);
    }
    for c in '0'..='9' {
        space.push(c);
    }
    space.extend(
        [
            '_', ']', '[', '.', '-', '+', '=', ':', ';', '/', '?', '<', '>', '\\', '*', '&', '^',
            '%', '$', '#', '@', '!', '`', '~', ',',
        ]
        .iter(),
    );
    let mut result = "".to_string();
    for _ in 0..PASSWORD_LEN {
        let i = (js_sys::Math::random() * space.len() as f64).floor() as usize;
        result.push(space[i]);
    }
    result
}
