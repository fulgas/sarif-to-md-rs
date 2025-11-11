use std::{fs, path::Path};

use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> anyhow::Result<()> {
    let content = fs::read_to_string("src/schema.json")?;
    let schema = serde_json::from_str(&content)?;

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(false));
    type_space.add_root_schema(schema)?;

    let contents = rustfmt_wrapper::rustfmt(type_space.to_stream())?;

    let mut out_file = Path::new("src").to_path_buf();
    out_file.push("snyk_container.rs");
    fs::write(out_file, contents)?;
    Ok(())
}
