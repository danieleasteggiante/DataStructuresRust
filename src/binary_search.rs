pub struct BinarySearch <T> {
    pub data: Vec<T>,
}

impl<T: PartialOrd> BinarySearch<T> {
    pub fn new(data: Vec<T>) -> Self {
        BinarySearch { data }
    }
    
    pub fn search(&self, target: T) -> Option<usize> {
        let mut middle = 0;
        let mut left = 0;
        let mut right = self.data.len() - 1;
        while left <= right {
            middle = (middle + right) / 2;
            if target < self.data[middle] {
                right = middle - 1;
                continue;
            }
            if target > self.data[middle] {
                left = middle + 1;
                continue;
            }
            if target == self.data[middle] {
                return Some(middle);
            }
        }
        None
    }
}