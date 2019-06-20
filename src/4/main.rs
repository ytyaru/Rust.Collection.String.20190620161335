/*
 * RustのコレクションString型。
 * CreatedAt: 2019-06-20
 * https://doc.rust-jp.rs/book/second-edition/ch08-02-strings.html#a%E5%86%85%E9%83%A8%E8%A1%A8%E7%8F%BE
 * 
 * 文字には3つの観点がある。（バイト、スカラー値、書記素クラスタ(人間が文字と呼ぶのに最も近い)）
 * 生の文字列データを解釈する方法がいろいろ用意されている。
 * Stringに添字アクセスすることが許されない最後の理由は、 添字アクセスという処理が常に定数時間(O(1))になると期待されるから。
 */
fn main() {
    println!("{}", "hola".to_string().len());
    println!("{}", "Здравствуйте".to_string().len()); // テキスト上は12だが24Byteのため24が返る

    let hello = "Здравствуйте";
//    let answer = &hello[0]; // error[E0277]: the type `str` cannot be indexed by `{integer}`
}

