use async_trait::async_trait;

/// 入力と出力の型を定義した、汎用的なユースケースを表すトレイトです。
///
/// ユースケースの業務ロジックやアプリケーション処理をカプセル化し、
/// 再利用しやすく、テストしやすい形にするために使用します。実装側では、
/// 入力と出力の型を指定し、`handle` メソッドにユースケース固有の処理を実装します。
///
/// # 関連型
///
/// - `Input`: ユースケースが入力として受け取るデータの型です。
/// - `Output`: ユースケースが出力として返すデータの型です。
///
/// # 必須メソッド
///
/// - `handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error>`:
///   非同期で入力を処理し、成功時は出力、失敗時はエラーを返します。
///
/// # 使用例
///
/// イベントを作成するユースケースでは、入力を構造体として定義し、
/// 処理結果を列挙型で表現できます。
///
/// ```rust
/// use async_trait::async_trait;
/// use libs_usecase::usecase::usecase::UseCase;
/// use std::convert::Infallible;
///
/// #[derive(Debug, PartialEq)]
/// struct CreateEventInput {
///     title: String,
/// }
///
/// #[derive(Debug, PartialEq)]
/// enum CreateEventOutput {
///     Created { event_id: u64 },
///     InvalidTitle,
/// }
///
/// struct CreateEventUseCase;
///
/// #[async_trait]
/// impl UseCase for CreateEventUseCase {
///     type Input = CreateEventInput;
///     type Output = CreateEventOutput;
///     type Error = Infallible;
///
///     async fn handle(
///         &self,
///         input: Self::Input,
///     ) -> Result<Self::Output, Self::Error> {
///         if input.title.trim().is_empty() {
///             Ok(CreateEventOutput::InvalidTitle)
///         } else {
///             // 実際には、ここでイベントを保存して発行された ID を返します。
///             Ok(CreateEventOutput::Created { event_id: 1 })
///         }
///     }
/// }
///
/// async fn example() {
///     let create_event_use_case = CreateEventUseCase;
///     let output = create_event_use_case
///         .handle(CreateEventInput {
///             title: "Rust 勉強会".to_string(),
///         })
///         .await
///         .unwrap();
///
///     assert_eq!(output, CreateEventOutput::Created { event_id: 1 });
/// }
/// ```
#[async_trait]
pub trait UseCase: Send + Sync {
    type Input: Send;
    type Output: Send;
    type Error: Send + Sync + 'static;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}
