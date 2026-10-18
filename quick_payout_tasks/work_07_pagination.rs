//! Work 7: Pagination
pub fn paginate<T>(items: &[T], offset: usize, limit: usize) -> &[T] {
    let max_limit = limit.min(100);
    let start = offset.min(items.len());
    let end = (start + max_limit).min(items.len());
    &items[start..end]
}
