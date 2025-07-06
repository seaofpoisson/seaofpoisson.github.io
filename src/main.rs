use std::{collections::{BTreeMap, HashMap}, io, path::{Path, PathBuf}};
use fs_err as fs;
use toml::Table;
use minijinja::{context, Environment};

fn copy_dir_all(src: impl AsRef<Path> + Into<PathBuf>, dst: impl AsRef<Path> + Into<PathBuf>) -> io::Result<()> {
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
    if std::fs::exists("_site")? {
        fs::remove_dir_all("_site")?;
    }
    fs::create_dir("_site")?;
    fs::create_dir("_site/poisson_structures/")?;
    fs::create_dir("_site/graph_cohomology/")?;

    // Copy static files.
    copy_dir_all("static", "_site/static")?;

    // Collect the names of graph cohomology classes.
    let mut graph_cohomology_classes = BTreeMap::new();
    let mut graph_cohomology_class_to_deformations: HashMap<String, BTreeMap<String, String>> = HashMap::new();
    for entry in fs::read_dir("data/graph_cohomology/")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_str().unwrap().to_owned();
            let toml_path = name.clone() + "/" + &name + ".toml";
            let toml_str = fs::read_to_string("data/graph_cohomology/".to_owned() + &toml_path)?;
            let toml_table = toml_str.parse::<Table>().unwrap();
            graph_cohomology_classes.insert(name.clone(), toml_table["name"].as_str().unwrap().to_string());
            graph_cohomology_class_to_deformations.insert(name.clone(), BTreeMap::new());
        }
    }

    // Collect the names of Poisson structures.
    let mut poisson_structures = BTreeMap::new();
    for entry in fs::read_dir("data/poisson_structures")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_str().unwrap().to_owned();
            let toml_path = name.clone() + "/" + &name + ".toml";
            let toml_str = fs::read_to_string("data/poisson_structures/".to_owned() + &toml_path)?;
            let toml_table = toml_str.parse::<Table>().unwrap();
            poisson_structures.insert(name, toml_table["name"].as_str().unwrap().to_string());
        }
    }

    // Load basic templates.
    let mut env = Environment::new();
    let base = fs::read_to_string("templates/base.html")?;
    env.add_template("base.html", &base).unwrap();
    let index = fs::read_to_string("templates/index.html")?;
    env.add_template("index.html", &index).unwrap();
    let inline_reference = fs::read_to_string("templates/inline_reference.html")?;
    env.add_template("inline_reference.html", &inline_reference).unwrap();
    let references = fs::read_to_string("templates/references.html")?;
    env.add_template("references.html", &references).unwrap();
    let poisson_structure = fs::read_to_string("templates/poisson_structure.html")?;
    env.add_template("poisson_structure.html", &poisson_structure).unwrap();
    let graph_cohomology_class = fs::read_to_string("templates/graph_cohomology_class.html")?;
    env.add_template("graph_cohomology_class.html", &graph_cohomology_class).unwrap();
    let poisson_deformation = fs::read_to_string("templates/poisson_deformation.html")?;
    env.add_template("poisson_deformation.html", &poisson_deformation).unwrap();

    // Create page for references.
    let references_template = env.get_template("references.html").unwrap();
    let references_toml_path = "data/references.toml";
    let references_toml_str = fs::read_to_string(&references_toml_path)?;
    let references_toml_table = references_toml_str.parse::<Table>().unwrap();
    let ctx= context! {
        references => references_toml_table["references"]
    };
    fs::write("_site/references.html", references_template.render(ctx).unwrap())?;

    // Create pages for Poisson structures.
    let poisson_template = env.get_template("poisson_structure.html").unwrap();
    let poisson_deformation_template = env.get_template("poisson_deformation.html").unwrap();
    for entry in fs::read_dir("data/poisson_structures")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let slug = entry.file_name().to_str().unwrap().to_owned();
            // Read Poisson structure TOML table.
            let toml_path = "data/poisson_structures/".to_owned() + &slug + "/" + &slug + ".toml";
            let toml_str = fs::read_to_string(&toml_path)?;
            let toml_table = toml_str.parse::<Table>().unwrap();
            // Create pages for universal deformations of Poisson structures.
            let mut deformations = BTreeMap::new();
            if let Ok(paths) = fs::read_dir("data/poisson_structures/".to_owned() + &slug + "/universal_deformations") {
                for path in paths {
                    // TODO: Clean up this inane error handling.
                    if let Ok(ref path2) = path {
                        if let Ok(file_type) = path2.file_type() {
                            if !file_type.is_dir() {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    let dir = path?;
                    let graph_cohomology_class_slug = dir.file_name().to_str().unwrap().to_owned();
                    let graph_cohomology_class_name = graph_cohomology_classes.get(&graph_cohomology_class_slug).unwrap();
                    deformations.insert(graph_cohomology_class_slug.clone(), graph_cohomology_class_name.clone());
                    graph_cohomology_class_to_deformations.get_mut(&graph_cohomology_class_slug).unwrap().insert(slug.to_owned() +  "_deformation_from_" + &graph_cohomology_class_slug, toml_table["name"].as_str().unwrap().to_string());
                    // Read deformation TOML table.
                    let deformation_toml_path = "data/poisson_structures/".to_owned() + &slug + "/universal_deformations/" + &graph_cohomology_class_slug + "/" + &slug + "_deformation_from_" + &graph_cohomology_class_slug + ".toml";
                    let deformation_toml_str = fs::read_to_string(&deformation_toml_path)?;
                    let deformation_toml_table = deformation_toml_str.parse::<Table>().unwrap();
                    // Check for references.
                    let references = deformation_toml_table.get("references");
                    // Check for coboundary data.
                    let is_known_coboundary: bool;
                    let vector_field_potential_formula: String;
                    if deformation_toml_table.contains_key("vector_field_potential") {
                        is_known_coboundary = true;
                        vector_field_potential_formula = fs::read_to_string("data/poisson_structures/".to_owned() + &slug + "/universal_deformations/" + &graph_cohomology_class_slug + "/" + &deformation_toml_table["vector_field_potential"]["txt"].as_str().unwrap().to_string())?;
                    } else {
                        is_known_coboundary = false;
                        vector_field_potential_formula = String::new();
                    }
                    // Check for Hamiltonian data (specific to 2D)
                    let hamiltonian_2d_formula: String;
                    if deformation_toml_table.contains_key("hamiltonian_2d") {
                        hamiltonian_2d_formula = fs::read_to_string("data/poisson_structures/".to_owned() + &slug + "/universal_deformations/" + &graph_cohomology_class_slug + "/" + deformation_toml_table["hamiltonian_2d"]["txt"].as_str().unwrap())?;
                    } else {
                        hamiltonian_2d_formula = String::new();
                    }
                    // Create Poisson deformation page.
                    let ctx = context!{
                        poisson_structure_slug => &slug,
                        poisson_structure_name => toml_table["name"],
                        poisson_structure_name_plain => toml_table["name_plain"],
                        graph_cohomology_class_slug => &graph_cohomology_class_slug,
                        graph_cohomology_class_name => &graph_cohomology_class_name,
                        references => references,
                        definition_code_gcaops => fs::read_to_string("data/poisson_structures/".to_owned() + &slug + "/universal_deformations/" + &graph_cohomology_class_slug + "/" + &deformation_toml_table["deformation_term"]["code"].as_str().unwrap().to_string())?,
                        definition_formula => fs::read_to_string("data/poisson_structures/".to_owned() + &slug + "/universal_deformations/" + &graph_cohomology_class_slug + "/" + &deformation_toml_table["deformation_term"]["txt"].as_str().unwrap().to_string())?,
                        is_known_coboundary => is_known_coboundary,
                        vector_field_potential_formula => vector_field_potential_formula,
                        hamiltonian_2d_formula => hamiltonian_2d_formula
                    };
                    fs::write("_site/poisson_structures/".to_owned() + &slug + "_deformation_from_" + &graph_cohomology_class_slug + ".html", poisson_deformation_template.render(ctx).unwrap())?;
                }
            }
            // Create Poisson structure page.
            let ctx = context!{
                slug => slug,
                name => toml_table["name"],
                name_plain => toml_table["name_plain"],
                alternative_names => toml_table["alternative_names"],
                tags => toml_table["tags"],
                references => toml_table["references"],
                definition_formula => toml_table["definition_formula"],
                definition_code_gcaops => fs::read_to_string("data/poisson_structures/".to_owned() + &slug + "/" + &slug + ".sage")?,
                deformations => deformations.iter().collect::<Vec<(&String, &String)>>(),
            };
            fs::write("_site/poisson_structures/".to_owned() + &slug + ".html", poisson_template.render(ctx).unwrap())?;
        }
    }

    // Create pages for graph cohomology classes.
    let graph_cohomology_class_template = env.get_template("graph_cohomology_class.html").unwrap();
    for entry in fs::read_dir("data/graph_cohomology/")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let slug = entry.file_name().to_str().unwrap().to_owned();
            // Copy JSON file.
            let json_path = "data/graph_cohomology/".to_owned() + &slug + "/" + &slug + ".json";
            fs::copy(json_path, "_site/graph_cohomology/".to_owned() + &slug + ".json")?;
            // Read TOML table.
            let toml_path = slug.clone() + "/" + &slug + ".toml";
            let toml_str = fs::read_to_string("data/graph_cohomology/".to_owned() + &toml_path)?;
            let toml_table = toml_str.parse::<Table>().unwrap();
            // Create graph cohomology class page.
            let ctx = context!{
                name => toml_table["name"],
                name_plain => toml_table["name_plain"],
                alternative_names => toml_table["alternative_names"],
                num_vertices => toml_table["num_vertices"],
                num_edges => toml_table["num_edges"],
                tags => toml_table["tags"],
                references => toml_table["references"],
                definition_code_gcaops => fs::read_to_string("data/graph_cohomology/".to_owned() + &slug + "/" + &slug + ".sage")?,
                json_filename => slug.clone() + ".json",
                deformations => graph_cohomology_class_to_deformations[&slug].iter().collect::<Vec<(&String, &String)>>(),
            };
            fs::write("_site/graph_cohomology/".to_owned() + &slug + ".html", graph_cohomology_class_template.render(ctx).unwrap())?;
        }
    }

    // Create index page.
    let index_template = env.get_template("index.html").unwrap();
    fs::write("_site/index.html", index_template.render(context!{
        poisson_structures => poisson_structures.iter().collect::<Vec<(&String, &String)>>(),
        graph_cohomology_classes => graph_cohomology_classes.iter().collect::<Vec<(&String, &String)>>(),
    }).unwrap())?;

    println!("Finished generating the Sea of Poisson structures website.");

    Ok(())
}
