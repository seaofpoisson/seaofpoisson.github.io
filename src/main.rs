use std::{io, fs, path::Path};
use minijinja::Environment;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Create new _site directory.
    if fs::exists("_site")? {
        fs::remove_dir_all("_site")?;
    }
    fs::create_dir("_site")?;

    // Copy static files.
    copy_dir_all("static", "_site/static")?;

    // Load basic templates.
    let mut env = Environment::new();
    let base = fs::read_to_string("templates/base.html")?;
    env.add_template("base.html", &base).unwrap();

    // Create index.
    let index = fs::read_to_string("templates/index.html")?;
    env.add_template("index.html", &index).unwrap();
    let tmpl = env.get_template("index.html").unwrap();
    fs::write("_site/index.html", tmpl.render(()).unwrap())?;

    println!("Finished generating the Sea of Poisson structures website.");

    Ok(())
}
