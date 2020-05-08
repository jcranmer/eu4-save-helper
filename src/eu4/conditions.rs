macro_rules! conditions { ($condition:ident) => {
    $condition!(
        fn absolutism(country: &Country, value: FixedPoint) -> bool {
            country.absolutism >= value
        })
}}
