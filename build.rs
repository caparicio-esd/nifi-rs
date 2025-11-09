// build.rs
use schemars::schema::{RootSchema, Schema};
use serde_json::Value;
// ¡CAMBIO 1: Importa BTreeMap en lugar de HashMap!
use std::collections::BTreeMap;
use std::{env, fs, path::Path};
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let spec_path_str = "./spec/nifi/openapi/2.6.0/swagger.json";
    let content = std::fs::read_to_string(spec_path_str).unwrap();
    println!("cargo:rerun-if-changed={}", spec_path_str);

    // 1. Parsear el OpenAPI como un valor JSON genérico
    let spec_value: Value = serde_json::from_str(&content)
        .expect("Error al parsear la especificación JSON");

    // 2. Extraer el objeto "components.schemas"
    let schemas_value = spec_value
        .get("components")
        .and_then(|c| c.get("schemas"))
        .expect("La especificación no contiene 'components.schemas'");

    // 3. ¡CAMBIO 2: Deserializar en un BTreeMap!
    let definitions: BTreeMap<String, Schema> =
        serde_json::from_value(schemas_value.clone())
            .expect("Error al deserializar 'components.schemas' en BTreeMap<String, Schema>");

    // 4. Crear un 'RootSchema' y poner nuestras definiciones en él.
    let mut root_schema = RootSchema::default();
    root_schema.definitions = definitions; // <-- ¡Ahora los tipos coinciden!

    // 5. Crear el TypeSpace
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));

    // 6. Añadir el RootSchema.
    type_space
        .add_root_schema(root_schema)
        .expect("Error al añadir el root schema a typify");

    // 7. Escribir el archivo de salida
    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("openapi_codegen.rs");

    fs::write(&dest_path, contents).unwrap();
}