use taeko_core::{TaekoCoreDatabase};
pub trait TaekoSerialTransformer: TaekoCoreDatabase {

    fn from_text<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned;

    fn from_blob<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned;
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::collections::HashMap;
    use taeko_core::{TaekoCoreDatabaseStorage, salsa};

    mod json {
        use super::*;
        #[salsa::database(TaekoCoreDatabaseStorage)]
        #[derive(Default)]
        pub struct JsonLoader {
            storage: salsa::Storage<Self>
        }

        impl salsa::Database for JsonLoader {}

        #[cfg(feature = "json")]
        impl TaekoSerialTransformer for JsonLoader {

            fn from_text<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned {
                serde_json::from_str(&self.text(name)).unwrap()
            }

            fn from_blob<T>(&self, _name: String) -> T where T: serde::de::DeserializeOwned {
                todo!();
            }
        }
    }

    mod yaml {
        use super::*;
        #[salsa::database(TaekoCoreDatabaseStorage)]
        #[derive(Default)]
        pub struct YamlLoader {
            storage: salsa::Storage<Self>
        }

        impl salsa::Database for YamlLoader {}


        #[cfg(feature = "yaml")]
        impl TaekoSerialTransformer for YamlLoader {

            fn from_text<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned {
                serde_yaml::from_str(&self.text(name)).unwrap()
            }

            fn from_blob<T>(&self, _name: String) -> T where T: serde::de::DeserializeOwned {
                todo!();
            }
        }
    }

    #[cfg(feature = "json")]
    #[test]
    fn json_works() {
        let mut json = json::JsonLoader::default();
        json.set_text("./test_content/simple.json".to_string(), Arc::new(String::from(r#"{"name": "Taeko"}"#)));
        let actual_map = json.from_text::<HashMap<String, String>>("./test_content/simple.json".to_string());
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), "Taeko".to_string());
        assert_eq!(expected_map, actual_map);
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn yaml_works() {
        let mut json = yaml::YamlLoader::default();
        json.set_text("./test_content/simple.json".to_string(), Arc::new(String::from(r#"name: Taeko"#)));
        let actual_map = json.from_text::<HashMap<String, String>>("./test_content/simple.json".to_string());
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), "Taeko".to_string());
        assert_eq!(expected_map, actual_map);
    }
}
