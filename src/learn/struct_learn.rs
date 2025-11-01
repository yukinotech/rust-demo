struct Tab {
    nums: i32,
}

impl Tab {
    fn new(nums: i32) -> Self {
        Tab { nums }
    }
    fn open_tab(&self) {
        println!("tab has nums: {}", self.nums);
    }
}

#[allow(dead_code)]
pub fn init() {
    let tab = Tab::new(2);
    tab.open_tab();
}
