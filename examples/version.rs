// Copyright (c) 2023 lontten
// lec is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.


///
/// -v --version
use lec::{App, AppConfig, ArgLimit, LecOption};

fn main() {
    let mut app = App::new(AppConfig {
        name: "lec".to_string(),
        version: "v0.1.0".to_string(),
        author: "lontten".to_string(),
        email: "lontten@163.com".to_string(),
        about: "".to_string(),
    });
    app.default()
        .set_option(
            vec![
                LecOption::new("version").set_short_name('v').set_about("打印版本信息")
            ],
            ArgLimit::LimitNum(0),
            |conf, opts, args| {
                if opts.len() == 1 {
                    let opt = &opts[0];
                    if opt.name == "version" {
                        println!("{} {}", conf.name, conf.version);
                        return;
                    }
                }
            })
        .run();
}