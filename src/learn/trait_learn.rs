struct Tab {
    nums: i32,
}

trait OpenAble {
    fn open_tab(&self) -> i32;
    fn new(nums: i32) -> Self;
}

impl OpenAble for Tab {
    fn new(nums: i32) -> Self {
        Tab { nums }
    }
    fn open_tab(&self) -> i32 {
        println!("nums {}", self.nums);
        self.nums
    }
}

#[allow(dead_code)]
pub fn init() {
    let t = Tab::new(32);
    t.open_tab();
}
