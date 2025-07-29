use color_eyre::Result;
use std::path::Path;

pub trait ReadFile: Sized {
    fn new<T: AsRef<Path>>(data_path: T) -> Result<Self>;
}
