#![feature(rust_2018_preview, use_extern_macros)]
use clap::App;
use std::fmt;

pub struct Plugin {
    pub author: String,
    pub name: String,
}

impl fmt::Display for Plugin {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}/{}", self.author, self.name)?;
        writeln!(formatter, "ok")?;
        Ok(())
    }
}

fn main() {
    let plugin = Plugin {
        author: "author".to_string(),
        name: "name".to_string(),
    };
    App::new("myapp")
      .version("1.0")
      .about("Does great things!")
      .author("Kevin K.")
      .get_matches();
    println!("{}", plugin);
}

