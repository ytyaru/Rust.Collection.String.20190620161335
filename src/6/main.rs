/
 * RustのコレクションString型。
 * CreatedAt: 2019-06-20
 * https://doc.rust-jp.rs/book/second-edition/ch08-02-strings.html#a%E6%96%87%E5%AD%97%E5%88%97%E3%82%92%E8%B5%B0%E6%9F%BB%E3%81%99%E3%82%8B%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89%E7%BE%A4
 */
fn main() {
    // str.chars()を使えばスカラー値(char型)で走査できる
    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{}", c);
    }

    // str.bytes()を使えばバイト単位(u8型)で走査できる
    let hello = "Здравствуйте";
    for c in hello.bytes() {
        println!("{}", c);
    }
}

