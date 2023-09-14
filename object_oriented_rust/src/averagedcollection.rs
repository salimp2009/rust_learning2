#[derive(Debug)]
pub struct AveragedCollection {
    // the member are private and can only be accessed by public functions
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
    }
    // add code here
    pub fn remove(&mut self) -> Option<i32> {
        if let Some(result) = self.list.pop() {
            self.update_average();
            Some(result)
        } else {
            None
        }
    }

    pub fn update_average(&mut self) -> f64 {
        let total = self.list.iter().sum::<i32>();
        self.average = total as f64 / self.list.len() as f64;
        self.average
    }
}
