use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};
#[derive(Clone, Copy)]
enum MinoKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 22;

type FieldSize = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

// テトリミノの形状
const MINOS: [[[usize; 4]; 4]; 7] = [
    // Iミノ
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
    // Oミノ
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    // Sミノ
    [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
    // Zミノ
    [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    // Jミノ
    [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
    // Lミノ
    [[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
    // Tミノ
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
];

// テトリミノがフィールドに衝突する場合は`true`を返す
fn is_collision(field: &FieldSize, pos: &Position, mino: MinoKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y + pos.y][x + pos.x] & MINOS[mino as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}
struct Position {
    x: usize,
    y: usize,
}

// フィールドを描画する
fn draw(field: &FieldSize, pos: &Position) {
    // 描画用フィールドの生成
    let mut field_buf = field.clone();
    // 描画用フィールドにテトリミノの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            field_buf[y + pos.y][x + pos.x] |= MINOS[MinoKind::I as usize][y][x];
        }
    }
    // フィールドを描画
    println!("\x1b[H"); // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}
fn main() {
    let field = Arc::new(Mutex::new([
        [1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]));
    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // フィールドを描画
    draw(&field.lock().unwrap(), &pos.lock().unwrap());

    // 自然落下処理
    {
        let pos = Arc::clone(&pos);
        let field = Arc::clone(&field);
        let _ = thread::spawn(move || {
            loop {
                // 1秒間スリーブする
                thread::sleep(time::Duration::from_millis(1000));
                // 自然落下
                let mut pos = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    // posの座標を更新
                    *pos = new_pos;
                } else {
                    // テトリミノをフィールドに固定
                    for y in 0..4 {
                        for x in 0..4 {
                            field[y + pos.y][x + pos.x] |= MINOS[MinoKind::I as usize][y][x];
                        }
                    }
                    // posの座標を初期値へ
                    *pos = Position { x: 4, y: 0 };
                }
                // フィールドを描画
                draw(&field, &pos);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                // フィールドを描画
                draw(&field, &pos);
            }
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                draw(&field, &pos);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                // フィールドを描画
                draw(&field, &pos);
            }
            Ok(Key::Char('q')) => {
                // カーソルを再表示
                println!("\x1b[?25h");
                return;
            }
            _ => (), // 何もしない
        }
    }
}
