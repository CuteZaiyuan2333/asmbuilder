pub mod backends;
pub mod symphony_playground;

pub trait Backend: Default{
    fn build_asm_body(self) -> Result<String, ()>;
}

#[derive(Clone)]
pub struct Label(pub String);

pub struct Block<B: Backend>{
    label: Label,
    ir: B,
}
impl<B: Backend> Block<B> {
    pub fn new(name: String) -> Self {
        Self{
            label: Label(name),
            ir: B::default(),
        }
    }
    pub fn build(self) -> Result<String, ()>{
        Ok(self.label.0 + ":\n" + &self.ir.build_asm_body().unwrap())
    }
}