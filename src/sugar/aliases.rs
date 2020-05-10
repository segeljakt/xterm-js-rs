use crate::Terminal;
impl Terminal {
    pub fn write(&self, data: &str) {
        self.write_with_callback(data, None);
    }

    pub fn writeln(&self, data: &str) {
        self.writeln_with_callback(data, None);
    }

    pub fn write_utf8(&self, data: Box<[u8]>) {
        self.write_utf8_with_callback(data, None);
    }
}
