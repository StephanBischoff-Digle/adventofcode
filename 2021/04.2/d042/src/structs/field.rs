#[derive(Debug)]
pub struct Field {
    val: i32,
    is_marked: bool,
}

impl Field {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            is_marked: false,
        }
    }

    pub fn mark(&mut self, val: i32) {
        if !self.is_marked && self.val == val {
            self.is_marked = true;
        }
    }

    pub fn is_marked(&self) -> bool {
        self.is_marked
    }

    pub fn value(&self) -> i32 {
        self.val
    }
}
