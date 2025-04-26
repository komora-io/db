#[macro_export]
macro_rules! open_tmp {
    () => {
        db::open(concat!(file!(), ':', line!())).unwrap()
    };
}
