use rand::{thread_rng, Rng};
use std::{fmt, io::stdin};

//newtype
struct Range(core::ops::Range<u64>);

//为结构体Range实现Display特征
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0.start, self.0.end - 1)
    }
}

impl Range {
    fn new(r: core::ops::Range<u64>) -> Range {
        Range(r)
    }
}

fn main() {
    let mut range = 1u64..101u64;
    'main: loop {
        println!("\n猜数字游戏\n输入:\ns     开始游戏\nr     设置随机数范围\nq     退出游戏\n");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("读取输入失败！");
        let input = input[..].trim_end();
        match input {
            "s" => play(range.clone()),
            "q" => {
                println!("游戏即将退出...");
                break 'main;
            }
            "r" => set_range(&mut range),
            _ => (),
        };
    }
}

//设置随机数范围
fn set_range(range: &mut core::ops::Range<u64>) {
    let mut start = String::new();
    let mut end = String::new();
    println!("当前随机数范围{}", Range::new(range.clone()));
    println!("请输入要设置的最小值: ");
    stdin().read_line(&mut start).unwrap();
    let start = start[..].trim_end().parse::<u64>().expect("解析数字失败！");
    println!("请输入要设置的最大值: ");
    stdin().read_line(&mut end).unwrap();
    let end = end[..].trim_end().parse::<u64>().expect("解析数字失败！");
    *range = start..(end + 1);
    println!("当前随机数范围{}", Range::new(range.clone()));
}

//开始游戏
fn play(range: core::ops::Range<u64>) {
    use std::cmp::Ordering;
    println!("生成随机数字中...\n开始游戏!");
    let num: u64 = thread_rng().gen_range(range);
    'play: loop {
        let mut input = String::new();
        println!("请输入您的答案：");
        stdin().read_line(&mut input).expect("读取输入失败！");
        let input = input[..].trim_end().parse::<u64>().expect("解析数字失败！");
        match input.cmp(&num) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("恭喜你猜对了: {}!", input);
                break 'play;
            }
        }
    }
}
