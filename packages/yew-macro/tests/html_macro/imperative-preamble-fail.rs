// Tests for the diagnostic emitted when a block-like Rust expression
// (`for`, `while`, `loop`, `{...}`) appears in preamble position. These
// expressions auto-terminate as statements per Rust grammar, so a trailing
// `;` is not folded into the `Stmt`. The preamble parser rejects them
// (matching `Stmt::Expr(_, None)`) and they would otherwise fall through
// to the html-control-flow parser, where the body's `()` value fails to
// convert to `VNode`. The diagnostic surfaces the pitfall at the right
// span and points at the `let _ = ...;` workaround.

fn main() {
    // Imperative `for` in a `for` body preamble.
    _ = ::yew::html! {
        for x in 0..3_u32 {
            let mut acc: u32 = 0;
            for i in 0..x {
                acc += i;
            }
            <span>{acc}</span>
        }
    };

    // Imperative `while` in a `for` body preamble.
    _ = ::yew::html! {
        for _x in 0..3_u32 {
            let mut counter: u32 = 0;
            while counter < 5 {
                counter += 1;
            }
            <span>{counter}</span>
        }
    };

    // Imperative `loop` in a `for` body preamble.
    _ = ::yew::html! {
        for _x in 0..3_u32 {
            let mut n: u32 = 0;
            loop {
                n += 1;
                if n > 3 { break; }
            }
            <span>{n}</span>
        }
    };

    // Imperative `for` in a `while` body preamble.
    _ = ::yew::html! {
        while false {
            let mut acc: u32 = 0;
            for i in 0..3_u32 {
                acc += i;
            }
            <span>{acc}</span>
        }
    };

    // Imperative `for` in a braced `match` arm preamble.
    _ = ::yew::html! {
        match 0_u32 {
            0 => {
                let mut acc: u32 = 0;
                for i in 0..3_u32 {
                    acc += i;
                }
                <span>{acc}</span>
            }
            _ => <span>{"other"}</span>,
        }
    };

}
