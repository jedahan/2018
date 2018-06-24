#![feature(rust_2018_preview, use_extern_macros)]
use clap::App;

fn main() {
    App::new("myapp")
       .version("1.0")
       .about("Does great things!")
       .author("Kevin K.")
       .get_matches();
}
