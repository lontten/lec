use crate::rule::command::Command;

mod rule;

fn main() {
    let c = Command::new();

    for x in std::env::args() {
        println!("{}", x);
        c.parse(x);
    }
    c.execute();
}


fn parse(args: Vec<&str>) {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let s1 = vec!["a", "b", "c"];
        parse(s1);

    }
}
