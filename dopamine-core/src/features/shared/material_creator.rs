use crate::game::material_system::{Material, MaterialSystem};
use crate::game::KeyValues;

pub struct MaterialCreator {
    #[allow(clippy::vec_box)] // We need a pointer indenpendent of vector reallocation
    kvs: Vec<Box<KeyValues>>,
}

impl MaterialCreator {
    pub fn new() -> Self {
        Self { kvs: Vec::new() }
    }

    pub fn shader(&mut self, shader: &str) -> IntermediateMaterial {
        self.kvs.push(KeyValues::new_boxed(shader));
        IntermediateMaterial::new(self.kvs.last_mut().unwrap())
    }
}

#[must_use = "IntermediateMaterial does nothing until you bind it"]
pub struct IntermediateMaterial<'a> {
    kv: &'a mut KeyValues,
}

impl<'a> IntermediateMaterial<'a> {
    pub fn new(kv: &'a mut KeyValues) -> Self {
        Self { kv }
    }

    pub fn string(self, key: &str, value: &str) -> Self {
        self.kv.set_string(key, value);
        self
    }

    pub fn bind<'b>(self, name: &str, material_system: &'b MaterialSystem) -> Option<&'b Material> {
        material_system.create_material(name, self.kv)
    }
}
