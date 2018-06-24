#![feature(rust_2018_preview, use_extern_macros)]
use clap;
use clap::clap_app;
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
    let mut zr = clap_app!(zr =>
        (version: crate_version!())
        (author: "Jonathan Dahan <hi@jonathan.is>")
        (about: "z:rat: - zsh plugin manager")
        (@subcommand list => (about: "list plugins") )
        (@subcommand load => (about: "load plugins fresh")
            (@arg plugins: +required +multiple +takes_value "plugin/name[/path/to/file.zsh] [[plugin/name [..]..]")
        )
        (@subcommand update => (about: "update plugins") )
    );
    println!("{}", plugin);
    println!("{:?}", zr)
}

