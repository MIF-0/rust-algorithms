pub mod path_compression_weighted_quick_union;
pub mod quick_find;
pub mod quick_union;
pub mod union_find;
pub mod weighted_quick_union;

#[cfg(test)]
mod test_utils {
    use crate::dynamic_connectivity::union_find::UnionFind;
    use std::any::Any;

    pub fn down_cast<T: 'static>(obj: &(impl UnionFind + 'static)) -> &T {
        match to_any(obj).downcast_ref::<T>() {
            Some(val) => val,
            None => panic!("&union_find isn't class you require!"),
        }
    }

    fn to_any(obj: &(impl UnionFind + 'static)) -> &dyn Any {
        return obj;
    }
}
