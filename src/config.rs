use std::path::PathBuf;

pub enum Mode {
    LowSpace,
    HighThrouput,
}
pub struct Config(Arc<Inner>);

pub struct Inner {
    pub cache_capacity: usize,
    pub flush_every_ms: Option<u64>,
    pub segment_size: usize,
    pub path: PathBuf,
    pub create_new: bool,
    pub mode: Mode,
}