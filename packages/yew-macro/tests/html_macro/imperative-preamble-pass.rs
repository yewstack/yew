// Imperative `for`, `while`, and `loop` blocks in preamble position are
// detected as Rust statements when the entire expression is Rust-parseable
// (no html elements anywhere inside). The user does not need to add a
// trailing `;` or wrap the loop in `let _ = ...;`.

fn main() {
    // Imperative `for` in a `for` body preamble.
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for x in 0..3_u32 {
                let mut acc: u32 = 0;
                for i in 0..x {
                    acc += i;
                }
                tally.push(acc);
                <span>{acc}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![0, 0, 1]);
    }

    // Imperative `while` in a `for` body preamble.
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for _x in 0..3_u32 {
                let mut counter: u32 = 0;
                while counter < 5 {
                    counter += 1;
                }
                tally.push(counter);
                <span>{counter}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![5, 5, 5]);
    }

    // Imperative `loop` in a `for` body preamble.
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for _x in 0..3_u32 {
                let mut n: u32 = 0;
                loop {
                    n += 1;
                    if n > 3 { break; }
                }
                tally.push(n);
                <span>{n}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![4, 4, 4]);
    }

    // Imperative `for` in a `while` body preamble.
    {
        let mut counter: u32 = 0;
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            while counter < 3 {
                let mut acc: u32 = 0;
                for i in 0..3_u32 {
                    acc += i;
                }
                tally.push(acc);
                counter += 1;
                <span>{acc}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![3, 3, 3]);
    }

    // Imperative `for` in a braced `match` arm preamble.
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            match 0_u32 {
                0 => {
                    let mut acc: u32 = 0;
                    for i in 0..3_u32 {
                        acc += i;
                    }
                    tally.push(acc);
                    <span>{acc}</span>
                }
                _ => <span>{"other"}</span>,
            }
        };
        ::std::assert_eq!(tally, ::std::vec![3]);
    }

    // Labeled imperative `for` (fully Rust-parseable).
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for x in 0..3_u32 {
                let mut acc: u32 = 0;
                'inner: for i in 0..10_u32 {
                    if i > x { break 'inner; }
                    acc += i;
                }
                tally.push(acc);
                <span>{acc}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![0, 1, 3]);
    }

    // Imperative `for` followed by another preamble statement, then html.
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for x in 0..3_u32 {
                let mut acc: u32 = 0;
                for i in 0..x {
                    acc += i;
                }
                let label = acc * 10;
                tally.push(label);
                <span>{label}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![0, 0, 10]);
    }

    // Several imperative loops back to back in the same preamble.
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for x in 0..2_u32 {
                let mut acc: u32 = 0;
                for i in 0..x {
                    acc += i;
                }
                let mut bonus: u32 = 0;
                while bonus < 2 {
                    bonus += 1;
                }
                tally.push(acc + bonus);
                <span>{acc + bonus}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![2, 2]);
    }

    // The trailing-`;` form still works (regression).
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for x in 0..3_u32 {
                let mut acc: u32 = 0;
                for i in 0..x {
                    acc += i;
                };
                tally.push(acc);
                <span>{acc}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![0, 0, 1]);
    }

    // The `let _ = ...;` form still works (regression).
    {
        let mut tally: ::std::vec::Vec<u32> = ::std::vec::Vec::new();
        _ = ::yew::html! {
            for x in 0..3_u32 {
                let mut acc: u32 = 0;
                let _ = for i in 0..x {
                    acc += i;
                };
                tally.push(acc);
                <span>{acc}</span>
            }
        };
        ::std::assert_eq!(tally, ::std::vec![0, 0, 1]);
    }
}
