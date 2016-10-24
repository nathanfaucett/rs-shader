use collections::string::String;

use core::hash::{Hash, Hasher};

use shared::Shared;


#[derive(Hash)]
pub struct ShaderData {
    vertex: String,
    fragment: String,
}

#[derive(Clone)]
pub struct Shader {
    data: Shared<ShaderData>,
}

impl Shader {

    pub fn new(vertex: &str, fragment: &str) -> Self {
        Shader {
            data: Shared::new(ShaderData {
                vertex: String::from(vertex),
                fragment: String::from(fragment),
            })
        }
    }

    pub fn vertex(&self) -> &str {
        &self.data.vertex
    }
    pub fn fragment(&self) -> &str {
        &self.data.fragment
    }
}

impl Hash for Shader {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
         self.data.hash(state);
    }
}
