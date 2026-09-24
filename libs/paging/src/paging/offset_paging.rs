/// オフセットベースのページネーションを表す構造体です。
///
/// 1 回に取得する項目数の上限と、データセットの先頭からスキップする
/// 項目数を指定するオフセットを使用して、データセットのページネーションを
/// 管理します。
///
/// # フィールド
///
/// * `limit` - 1 ページで取得する項目数の上限です。
/// * `offset` - データセットの先頭からスキップする項目数です。
///
/// # 使用例
///
/// ```rust
/// use paging::paging::offset_paging::OffsetPaging;
///
/// let paging = OffsetPaging::new(10, 20);
///
/// println!("Page: {}, Per page: {}", paging.page(), paging.per_page());
/// ```
///
/// この例では、先頭の 20 項目をスキップした後に 10 項目を取得する
/// `OffsetPaging` インスタンスを初期化しています。
pub struct OffsetPaging {
    pub limit: u32,
    pub offset: u32,
}

impl OffsetPaging {
    pub fn new(limit: u32, offset: u32) -> Self {
        Self { limit, offset }
    }

    pub fn new_by_page(page: u32, per_page: u32) -> Self {
        Self {
            limit: per_page,
            offset: (page - 1) * per_page,
        }
    }

    pub fn page(&self) -> u32 {
        self.offset / self.limit + 1
    }

    pub fn per_page(&self) -> u32 {
        self.limit
    }
    
    pub fn next(&self) -> Self {
        Self {
            limit: self.limit,
            offset: self.offset + self.limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_paging_with_limit_and_offset() {
        let paging = OffsetPaging::new(10, 20);

        assert_eq!(paging.page(), 3);
        assert_eq!(paging.per_page(), 10);
    }

    #[test]
    fn new_by_page_creates_first_page() {
        let paging = OffsetPaging::new_by_page(1, 10);

        assert_eq!(paging.page(), 1);
        assert_eq!(paging.per_page(), 10);
    }

    #[test]
    fn new_by_page_creates_later_page() {
        let paging = OffsetPaging::new_by_page(3, 10);

        assert_eq!(paging.page(), 3);
        assert_eq!(paging.per_page(), 10);
    }

    #[test]
    fn page_rounds_down_when_offset_is_not_page_aligned() {
        let paging = OffsetPaging::new(10, 25);

        assert_eq!(paging.page(), 3);
    }
}
