use std::{io, fs, path::Path};
use toml::Table;
use minijinja::{Environment, context};

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
    fs::create_dir("_site/poisson_structures/")?;
    fs::create_dir("_site/graph_cohomology/")?;

    // Copy static files.
    copy_dir_all("static", "_site/static")?;

    // Load basic templates.
    let mut env = Environment::new();
    let base = fs::read_to_string("templates/base.html")?;
    env.add_template("base.html", &base).unwrap();
    let index = fs::read_to_string("templates/index.html")?;
    env.add_template("index.html", &index).unwrap();
    let poisson_structure = fs::read_to_string("templates/poisson_structure.html")?;
    env.add_template("poisson_structure.html", &poisson_structure).unwrap();
    let graph_cohomology_class = fs::read_to_string("templates/graph_cohomology_class.html")?;
    env.add_template("graph_cohomology_class.html", &graph_cohomology_class).unwrap();

    // Create index.
    let index_template = env.get_template("index.html").unwrap();
    fs::write("_site/index.html", index_template.render(()).unwrap())?;

    // Create Poisson structures.
    let poisson_template = env.get_template("poisson_structure.html").unwrap();
    for entry in fs::read_dir("data/poisson_structures")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_str().unwrap().to_owned();
            let toml_path = name.clone() + "/" + &name + ".toml";
            let toml_str = fs::read_to_string("data/poisson_structures/".to_owned() + &toml_path)?;
            let toml_table = toml_str.parse::<Table>().unwrap();
            let ctx = context!{
                name => toml_table["name"],
                name_plain => toml_table["name_plain"],
                alternative_names => toml_table["alternative_names"],
                tags => toml_table["tags"],
                references => toml_table["references"],
                definition_formula => toml_table["definition_formula"],
                definition_code_gcaops => fs::read_to_string("data/poisson_structures/".to_owned() + &name + "/" + &name + ".sage")?,
            };
            fs::write("_site/poisson_structures/".to_owned() + &name + ".html", poisson_template.render(ctx).unwrap())?;
        }
    }

    // Create graph cohomology classes.
    let graph_cohomology_class_template = env.get_template("graph_cohomology_class.html").unwrap();
    for entry in fs::read_dir("data/graph_cohomology/")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_str().unwrap().to_owned();
            let toml_path = name.clone() + "/" + &name + ".toml";
            let toml_str = fs::read_to_string("data/graph_cohomology/".to_owned() + &toml_path)?;
            let toml_table = toml_str.parse::<Table>().unwrap();
            let ctx = context!{
                name => toml_table["name"],
                name_plain => toml_table["name_plain"],
                alternative_names => toml_table["alternative_names"],
                num_vertices => toml_table["num_vertices"],
                num_edges => toml_table["num_edges"],
                tags => toml_table["tags"],
                references => toml_table["references"],
                definition_code_gcaops => fs::read_to_string("data/graph_cohomology/".to_owned() + &name + "/" + &name + ".sage")?,
                representative_json => fs::read_to_string("data/graph_cohomology/".to_owned() + &name + "/" + &name + ".json")?,
            };
            fs::write("_site/graph_cohomology/".to_owned() + &name + ".html", graph_cohomology_class_template.render(ctx).unwrap())?;
        }
    }

    println!("Finished generating the Sea of Poisson structures website.");

    Ok(())
}
