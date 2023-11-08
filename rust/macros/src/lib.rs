#[macro_export]
macro_rules! hashmap {
    // This second repetition is to capture a possible trailing comma, which is not handled by the
    // normal repetition handling.
    ( $( $x:expr => $y:expr ),* $(,)? ) => {
        {
        let mut hm = ::std::collections::HashMap::new();
        $(
            hm.insert($x, $y);
        )*
        hm
        }
    };
}
