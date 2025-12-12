use schemars::schema::{RootSchema, Schema};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::{env, fs, path::Path};
use typify::{TypeSpace, TypeSpaceSettings};

fn apply_all_patches(spec_value: &mut Value) {
    let schemas = spec_value
        .get_mut("components")
        .and_then(|c| c.get_mut("schemas"))
        .expect("No components.schemas available in spec");

    patch_parameter_provider_dto_properties(schemas);
    patch_recursive_maps(schemas);
}

fn patch_parameter_provider_dto_properties(schemas: &mut Value) {
    let schema_name = "ParameterProviderDTO";
    let field_name = "properties";

    let field_schema = schemas
        .get_mut(schema_name)
        .expect(&format!("FATAL: schema '{}' not to be found", schema_name))
        .get_mut("properties")
        .expect(&format!(
            "FATAL: DTO '{}'.properties not to be found",
            schema_name
        ))
        .get_mut(field_name)
        .expect(&format!(
            "FATAL: El DTO '{}' has no properties called '{}'",
            schema_name, field_name
        ));

    let field_obj = field_schema.as_object_mut().expect(&format!(
        "FATAL: Field '{}' is not a JSON object",
        field_name
    ));
    field_obj.insert("nullable".to_string(), json!(true));
    let add_props = field_obj.get_mut("additionalProperties").expect(&format!(
        "FATAL: Field '{}' has not 'additionalProperties'. Is it a Map?",
        field_name
    ));
    let add_props_obj = add_props
        .as_object_mut()
        .expect("FATAL: 'additionalProperties' is not a JSON object");
    let type_key = "type";
    if add_props_obj.contains_key(type_key) && add_props_obj[type_key] == "string" {
        add_props_obj.insert(
            type_key.to_string(),
            json!(["string", "null"]), // change key
        );
    } else {
        add_props_obj.insert("nullable".to_string(), json!(true));
    }
    println!(
        "cargo:warning={}.{} patched to match components.schemas.ParameterProviderDTO.properties and set nullable properties.",
        schema_name, field_name
    );
}

fn patch_recursive_maps(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let is_string_map = {
                let has_type_object = map.get("type").map_or(false, |t| t == "object");

                let has_string_values = map
                    .get("additionalProperties")
                    .and_then(|ap| ap.as_object())
                    .and_then(|ap_obj| ap_obj.get("type"))
                    .map_or(false, |t| t == "string");

                has_type_object && has_string_values
            };

            if is_string_map {
                map.insert("nullable".to_string(), json!(true));
                if let Some(add_props) = map.get_mut("additionalProperties") {
                    if let Some(add_props_obj) = add_props.as_object_mut() {
                        add_props_obj.insert("type".to_string(), json!(["string", "null"]));
                    }
                }
            }

            for (_, v) in map.iter_mut() {
                patch_recursive_maps(v);
            }
        },
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                patch_recursive_maps(v);
            }
        },
        _ => {},
    }
}

fn main() {
    let spec_path_str = "./spec/nifi/openapi/2.6.0/swagger.json";
    let content = std::fs::read_to_string(spec_path_str).unwrap();
    println!("cargo:rerun-if-changed={}", spec_path_str);
    // extract json
    let mut spec_value: Value =
        serde_json::from_str(&content).expect("Not able to parse specification json");
    // apply pathces
    apply_all_patches(&mut spec_value);
    // extract just components.schemas from spec
    let schemas_value = spec_value
        .get("components")
        .and_then(|c| c.get("schemas"))
        .expect("No components.schemas available in spec");
    // cast components.schemas to BTreeMap<String, Schema>
    let definitions: BTreeMap<String, Schema> = serde_json::from_value(schemas_value.clone())
        .expect("Not a valid schema or cast impossible");
    // root schema
    let mut root_schema = RootSchema::default();
    // put definitions in root schema
    root_schema.definitions = definitions;
    // using typify, create root
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    // add root schema to typify (typify creates ast)
    type_space
        .add_root_schema(root_schema)
        .expect("Error al añadir el root schema a typify");
    // prettyprint of all rust codegen
    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
    // create out_dir folder path
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("openapi_codegen.rs");
    // write in path
    fs::write(&dest_path, contents).unwrap();
}
