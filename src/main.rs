use crate::rule::command::Command;

mod rule;

fn main() {
    let c = Command::new();

    for x in std::env::args() {
        println!("{}", x);
        c.parse(x.as_str());
    }
    c.execute();
}


fn parse(args: Vec<&str>) {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {

        //没有任何参数、选项
        let c = Command::new();
        let s1 = vec![];
        for x in s1 {
            c.parse(x);
        }

        assert_eq!(c.execute(), "lec");


        //一个参数
        let c = Command::new();
        
        let s1 = vec!["a"];
        for x in s1 {
            c.parse(x);
        }

        assert_eq!(c.execute(), "lec a");


        //多个参数
        let c = Command::new();
        let s1 = vec!["a", "b", "c"];
        for x in s1 {
            c.parse(x);
        }

        assert_eq!(c.execute(), "lec a b c");
    }
}
