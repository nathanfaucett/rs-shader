use collections::string::String;
use alloc::arc::Arc;
use core::cell::RefCell;


pub struct ShaderData {
    vertex: String,
    fragment: String,
}

#[derive(Clone)]
pub struct Shader {
    data: Arc<RefCell<ShaderData>>,
}

impl Shader {

    pub fn new(vertex: String, fragment: String) -> Self {
        Shader {
            data: Arc::new(RefCell::new(ShaderData {
                vertex: vertex,
                fragment: fragment,
            }))
        }
    }

    pub fn vertex(&self) -> String {
        self.data.borrow().vertex.clone()
    }
    pub fn fragment(&self) -> String {
        self.data.borrow().fragment.clone()
    }
}
