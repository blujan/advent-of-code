#[derive(Debug)]
pub struct Condition {
    pub dest: String,
    pub attr: char,
    pub operator: std::cmp::Ordering,
    pub value: i32,
}

impl Default for Condition {
    fn default() -> Self {
        Condition {
            dest: Default::default(),
            attr: Default::default(),
            operator: std::cmp::Ordering::Equal,
            value: Default::default(),
        }
    }
}
