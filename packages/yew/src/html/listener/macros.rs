macro_rules! static_event_impl {
    ($(
        $(#[$event_doc:meta])*
        $event_type:ident => $event:ty,)* $(,)?
    ) => {
        $(
            $(#[$event_doc])*
            #[allow(non_camel_case_types, missing_debug_implementations, missing_docs)]
            pub struct $event_type;

            impl StaticEvent for $event_type {
                type Event = $event;

                fn event_name() -> &'static str {
                    stringify!($event_type)
                }
            }
        )*
    };
}
