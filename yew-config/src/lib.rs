use paste::paste;
use tt_call::{tt_call, tt_replace, tt_return};
use quote::quote;

macro_rules! is_body_placeholder {
    {
        $caller:tt
        input = [{ ConfigBody }]
    } => {
        tt_return! {
            $caller
            is = [{ true }]
        }
    };

    {
        $caller:tt
        input = [{ $other:tt }]
    } => {
        tt_return! {
            $caller
            is = [{ false }]
        }
    };
}

macro_rules! extract_arg_names {
    ( $ident_first:ident : $type_first:ty, $ident_rest:ident : $type_rest:ty ) => {
        paste! {
            <$ident_first , $ident_rest>
        }
    };
}

macro_rules! declare_yew_config_version {
    (
        #[version($maj:expr, $min:expr, $pat:expr)]
        struct {
            $($struct:tt)*
        }
        fn new($($new_args:tt)*) {
            $($new_impl:tt)*
        }
    ) => {
        paste! {
            pub struct [<YewConfigBody_ $maj _ $min _ $pat>] {
                $($struct)*
            }

            pub struct [<YewConfig_ $maj _ $min _ $pat>] {
                version: String,
                body: Box<[<YewConfigBody_ $maj _ $min _ $pat>]>
            }

            // impl [<YewConfigBody_ $maj _ $min _ $pat>] {
            //     fn new($($new_args)*) -> [<YewConfigBody_ $maj _ $min _ $pat>] {
            //         tt_call! {
            //             macro = [{ tt_replace }]
            //             condition = [{ is_body_placeholder }]
            //             replace_with = [{ [<YewConfigBody_ $maj _ $min _ $pat>] }]
            //             input = [{ $($new_impl)* }]
            //         }
            //     }
            // }

            // impl [<YewConfig_ $maj _ $min _ $pat>] {
            //     pub fn new($($new_args)*) -> [<YewConfig_ $maj _ $min _ $pat>] {
            //         let body = [<YewConfigBody_ $maj _ $min _ $pat>]::new(
            //             extract_arg_names!($($new_args)*)
            //         );

            //         [<YewConfig_ $maj _ $min _ $pat>] {
            //             version: format!("{}.{}.{}", quote!($maj), quote!($min), quote!($pat)).to_string(),
            //             body: Box::new(body),
            //         }
            //     }
            // }
        }
    }
}

declare_yew_config_version!(
    #[version(1,2,3)]
    struct {
        name: String,
    }

    fn new(foo: i32, bar: i32) {
        ConfigBody {
            name: "foo".to_string()
        }
    }
);

/*


    fn new(foo: i32, bar: i32) -> Self {
        Self {
            foo,
            bar
        }
    }


    fn new($($new_args:tt)*) -> Self {
        $($new_impl:tt)*
    }
    */

pub struct YewConfig_UnknownVersion {
    version: String,
    body: Box<()>, // this actually points to a YewConfigBody, but we don't know what type.
}

// impl YewConfig {
//     fn new() -> Self {
//         YewConfig {
//             version: "0.2.0",
//         }
//     }
// }