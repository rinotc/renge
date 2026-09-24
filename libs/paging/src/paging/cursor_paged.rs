use crate::paging::cursor_paging::CursorPaging;

/// ページングによって取得した項目と、その取得に使用したページング情報を表す構造体です。
pub struct CursorPaged<T, C> {
    pub items: Vec<T>,
    pub paging: CursorPaging<C>,
}

impl<T, C> CursorPaged<T, C> {
    pub fn new(items: Vec<T>, paging: CursorPaging<C>) -> Self {
        Self { items, paging }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_paged_new_creates_paged_result_with_items_and_paging() {
        let paging = CursorPaging::new("cursor");
        let paged = CursorPaged::new(vec![1, 2, 3], paging);

        assert_eq!(paged.items, vec![1, 2, 3]);
        assert_eq!(paged.paging.cursor, "cursor");
    }
}
