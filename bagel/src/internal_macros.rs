macro_rules! new {
    () => {
        Self::new()
    };
    ($ty:ty) => {
        <$ty>::new()
    };
    ($ty:ty, $init:expr) => {
        <$ty>::new($init)
    };
}
