// This module contains shared functionality for instance batching

pub trait InstanceBatch {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}
