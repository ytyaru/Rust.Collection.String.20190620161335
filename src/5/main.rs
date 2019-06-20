/*
 * RustのコレクションString型。
 * CreatedAt: 2019-06-20
 * https://doc.rust-jp.rs/book/second-edition/ch08-02-strings.html#a%E6%96%87%E5%AD%97%E5%88%97%E3%82%92%E3%82%B9%E3%83%A9%E3%82%A4%E3%82%B9%E3%81%99%E3%82%8B
 */
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", hello);
    println!("{}", s);

    let s = &hello[0..1]; // 実行時エラー（パニック）: thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'

    // 文字の区切をバイト数で指定せねばならない
}

