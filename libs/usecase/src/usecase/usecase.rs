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
/// - `handle(&self, input: Self::Input) -> Self::Output`:
///   入力を処理し、対応する出力を返します。ユースケースの中心となる処理を実装します。
///
/// # 使用例
///
/// イベントを作成するユースケースでは、入力を構造体として定義し、
/// 処理結果を列挙型で表現できます。
///
/// ```rust
/// use libs_usecase::usecase::usecase::UseCase;
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
/// impl UseCase for CreateEventUseCase {
///     type Input = CreateEventInput;
///     type Output = CreateEventOutput;
///
///     fn handle(&self, input: Self::Input) -> Self::Output {
///         if input.title.trim().is_empty() {
///             CreateEventOutput::InvalidTitle
///         } else {
///             // 実際には、ここでイベントを保存して発行された ID を返します。
///             CreateEventOutput::Created { event_id: 1 }
///         }
///     }
/// }
///
/// let create_event_use_case = CreateEventUseCase;
/// let output = create_event_use_case.handle(CreateEventInput {
///     title: "Rust 勉強会".to_string(),
/// });
///
/// assert_eq!(output, CreateEventOutput::Created { event_id: 1 });
/// ```
pub trait UseCase {
    type Input;
    type Output;

    fn handle(&self, input: Self::Input) -> Self::Output;
}
