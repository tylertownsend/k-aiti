// use std::collections::HashMap;

// pub struct ModelRegistry {
//     models: HashMap<String, Model>,
// }

// impl ModelRegistry {
//     fn new() -> ModelRegistry {
//         ModelRegistry {
//             models: HashMap::new(),
//         }
//     }
//     fn register_model(&mut self, id: &str, model: Model) {
//         let model = Model::new(id, config);
//         self.models.insert(id.to_string(), model);
//     }

//     fn get_model(&self, id: &str) -> Option<&Model> {
//         self.models.get(id)
//     }

//     // Methods to activate or switch modes
//     fn activate_chat_mode(&self, id: &str) {
//         if let Some(model) = self.get_model(id) {
//             model.chat();
//         }
//     }

//     // Similar methods for completion and generate image modes
// }