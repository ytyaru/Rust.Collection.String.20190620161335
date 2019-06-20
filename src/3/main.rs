/*
 * RustのコレクションString型。
 * String型はchar型の可変配列。
 * CreatedAt: 2019-06-20
 * https://doc.rust-jp.rs/book/second-edition/ch08-02-strings.html
 * http://d.sunnyone.org/2015/06/rustwindowslpcwstr-lpwstr.html
 */
fn main() {
    let s = "hello".to_string();
    let h = s[0]; // error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`
    // 文字列は添字アクセスをサポートしていない
}

