/*
 * RustのコレクションString型。
 * String型はchar型の可変配列。
 * CreatedAt: 2019-06-20
 * https://doc.rust-jp.rs/book/second-edition/ch08-02-strings.html
 * http://d.sunnyone.org/2015/06/rustwindowslpcwstr-lpwstr.html
 */
fn main() {
    let s1 = "A".to_string();
    let s2 = "B".to_string();
//    let s3 = s1 + s2; // error[E0308]: mismatched types
    let s3 = s1 + &s2; // error[E0308]: mismatched types
    println!("{}", s3);
//    println!("{}", s1); // s1 + &s2 で s1の所有権がs3にムーブしたためs1は無効化。error[E0382]: use of moved value: `s1`
    println!("{}", s2);
}

