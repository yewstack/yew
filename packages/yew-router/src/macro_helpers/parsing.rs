use std::borrow::Cow;

use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};

use super::{Arg, Args};

pub fn remove_segment<'a>(path: &mut &'a str) -> Option<&'a str> {
    *path = &path[1..];
    let i = path.find('/').unwrap_or_else(|| path.len());
    if i > 0 {
        let res = &path[0..i];
        if i < path.len() {
            *path = &path[i..];
        } else {
            *path = "/"
        }
        Some(res)
    } else {
        None
    }
}

pub fn extract_named_args(prefix: &str, names: &[&str], value: &mut String, args: &mut Args) {
    if names.is_empty() {
        return;
    }
    let mut arr = vec![Arg::Empty; names.len()];
    if let Some(remainder) = value.strip_prefix(prefix) {
        let mut unused_parts = Vec::new();
        for part in remainder.split('&') {
            let mut it = part.splitn(2, '=');
            if let (Some(lhs), Some(rhs)) = (it.next(), it.next()) {
                if let Some(i) = names.iter().position(|&name| name == lhs) {
                    let rhs = rhs.replace('+', " ");
                    let rhs = percent_decode_str(rhs.as_str()).decode_utf8_lossy();
                    arr[i] = rhs.as_ref().into();
                    continue;
                }
            }
            unused_parts.push(part);
        }
        if unused_parts.is_empty() {
            value.clear();
        } else {
            *value = format!("{}{}", prefix, unused_parts.join("&"));
        }
    }
    for item in arr {
        args.push(item);
    }
}

pub fn insert_named_args(prefix: &str, names: &[&str], value: &mut String, args: &mut Args) {
    if names.is_empty() {
        return;
    }
    let existing = value.strip_prefix(prefix).unwrap_or(&value);

    let mut rev_arg_acc = if existing.is_empty() {
        vec![]
    } else {
        vec![Cow::Borrowed(existing)]
    };
    for &name in names.iter().rev() {
        if let Some(value) = args.pop::<String>() {
            if !rev_arg_acc.is_empty() {
                rev_arg_acc.push("&".into());
            }

            let rhs: Cow<str> = percent_encode(value.as_bytes(), NON_ALPHANUMERIC).into();
            rev_arg_acc.push(rhs.replace("%20", "+").into());
            rev_arg_acc.push("=".into());
            rev_arg_acc.push(name.into());
        }
    }
    if !rev_arg_acc.is_empty() {
        rev_arg_acc.push(prefix.into());
    }
    rev_arg_acc.reverse();
    *value = rev_arg_acc.concat();
}
