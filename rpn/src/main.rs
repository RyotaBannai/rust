use anyhow::{bail, ensure, Context, Result};

use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

use std::path::PathBuf;

// command:
// cargo run -- ./src/input.txt -v
// cargo run --bin rpn -- ./src/input.txt -v
#[derive(Clap, Debug)]
#[clap(
    name = "Name: My RPN program.",
    version = "1.0.0",
    author = "Author: Ryota, ryotala0528@gmail.com",
    about = "About: Super awesome RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>, // OS に依存しない汎用的なファイルパスにする
}

fn main() {
    let opts = Opts::parse();
    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap(); // unwrap is generally discouraged.. use or call.
        let reader = BufReader::new(f); // 読み出しの都度システムコールを呼ばないため高速
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        // 単純に　stdin() の結果をそのまま使うと 1 バイト読み込みごとに排他制御が働いて遅いが、ロックすることでバッファリングして読み出せる StdinLock 型のインスタンスを得ることができ、高速になる。
        run(reader, opts.verbose);
    }
}
// BufReader<File> (ファイルハンドル)
// StdinLock (標準入力ハンドル)
// いずれも、BufRead トレイト実装しているので型をトレイト境界で指定
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
    Ok(())
}

struct RpnCalculator(bool);
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        // token ごとに分割し、順序を反転、collect で iteractor 型へ変換（_ とする型推論される）
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens) // mutable reference.
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;
        let err_txt = |i: i32| -> String { format!("invalid syntax  at {}", i) };
        while let Some(token) = tokens.pop() {
            pos += 1;
            // test parse some value into i32 type.
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                // returns the contained Ok values, consuming the self value. Panics if the value is Err, with a panic message including the passed message, and the content of the Err.
                let y = stack.pop().with_context(|| err_txt(pos))?;
                let x = stack.pop().with_context(|| err_txt(pos))?;
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!(err_txt(pos)),
                };
                stack.push(res);
            }
            // `-v` オプションが有効の場合
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
    }
}

// #[..] の部分はアトリビュートと呼ばれる
// cgf(test) -> test subcommand の時だけ有効化される
#[cfg(test)]
mod tests {
    use super::*;

    // #[test] こちらも同様アトリビュート 単体テストと認識させるためのアトリビュート
    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("-5").unwrap(), -5);
        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
        assert_eq!(calc.eval("1 2 + 3 4 + *").unwrap(), 21);
        assert_eq!(calc.eval("1 2 * -3 * -6 %").unwrap(), 0);
    }
    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("3 3 ^").unwrap(), 27);
    }
}
