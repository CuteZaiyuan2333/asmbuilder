pub mod backends;
pub mod symphony_playground;

pub trait Backend: Default{
    fn build_asm_body(self) -> Result<String, ()>;
}

pub struct Block<B: Backend>{
    label: String,
    ir: B,
}
impl<B: Backend> Block<B> {
    pub fn new(name: String) -> Self {
        Self{
            label: name,
            ir: B::default(),
        }
    }
    pub fn build(self) -> Result<String, ()>{
        Ok(self.label + ":\n" + &self.ir.build_asm_body().unwrap())
    }
}