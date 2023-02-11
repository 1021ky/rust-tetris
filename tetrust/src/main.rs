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

// テトリミノがフィールドに衝突する場合は`ture`を返す
fn is_collision(field: &[[usize; 12]], pos: &Position, mino: MinoKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y + pos.y + 1][x + pos.x] & MINOS[mino as usize][y][x] == 1 {
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

fn main() {
    let field = [
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
    ];
    let mut pos = Position { x: 4, y: 0 };
    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // 30マス分落下させてみる
    for _ in 0..30 {
        // 描画用フィールドを初期化
        let mut field_buf = field;
        // 当たり判定
        if !is_collision(&field, &pos, MinoKind::I) {
            // posのy座標を更新
            pos.y += 1;
        }
        // 描画用フィールドにテトリミノの情報を書き込む
        for y in 0..4 {
            for x in 0..4 {
                field_buf[y + pos.y][x + pos.x] |= MINOS[MinoKind::I as usize][y][x];
            }
        }
        // フィールドを描画
        println!("\x1b[H"); // カーソルを先頭に移動
        for y in 0..22 {
            for x in 0..12 {
                if field_buf[y][x] == 1 {
                    print!("[]");
                } else {
                    print!(" .");
                }
            }
            println!();
        }
        // 落下速度を調整するために1秒間スリーブする
        thread::sleep(time::Duration::from_millis(1000));
    }
    // カーソルを再表示
    println!("\x1b[?25h");
}
