pub struct CodeGenerator(Vec<u8>);

impl CodeGenerator {
    fn bytes(&self) -> &Vec<u8> {
        &self.0
    }

    fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn with_capacity(size: usize) -> Self {
        Self(Vec::with_capacity(size))
    }

    pub fn push(&mut self, value: u8) {
        self.bytes_mut().push(value)
    }

    pub fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        self.bytes_mut().extend(iter)
    }

    pub fn into_code(self) -> String {
        unsafe { String::from_utf8_unchecked(self.into_bytes()) }
    }
}
