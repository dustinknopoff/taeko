use taeko_core::{TaekoCoreDatabase};
pub trait TaekoJsonTransformer: TaekoCoreDatabase {

    fn from_text<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned;

    fn from_blob<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::collections::HashMap;
    use taeko_core::{TaekoCoreDatabaseStorage, salsa};


    #[salsa::database(TaekoCoreDatabaseStorage)]
    #[derive(Default)]
    struct JsonLoader {
        storage: salsa::Storage<Self>
    }

    impl salsa::Database for JsonLoader {}

    #[cfg(feature = "json")]
    impl TaekoJsonTransformer for JsonLoader {

        fn from_text<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned {
            serde_json::from_str(&self.text(name)).unwrap()
        }

        fn from_blob<T>(&self, _name: String) -> T where T: serde::de::DeserializeOwned {
            todo!();
        }
    }

    #[cfg(feature = "yaml")]
    impl TaekoJsonTransformer for JsonLoader {

        fn from_text<T>(&self, name: String) -> T where T: serde::de::DeserializeOwned {
            serde_yaml::from_str(&self.text(name)).unwrap()
        }

        fn from_blob<T>(&self, _name: String) -> T where T: serde::de::DeserializeOwned {
            todo!();
        }
    }

    #[cfg(feature = "json")]
    #[test]
    fn json_works() {
        let mut json = JsonLoader::default();
        json.set_text("./test_content/simple.json".to_string(), Arc::new(String::from(r#"{"name": "Taeko"}"#)));
        let actual_map = json.from_text::<HashMap<String, String>>("./test_content/simple.json".to_string());
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), "Taeko".to_string());
        assert_eq!(expected_map, actual_map);
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn yaml_works() {
        let mut json = JsonLoader::default();
        json.set_text("./test_content/simple.json".to_string(), Arc::new(String::from(r#"name: Taeko"#)));
        let actual_map = json.from_text::<HashMap<String, String>>("./test_content/simple.json".to_string());
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), "Taeko".to_string());
        assert_eq!(expected_map, actual_map);
    }
}
