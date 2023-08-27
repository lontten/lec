/*
   Copyright (c) 2023 lontten
   lec is licensed under Mulan PSL v2.
   You can use this software according to the terms and conditions of the Mulan PSL v2.
   You may obtain a copy of Mulan PSL v2 at:
            http://license.coscl.org.cn/MulanPSL2
   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
   See the Mulan PSL v2 for more details.

 */

use crate::rule::command::Command;
use crate::rule::token::OptToken;

mod rule;

fn main() {
    // let c = Command::new();
    //
    // let args: Vec<String> = std::env::args().collect();
    // c.parse(&args);
    // c.execute();
}

fn print_info(opts: Vec<OptToken>, args: Vec<String>) -> String {
    println!("this is args:{:?}", args);
    return "this is lec".to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {

        //没有任何参数、选项
        let mut c = Command::new();
        let f = Some(|opts, args| {
            println!("func do done: args:{:?}", args);
            return "func do done:".to_string();
        });
        c.func = f;

        let s1 = vec![];
        c.parse(&s1);
        c.func.unwrap()(vec![], s1);

        assert_eq!(c.execute_str(), "lec");
    }
}
