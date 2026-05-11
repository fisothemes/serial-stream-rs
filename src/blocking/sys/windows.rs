use crate::config::{Direction, SerialConfig};

#[derive(Debug)]
pub struct InnerStream<P: AsRef<str>, D: Direction> {
    config: SerialConfig<P, D>,
}
