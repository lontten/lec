/*
   Copyright (c) 2023 lontten
   lec is licensed under Mulan PSL v2.
   You can use this software according to the terms and conditions of the Mulan PSL v2.
   You may obtain a copy of Mulan PSL v2 at:
            http://license.coscl.org.cn/MulanPSL2
   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
   See the Mulan PSL v2 for more details.

 */

use crate::rule::command::LecCommand;
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
    use crate::rule::command::{App, LecOption, LecOption};

    use super::*;

    #[test]
    fn parse_test() {

        //没有任何参数、选项
        App::new()
            .add_option(LecOption::new("version").set_short_name('v'))
            .set_func(|opts, args| {
                println!("opts:{:?},args:{:?}", opts, args);
                "0.1.0".to_string()
            });

        app.set_func(|opts: Vec<OptToken>, args: Vec<String>| {
            return "func do done:".to_string();
        });
        let c_list = LecCommand::new("list")
            .add_option(
                LecOption::new("all")
                    .set_short_name('a')
            );


        let s1 = vec!["a".to_string(), "b".to_string()];
        app.parse(&s1);
        app.func.unwrap()(vec![], s1);

        assert_eq!(app.execute_str(), "lec");
    }

    #[test]
    fn mut_test() {}
}
