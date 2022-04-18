use std::sync::Arc;

use crate::handler::Handler;

pub trait Server {
    fn new(addr: String) -> Self;
    fn run(self, handler: Arc<dyn Handler>);
}
