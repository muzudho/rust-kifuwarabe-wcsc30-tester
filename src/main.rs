//! [Struct std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
//! [External Command](https://rust-lang-nursery.github.io/rust-cookbook/os/external.html)
//! [Rustで外部コマンド実行](https://qiita.com/imos/items/fdb9bfcc1bb3837576de)
//! [18.5.1 パイプ](https://doc.rust-jp.rs/rust-by-example-ja/std_misc/process/pipe.html)
//! [command - 生成された子プロセスとの間で複数回パイプできない](https://tutorialmore.com/questions-1733265.htm)
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    // とりあえず、Windows 10 で使う☆（＾～＾）
    println!("Trace   | Check OS.");
    if !cfg!(target_os = "windows") {
        panic!("Unexpected OS. Please use Windows.");
    };

    // Test.
    {
        println!("Trace   | cd...");
        // Windows は cmd 実行ファイル経由でコマンドを実行しろだぜ☆（＾～＾）
        // コマンドを入力して、出力が返ってくるだけのコマンドなら output を呼び出すだけだぜ☆（＾～＾）
        let output = match Command::new("cmd").args(&["/C", "cd"]).output() {
            Ok(x) => x,
            Err(err) => panic!("Running process error: {}", err),
        };

        println!("Trace   | Check status...");
        if !output.status.success() {
            panic!("Command executed with failing error code");
        }
        // TODO 日本語の文字化けを直してほしい☆（＾～＾）
        println!("Trace   | Encoding...");
        let encoded = String::from_utf8_lossy(output.stdout.as_slice());
        println!("Trace   | Encoded: {}", encoded);
    }

    {
        // 将棋ソフトのプロセスを取得するぜ☆（＾～＾）
        // 将棋ソフトは、Rustアプリケーションから見れば、シェルに似ている☆（＾～＾）
        println!("Trace   | Spawn...");
        let mut child_shell36 = match Command::new("cmd").args(&["/C", "C:/Users/むずでょ/source/repos/rust-kifuwarabe-wcsc30/target/release/rust-kifuwarabe-wcsc30.exe"])
            .stdin(Stdio::piped())
            // .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(process) => process,
            Err(err) => panic!("Running process error: {}", err),
        };
        println!("Trace   | Child stdin...");
        let child_in22 = match child_shell36.stdin.as_mut() {
            Some(x) => x,
            None => panic!("Child process stdin has not been captured!"),
        };
        println!("Trace   | Buffer reader...");
        let mut child_out = BufReader::new(match child_shell36.stdout.as_mut() {
            Some(x) => x,
            None => panic!("Child process stdout has not been captured!"),
        });
        println!("Trace   | New line...");
        let mut line = String::new();
        println!("Trace   | usi...");
        match child_in22.write(b"usi\n") {
            Ok(size) => println!("Trace   | Size {:?}", size),
            Err(why) => panic!("{}", Error::description(&why)),
        };
        // 複数行返ってくるやつは　どうやって終わりを判定するんだぜ☆（＾～＾）？
        {
            // usiok を受け取るまで無限ループするからな☆（＾～＾）
            loop {
                // 1行目: id name Kifuwarabe WCSC30.build55\n
                // 2行目: id author TAKAHASHI, Satoshi\n
                // 3行目: usiok\n
                println!("Trace   | Read line...");
                match child_out.read_line(&mut line) {
                    Ok(size) => println!("Trace   | Size {:?}", size),
                    Err(why) => panic!("{}", Error::description(&why)),
                };
                println!("Trace   > [{}]", line);
                if line == "usiok\n" {
                    println!("Trace   | Usiok.");
                    break;
                }
                line.clear();
            }
        }

        // 将棋ソフトを終わらせてから、このテスターを終わらせろだぜ☆（＾～＾）
        println!("Trace   | Quit...");
        match child_in22.write(b"quit\n") {
            Ok(size) => println!("Trace   | Size {:?}", size),
            Err(why) => panic!("{}", Error::description(&why)),
        };
        println!("Trace   | Wait child process...");
        match child_shell36.wait() {
            Ok(size) => println!("Trace   | Size {:?}", size),
            Err(why) => panic!("{}", Error::description(&why)),
        };
    }

    println!("Trace   | Finished.");
}
