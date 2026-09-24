/// カーソルベースのページネーションを表す構造体です。
///
/// データセットの続きの位置を表すカーソルを使用して、ページネーションを
/// 管理します。
pub struct CursorPaging<T> {
    pub cursor: T,
}

impl<T> CursorPaging<T> {
    pub fn new(cursor: T) -> Self {
        Self { cursor }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_paging_new_creates_paging_with_cursor() {
        let paging = CursorPaging::new("cursor");

        assert_eq!(paging.cursor, "cursor");
    }
}
