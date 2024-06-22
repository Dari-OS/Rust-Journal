pub struct Generic<T> {
    name: String,
    str_type: T,
}

impl<T> Generic<T> {
    pub fn new(any: T) -> Self {
        Self {
            name: "Hello World".to_string(),
            str_type: any,
        }
    }

    pub fn get(&self) -> &T {
        return &self.str_type;
    }
}