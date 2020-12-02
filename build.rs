use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::path::Path;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn main() {
    let file = File::open("spinners.json").expect("spinners.json not in root directory");
    let spinners: serde_json::Value = serde_json::from_reader(file).expect("Found invalid json");
    let spinners = spinners.as_object().unwrap();

    let mut buf = String::new();
    buf.push_str(
        r#"
        pub enum SpinnerType {
    "#,
    );

    let mut spinner_type_to_string = String::new();
    spinner_type_to_string.push_str(
        r#"
        impl ToString for SpinnerType {
            fn to_string(&self) -> String {
                match self {
    "#,
    );

    let mut spinner_get_data = String::new();
    let mut spinners_data = vec![];

    for (key, val) in spinners.iter().map(|(k, v)| (capitalize(k), v)) {
        let enum_variant = format!("    {},\n", &key);
        buf.push_str(&enum_variant);
        spinner_type_to_string.push_str(&format!("{} => \"{}\".to_string(),\n", key, key));
        spinners_data.push((key, val.to_string()));
    }

    let spinners_data_map: HashMap<_, _> = spinners_data.into_iter().collect();
    spinner_type_to_string.push_str("      }\n    }\n}");
    buf.push('}');

    for (key, val) in spinners.iter().map(|(k, v)| (capitalize(k), v)) {
        spinner_get_data.push_str(&format!(
            "{} => Spinner::with_data(SpinnerData::new({}, vec!{})),\n",
            key,
            80,
            serde_json::to_string(&vec![".", "..", "..."]).unwrap()
        ));
    }

    buf.push_str(&format!(
        r#"
        impl Spinner {{
            fn new(spinner_type: SpinnerType) -> Self {{
                match spinner_type {{
                    {}
                }}
            }}
        }}

        use SpinnerType::*;

        {}
    "#,
        spinner_get_data, spinner_type_to_string
    ));

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("spinner.rs");
    println!("{}", buf);

    fs::write(&dest_path, &buf).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
