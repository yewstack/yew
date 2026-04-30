#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

fn main() {
    _ = ::yew::html!{
        for i in 0 .. 10 {
            <span>{i}</span>
        }
    };

    struct Pair {
        value1: &'static ::std::primitive::str,
        value2: ::std::primitive::i32
    }

    _ = ::yew::html! {
        for Pair { value1, value2 } in ::std::iter::Iterator::map(0 .. 10, |value2| Pair { value1: "Yew", value2 }) {
            <span>{value1}</span>
            <span>{value2}</span>
        }
    };

    fn rand_number() -> ::std::primitive::u32 {
        4 // chosen by fair dice roll. guaranteed to be random.
    }

    _ = ::yew::html!{
        for _ in 0..5 {
            <div>
                {{
                    loop {
                        let a = rand_number();
                        if a % 2 == 0 {
                            break a;
                        }
                    }
                }}
            </div>
        }
    };

    // Single let binding in for body
    _ = ::yew::html!{
        for i in 0 .. 10 {
            let doubled = i * 2;
            <span>{doubled}</span>
        }
    };

    // Multiple let bindings
    _ = ::yew::html!{
        for i in 0 .. 10 {
            let doubled = i * 2;
            let label = "item";
            <span>{label}</span>
            <span>{doubled}</span>
        }
    };

    // Let with pattern destructuring
    _ = ::yew::html!{
        for (a, b) in ::std::iter::Iterator::map(0..5, |i| (i, i * 2)) {
            let sum = a + b;
            <span>{sum}</span>
        }
    };

    // Let with type annotation
    _ = ::yew::html!{
        for i in 0..10 {
            let x: ::std::primitive::i32 = i * 3;
            <span>{x}</span>
        }
    };

    // Let binding with multiple bare children
    _ = ::yew::html!{
        for i in 0..5 {
            let doubled = i * 2;
            let label = "val";
            <span>{label}</span>
            <span>{doubled}</span>
            <hr/>
        }
    };

    // break in for loop
    _ = ::yew::html!{
        for i in 0..10 {
            if i > 5 {
                break
            }
            <span>{i}</span>
        }
    };

    // continue in for loop
    _ = ::yew::html!{
        for i in 0..10 {
            if i % 2 == 0 {
                continue
            }
            <span>{i}</span>
        }
    };

    // break with trailing semicolon
    _ = ::yew::html!{
        for i in 0..10 {
            if i > 5 {
                break;
            }
            <span>{i}</span>
        }
    };

    // continue with trailing semicolon
    _ = ::yew::html!{
        for i in 0..10 {
            if i % 2 == 0 {
                continue;
            }
            <span>{i}</span>
        }
    };

    // unbraced match arm with break
    _ = ::yew::html!{
        for i in 0..10 {
            match i {
                0 => break,
                _ => <span>{i}</span>,
            }
        }
    };

    // unbraced match arm with continue
    _ = ::yew::html!{
        for i in 0..10 {
            match i {
                0 => continue,
                _ => <span>{i}</span>,
            }
        }
    };

    // braced match arm with break
    _ = ::yew::html!{
        for i in 0..10 {
            match i {
                0 => { break },
                _ => <span>{i}</span>,
            }
        }
    };

    // break/continue in a for body must not emit `unreachable_code` warnings even
    // under `#[deny(unreachable_code)]`.
    #[deny(unreachable_code)]
    fn break_continue_no_warn() {
        _ = ::yew::html!{
            for i in 0..10 {
                if i > 5 {
                    break;
                }
                if i % 2 == 0 {
                    continue;
                }
                <span>{i}</span>
            }
        };
    }
    break_continue_no_warn();

    // Expression statement in loop body preamble (`.method(..);` for side effects).
    _ = ::yew::html!{
        for i in 0..5 {
            let counter = ::std::cell::Cell::new(i);
            counter.set(i * 2);
            <span>{counter.get()}</span>
        }
    };

    // Compound-assignment expression statement in loop body preamble.
    {
        let mut total: ::std::primitive::i32 = 0;
        _ = ::yew::html!{
            for i in 0..5 {
                let current = i;
                total += current;
                <span>{current}</span>
            }
        };
        _ = total;
    }

    // Local fn item in loop body preamble.
    _ = ::yew::html!{
        for i in 0..5 {
            fn double(x: ::std::primitive::i32) -> ::std::primitive::i32 { x * 2 }
            let v = double(i);
            <span>{v}</span>
        }
    };

    // Macro statement in loop body preamble (with `;`).
    _ = ::yew::html!{
        for _i in 0..1 {
            ::std::stringify!(debug_marker);
            <span>{"ok"}</span>
        }
    };

    // Mixed interleaving: let / expr-stmt / let / html.
    {
        let mut acc: ::std::primitive::i32 = 0;
        _ = ::yew::html!{
            for i in 0..3 {
                let x = i;
                acc += x;
                let y = x + 1;
                <span>{y}</span>
            }
        };
        _ = acc;
    }

    // Labeled `break` targeting an enclosing labeled loop in user code.
    {
        let mut outer_hit: ::std::primitive::i32 = 0;
        'outer: for _ in 0..3 {
            _ = ::yew::html!{
                for i in 0..10 {
                    if i > 2 {
                        break 'outer;
                    }
                    <span>{i}</span>
                }
            };
            outer_hit += 1;
        }
        _ = outer_hit;
    }

    // Labeled `continue` targeting an enclosing labeled loop in user code.
    {
        let mut outer_hit: ::std::primitive::i32 = 0;
        'outer: for _ in 0..3 {
            _ = ::yew::html!{
                for i in 0..10 {
                    if i > 2 {
                        continue 'outer;
                    }
                    <span>{i}</span>
                }
            };
            outer_hit += 1;
        }
        _ = outer_hit;
    }

    // Labeled `break 'outer` without a trailing `;` (body-top position).
    {
        let mut outer_hit: ::std::primitive::i32 = 0;
        'outer: for _ in 0..3 {
            _ = ::yew::html!{
                for i in 0..10 {
                    if i > 2 {
                        break 'outer
                    }
                    <span>{i}</span>
                }
            };
            outer_hit += 1;
        }
        _ = outer_hit;
    }

    // Labeled `continue 'outer` without a trailing `;` (body-top position).
    {
        let mut outer_hit: ::std::primitive::i32 = 0;
        'outer: for _ in 0..3 {
            _ = ::yew::html!{
                for i in 0..10 {
                    if i > 2 {
                        continue 'outer
                    }
                    <span>{i}</span>
                }
            };
            outer_hit += 1;
        }
        _ = outer_hit;
    }

    // Bare `return` (no value, no trailing `;`) at body top. Returns `()` from main.
    fn bare_return_at_body_top() {
        _ = ::yew::html!{
            for _ in 0..1 {
                return
                <span>{"unreachable"}</span>
            }
        };
    }
    bare_return_at_body_top();

    // Bare `return` in unbraced match arm.
    fn return_in_unbraced_match_arm() {
        _ = ::yew::html!{
            for i in 0..10 {
                match i {
                    3 => return,
                    _ => <span>{i}</span>,
                }
            }
        };
    }
    return_in_unbraced_match_arm();

    // Bare `return` in braced match arm.
    fn return_in_braced_match_arm() {
        _ = ::yew::html!{
            for i in 0..10 {
                match i {
                    3 => { return },
                    _ => <span>{i}</span>,
                }
            }
        };
    }
    return_in_braced_match_arm();

    // `return` with a value in preamble position (statement form with `;`).
    fn return_value_from_preamble() -> ::std::primitive::i32 {
        _ = ::yew::html!{
            for i in 0..3 {
                return i;
                <span>{i}</span>
            }
        };
        0
    }
    let _ = return_value_from_preamble();

    // `return` with a value in unbraced match arm (value part of ExprReturn).
    fn return_value_from_unbraced_arm() -> ::std::primitive::i32 {
        _ = ::yew::html!{
            for i in 0..10 {
                match i {
                    3 => return i,
                    _ => <span>{i}</span>,
                }
            }
        };
        0
    }
    let _ = return_value_from_unbraced_arm();

    // Imperative block-like preambles (`for`, `while`, `loop`, `{...}`) with a
    // trailing `;` parse as `Stmt::Expr(_, Some(_))` and are accepted as
    // preamble. The bare form (no `;`) is auto-detected by the preamble
    // parser; these test that the explicit `;` form keeps working too.
    {
        let mut sink: ::std::vec::Vec<::std::primitive::i32> = ::std::vec::Vec::new();
        _ = ::yew::html!{
            for x in 0..3_i32 {
                let mut acc: ::std::primitive::i32 = 0;
                for i in 0..x {
                    acc += i;
                };
                sink.push(acc);
                <span>{acc}</span>
            }
        };
        _ = sink;
    }
    {
        let mut sink: ::std::vec::Vec<::std::primitive::i32> = ::std::vec::Vec::new();
        _ = ::yew::html!{
            for _x in 0..3_i32 {
                let mut counter: ::std::primitive::i32 = 0;
                while counter < 3 {
                    counter += 1;
                };
                sink.push(counter);
                <span>{counter}</span>
            }
        };
        _ = sink;
    }
    {
        let mut sink: ::std::vec::Vec<::std::primitive::i32> = ::std::vec::Vec::new();
        _ = ::yew::html!{
            for _x in 0..2_i32 {
                let mut n: ::std::primitive::i32 = 0;
                loop {
                    n += 1;
                    if n > 2 { break; }
                };
                sink.push(n);
                <span>{n}</span>
            }
        };
        _ = sink;
    }
    {
        let mut sink: ::std::vec::Vec<::std::primitive::i32> = ::std::vec::Vec::new();
        _ = ::yew::html!{
            for x in 0..3_i32 {
                {
                    sink.push(x * 10);
                };
                <span>{x}</span>
            }
        };
        _ = sink;
    }

    // Same pattern in a braced match arm preamble.
    _ = ::yew::html!{
        match 1_i32 {
            0 => <span>{"zero"}</span>,
            _ => {
                let mut acc: ::std::primitive::i32 = 0;
                for i in 0..3_i32 {
                    acc += i;
                };
                <span>{acc}</span>
            }
        }
    };

    // Same pattern in a `while` body preamble.
    {
        let mut counter: ::std::primitive::i32 = 0;
        _ = ::yew::html!{
            while counter < 2 {
                let mut acc: ::std::primitive::i32 = 0;
                for i in 0..counter {
                    acc += i;
                };
                counter += 1;
                <span>{acc}</span>
            }
        };
        _ = counter;
    }

    // The `let _ = ...;` form also works, for users who prefer it.
    {
        let mut sink: ::std::vec::Vec<::std::primitive::i32> = ::std::vec::Vec::new();
        _ = ::yew::html!{
            for x in 0..3_i32 {
                let mut acc: ::std::primitive::i32 = 0;
                let _ = for i in 0..x {
                    acc += i;
                };
                sink.push(acc);
                <span>{acc}</span>
            }
        };
        _ = sink;
    }
}
