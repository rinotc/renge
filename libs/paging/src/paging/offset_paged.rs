use crate::paging::offset_paging::OffsetPaging;

/// ページングによって取得した項目と、その取得に使用したページング情報を表す構造体です。
///
/// # 型パラメータ
/// - `T`: ページング結果に含まれる項目の型です。
///
/// # フィールド
/// - `items`: ページングによって取得した項目を格納するベクターです。
/// - `paging`: 現在の項目を取得するために使用した `OffsetPaging` 構造体のページング情報です。
pub struct OffsetPaged<T> {
    pub items: Vec<T>,
    pub paging: OffsetPaging,
}

impl<T> OffsetPaged<T> {
    pub fn new(items: Vec<T>, paging: OffsetPaging) -> Self {
        Self { items, paging }
    }
}
