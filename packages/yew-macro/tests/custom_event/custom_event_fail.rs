mod attribute {
    use yew::custom_event;

    #[custom_event]
    struct NoAttributeGiven(Event);

    #[custom_event(ident = not a literal string value)]
    struct AttributeValueNotALitStr(Event);

    #[custom_event("string without ident")]
    struct StringWithoutIdent(Event);
}

mod structs {
    use yew::custom_event;

    #[custom_event(unit_struct)]
    struct UnitStruct;

    #[custom_event(tuple_struct)]
    struct TupleStruct(Event, Event);

    #[custom_event(normal_struct)]
    struct PlainOldStruct {
        event: Event,
    }
}

fn main() {}
