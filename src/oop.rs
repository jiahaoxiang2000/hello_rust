//! the oop module contains the object oriented programming examples


pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {


    /// create a new AveragedCollection
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }

    /// add a value to the list, and update the average
    /// # example
    /// ```
    /// use hello_rust_xjh::oop::AveragedCollection;
    /// let mut ac = AveragedCollection::new();
    /// ac.add(1);
    /// ac.add(2);
    /// assert_eq!(ac.average(), 1.5);
    /// ```
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// remove a value from the list, and update the average
    /// # example
    /// ```
    /// use hello_rust_xjh::oop::AveragedCollection;
    /// let mut ac = AveragedCollection::new();
    /// ac.add(1);
    /// ac.add(2);
    /// assert_eq!(ac.remove(), Some(2));
    /// assert_eq!(ac.average(), 1.0);
    /// ```
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    /// Returns the average of this [`AveragedCollection`].
    /// 
    /// # example
    /// ```
    /// use hello_rust_xjh::oop::AveragedCollection;
    /// let mut ac = AveragedCollection::new();
    /// ac.add(1);
    /// ac.add(2);  
    /// assert_eq!(ac.average(), 1.5);
    /// ```
    pub fn average(&self) -> f64 {
        self.average
    }

    
    /// update average of this [`AveragedCollection`]. this is private function, so no test for it, because this function used by the public function, so also tested by the public function
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}