#[derive(Default)]
pub struct Counter(u32);

impl Counter {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
