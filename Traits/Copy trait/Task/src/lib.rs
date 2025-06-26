// TODO: implement the necessary traits to make the test compile and pass.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value.wrapping_add(rhs.value))
    }
}
