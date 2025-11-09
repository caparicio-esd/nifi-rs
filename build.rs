// build.rs
use schemars::schema::{RootSchema, Schema};
use serde_json::{json, Value}; // Necesitamos 'json' para crear el valor 'true'
use std::collections::BTreeMap;
use std::{env, fs, path::Path};
use typify::{TypeSpace, TypeSpaceSettings};

//=================================================================
// 🕵️ SECCIÓN DE PARCHEO DE ESQUEMAS
//=================================================================
// Aquí es donde "arreglamos" la especificación de OpenAPI antes
// de que 'typify' la vea.
//
// "Para que cante más", agrupamos todos los parches aquí.
//=================================================================

/// Función principal que aplica TODOS los parches a la especificación.
/// Si la API de NiFi tiene más errores, añada más llamadas aquí.
fn apply_all_patches(spec_value: &mut Value) {
    // 1. Extraemos el objeto 'schemas' como mutable
    let schemas = spec_value
        .get_mut("components")
        .and_then(|c| c.get_mut("schemas"))
        .expect("La especificación no tiene 'components.schemas'");

    // --- Lista de Parches Quirúrgicos ---
    patch_parameter_provider_dto_properties(schemas);
    // patch_otro_dto_roto(schemas); // <-- Los futuros parches irían aquí
    // ...
}

/// PARCHE 1: Los 'properties' del ParameterProviderDTO son doblemente nulables.
///
/// * Razón: La API devuelve `null` para el propio campo 'properties' Y
///   también `null` para los *valores* dentro del mapa (ej. "prop": null).
/// * Objetivo: Generar el tipo `Option<BTreeMap<String, Option<String>>>`.
// Reemplace la función de parcheo anterior por esta
// Reemplace la función de parcheo por esta versión "paranoica"
fn patch_parameter_provider_dto_properties(schemas: &mut Value) {
    let schema_name = "ParameterProviderDTO";
    let field_name = "properties";

    println!("cargo:warning=INTENTANDO PARCHE (MODO 'REPLACE') EN {}.{}...", schema_name, field_name);

    // --- CADENA DE NAVEGACIÓN ---
    let field_schema = schemas
        .get_mut(schema_name)
        .expect(&format!("FATAL: No se encontró el esquema '{}'", schema_name))
        .get_mut("properties")
        .expect(&format!("FATAL: El DTO '{}' no tiene un mapa 'properties'", schema_name))
        .get_mut(field_name)
        .expect(&format!("FATAL: El DTO '{}' no tiene un campo '{}'", schema_name, field_name));

    let field_obj = field_schema
        .as_object_mut()
        .expect(&format!("FATAL: El campo '{}' no es un objeto JSON", field_name));

    // --- PARCHE A (Externo): Hacer el campo nulable ---
    field_obj.insert("nullable".to_string(), json!(true));
    println!("cargo:warning=Parche A (externo) aplicado.");

    // --- PARCHE B (Interno): Reemplazar el tipo para que sea nulable ---
    let add_props = field_obj
        .get_mut("additionalProperties")
        .expect(&format!("FATAL: El campo '{}' no tiene 'additionalProperties'. ¿Es un mapa?", field_name));

    let add_props_obj = add_props
        .as_object_mut()
        .expect("FATAL: 'additionalProperties' no es un objeto JSON");

    // --- ¡AQUÍ ESTÁ LA LÓGICA CLAVE! ---
    let type_key = "type";
    if add_props_obj.contains_key(type_key) && add_props_obj[type_key] == "string" {
        // Reemplazamos "type": "string"
        // con "type": ["string", "null"]
        add_props_obj.insert(
            type_key.to_string(),
            json!(["string", "null"]) // El estándar JSON Schema para nulable
        );
        println!("cargo:warning=Parche B (interno) REEMPLAZADO con type:['string', 'null'].");
    } else {
        // Si no era type: "string", solo añadimos nullable (plan B)
        add_props_obj.insert("nullable".to_string(), json!(true));
        println!("cargo:warning=Parche B (interno) AÑADIDO (nullable: true).");
    }

    println!("cargo:warning=ÉXITO: Parche (MODO 'REPLACE') aplicado a {}.{}", schema_name, field_name);
}
//=================================================================
// ⚙️ LÓGICA PRINCIPAL DE BUILD.RS
//=================================================================
// Esta parte ahora está limpia y solo se preocupa de la
// orquestación, no de los detalles sucios.
//=================================================================

fn main() {
    let spec_path_str = "./spec/nifi/openapi/2.6.0/swagger.json";
    let content = std::fs::read_to_string(spec_path_str).unwrap();
    println!("cargo:rerun-if-changed={}", spec_path_str);

    // 1. Parsear como un valor JSON genérico mutable
    let mut spec_value: Value = serde_json::from_str(&content)
        .expect("Error al parsear la especificación JSON");

    // 2. ----------------------------------------------------
    //    ¡AQUÍ ES DONDE "CANTA"!
    //    Llamamos a nuestra función de parcheo aislada.
    apply_all_patches(&mut spec_value);
    //    ----------------------------------------------------

    // 3. Extraer el objeto "components.schemas" (ahora parcheado)
    let schemas_value = spec_value
        .get("components")
        .and_then(|c| c.get("schemas"))
        .expect("La especificación no contiene 'components.schemas'");

    // 4. Deserializar el JSON parcheado en el BTreeMap que 'schemars' espera
    let definitions: BTreeMap<String, Schema> =
        serde_json::from_value(schemas_value.clone())
            .expect("Error al deserializar 'components.schemas' en BTreeMap<String, Schema>");

    // 5. Crear el RootSchema
    let mut root_schema = RootSchema::default();
    root_schema.definitions = definitions;

    // 6. Crear el TypeSpace
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));

    // 7. Añadir el RootSchema.
    type_space
        .add_root_schema(root_schema)
        .expect("Error al añadir el root schema a typify");

    // 8. Escribir el archivo
    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("openapi_codegen.rs");

    fs::write(&dest_path, contents).unwrap();
}