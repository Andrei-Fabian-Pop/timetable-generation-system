macro_rules! empty_box_body {
    () => {
        BoxBody::new(http_body_util::Empty::<bytes::Bytes>::new())
    };
}
pub(crate) use empty_box_body;

macro_rules! full_box_body {
    ($chunk:expr) => {
        BoxBody::new(http_body_util::Full::new($chunk.into()))
    };
}
pub(crate) use full_box_body;
