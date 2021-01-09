use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// command:
// cargo run -- ./src/input.txt -v
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
    formula_file: Option<String>,
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
        run(reader, opts.verbose)
    }
}
// BufReader<File> (ファイルハンドル)
// StdinLock (標準入力ハンドル)
// いずれも、BufRead トレイト実装しているので型をトレイト境界で指定
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

struct RpnCalculator(bool);
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        // token ごとに分割し、順序を反転、collect で iteractor 型へ変換（_ とする型推論される）
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens) // mutable reference.
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();
        while let Some(token) = tokens.pop() {
            // test parse some value into i32 type.
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax"); // returns the contained Ok values, consuming the self value. Panics if the value is Err, with a panic message including the passed message, and the content of the Err.
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }
            // `-v` オプションが有効の場合
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
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
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("-5"), -5);
        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
        assert_eq!(calc.eval("1 2 + 3 4 + *"), 21);
        assert_eq!(calc.eval("1 2 * -3 * -6 %"), 0);
    }
    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("3 3 ^"), 27);
    }
}
