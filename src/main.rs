use std::{
    collections::VecDeque,
    io::{Read, Write},
};

fn main() {
    let mut args = std::env::args();
    args.next();
    let file = match args.next() {
        Some(s) => s,
        None => panic!("Specify the file to be executed."),
    };

    let mut script = String::new();
    std::fs::File::open(file)
        .unwrap()
        .read_to_string(&mut script)
        .unwrap();
    let script: Vec<char> = script.chars().collect();

    let mut memory = VecDeque::new(); // 前方向にも拡張できるようにVecDequeを使う
    memory.push_back(0); // 0番目のメモリを用意する
    let mut pointer = 0;
    exec(&script, &mut memory, &mut pointer, false);
}

fn exec(script: &[char], memory: &mut VecDeque<i128>, pointer: &mut usize, noexec: bool) -> i128 {
    let mut i = 0;
    while i < script.len() {
        // 非実行モードのときは実行しない
        if !noexec {
            if script[i] == '>' {
                // ポインタを進める
                *pointer += 1;
                if memory.len() - 1 < *pointer {
                    // memoryのサイズが足りないなら増やす
                    memory.push_back(0);
                }
            } else if script[i] == '<' {
                if *pointer != 0 {
                    // ポインタが0でなければ単純にポインタを戻す
                    *pointer -= 1;
                } else {
                    // 負のメモリにアクセスする場合はmemoryを前方向に拡張する
                    memory.push_front(0);
                }
            } else if script[i] == '+' {
                memory[*pointer] += 1;
            } else if script[i] == '-' {
                memory[*pointer] -= 1;
            } else if script[i] == '.' {
                // ポインタの指す先の値を文字として画面に出力する
                if let Ok(output) = u8::try_from(memory[*pointer]) {
                    // u8に変換できるならUTF-8として出力する
                    let _ = std::io::stdout().write_all(&[output]);
                } else {
                    // そうでないならUTF-32として出力する
                    let output = u32::try_from(memory[*pointer]).unwrap();
                    let output: char = unsafe { std::mem::transmute(output) };
                    print!("{}", output);
                }
                let _ = std::io::stdout().flush();
            } else if script[i] == ',' {
                // 入力を1バイト受けとり､ポインタの指す先に書き込む
                let mut buf = [0];
                std::io::stdin().read_exact(&mut buf).unwrap();
                memory[*pointer] = buf[0] as i128;
            }
        }

        if script[i] == '[' {
            let mut noexec = false;
            if memory[*pointer] == 0 {
                // pointerの指す値が0なら[]の中を実行しない
                noexec = true;
            }

            let mut end_position = -1;
            while end_position == -1 {
                // -1が返ってきたらループ､そうでなければ抜ける
                end_position = exec(&script[(i + 1)..], memory, pointer, noexec);
            }
            // []の終わりまで飛ばす
            i += end_position as usize + 1;
        } else if script[i] == ']' {
            if memory[*pointer] != 0 {
                // pointerの指す値が0でなければループする
                return -1;
            } else {
                // そうでないならループから抜ける
                return i as i128;
            }
        }

        i += 1;
    }

    // iはscriptのインデックスの最大値+1なので1を引いてから返す
    (i - 1) as i128
}
