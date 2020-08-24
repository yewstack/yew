use paste::paste;
use quote::quote;
use std::convert::TryFrom;
use tt_call::{tt_call, tt_replace, tt_return};

pub struct YewConfigUnknownVersion {
    pub version: String,

    // this actually points to a YewConfigBody, but we need to manually cast based on
    // the version string. This can be done automatically with to_versioned_config().
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
        $(
            #[version($maj:expr, $min:expr, $pat:expr)]
            {
                struct {
                    $($struct:tt)*
                }

                // TODO might be able to define schema-specific types inside modules
                $(types {
                    $($type_decls:item)*
                })?

                fn new( $( $new_arg:ident : $new_ty:ty ),* ) {
                    $($new_impl:tt)*
                }
            }
        )*
    ) => {
        $(
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
                            version: format!("{}.{}.{}", $maj, $min, $pat),
                            body: Box::new(body),
                        }
                    }
                }
            }
        )*

        paste! {
            pub enum YewVersionedConfig {
                $(
                    [<Version $maj _ $min _ $pat>]([<YewConfig $maj _ $min _ $pat>])
                ),*
            }
        }

        impl TryFrom<YewConfigUnknownVersion> for YewVersionedConfig {
            type Error = &'static str;

            fn try_from(config: YewConfigUnknownVersion) -> Result<YewVersionedConfig, Self::Error> {
                $(
                    if (config.version == format!("{}.{}.{}", $maj, $min, $pat)) {
                        return Ok(
                            paste! {
                                YewVersionedConfig::[<Version $maj _ $min _ $pat>](
                                    unsafe {
                                        std::mem::transmute::<
                                            YewConfigUnknownVersion, [<YewConfig $maj _ $min _ $pat>]
                                        >(config)
                                    }
                                )
                            }
                        )
                    }
                )*
                // TODO make into a real error
                Err("Unknown schema of loaded Yew project configuration. The `yew-config` crate is most likely out of date.")?
            }
        }
    }
}

declare_yew_config_version!(
    #[version(0, 2, 0)]
    {
        struct {
            pub app_name: String,
        }

        fn new(app_name: String) {
            ConfigBody {
                app_name,
            }
        }
    }
);

// Latest schema is available under YewConfig alias
pub type YewConfig = YewConfig0_2_0;