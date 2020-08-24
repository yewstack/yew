use paste::paste;
use tt_call::{tt_call, tt_replace, tt_return};
use quote::quote;

pub struct YewConfigNonspecific {
    pub version: String,

    // this actually points to a YewConfigBody, but we need to manually cast based on
    // the version string.
    pub body: Box<()>,
}

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

macro_rules! declare_yew_config_version {
    (
        #[version($maj:expr, $min:expr, $pat:expr)]
        struct {
            $($struct:tt)*
        }
        fn new( $( $new_arg:ident : $new_ty:ty ),* ) {
            $($new_impl:tt)*
        }
    ) => {
        paste! {
            pub struct [<YewConfigBody $maj _ $min _ $pat>] {
                $($struct)*
            }

            pub struct [<YewConfig $maj _ $min _ $pat>] {
                pub version: String,
                pub body: Box<[<YewConfigBody $maj _ $min _ $pat>]>
            }

            impl [<YewConfigBody $maj _ $min _ $pat>] {
                fn new(
                    $( $new_arg: $new_ty ),*
                ) -> [<YewConfigBody $maj _ $min _ $pat>] {
                    tt_call! {
                        macro = [{ tt_replace }]
                        condition = [{ is_body_placeholder }]
                        replace_with = [{ [<YewConfigBody $maj _ $min _ $pat>] }]
                        input = [{ $($new_impl)* }]
                    }
                }
            }

            impl [<YewConfig $maj _ $min _ $pat>] {
                pub fn new(
                    $( $new_arg: $new_ty ),*
                ) -> [<YewConfig $maj _ $min _ $pat>] {
                    let body = [<YewConfigBody $maj _ $min _ $pat>]::new(
                        $( $new_arg ),*
                    );

                    [<YewConfig $maj _ $min _ $pat>] {
                        version: format!("{}.{}.{}", quote!($maj), quote!($min), quote!($pat)).to_string(),
                        body: Box::new(body),
                    }
                }
            }
        }
    }
}

declare_yew_config_version!(
    #[version(0,0,2)]
    struct {
        pub app_name: String,
    }

    fn new(app_name: String) {
        ConfigBody {
            app_name,
        }
    }
);