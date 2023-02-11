# メモ

## rustのよさ

```rust
let mut field_buf = field;
```

既存のコードを修正して、変数の値を書き換えるようになると、必ずmutableにしないといけない。
それだけでもレビューのときに変更の影響範囲を確認しやすくていいな。デフォルトがimmutableなのは。

## 知らなかったことメモ

[継承(Derive)](https://doc.rust-jp.rs/rust-by-example-ja/trait/derive.html)
pythonでいうアノテーションのように、対象の上の行に書く。
つけるだけで意味がある場合と、実装してオーバーライドする場合がある。

```rust
// `Centimeters`, a tuple struct that can be compared
// `Centimeters`は比較可能なタプルになる
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
// `Inches`はプリント可能なタプルになる
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}
```
