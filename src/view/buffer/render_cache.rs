use std::collections::HashMap;

pub trait RenderCache {
    fn invalidate_from(&mut self, _: usize) {}
}

impl<T> RenderCache for HashMap<usize, T> {
    /// Invalidates cache entries beyond the specified limit.
    fn invalidate_from(&mut self, limit: usize) {
        self.reta