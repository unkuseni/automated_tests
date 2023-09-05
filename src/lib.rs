pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    pub  fn area(&self, shape: &Rectangle) -> bool {
        self.height > shape.height && self.width > shape.width
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(19, 10);
        assert_eq!(result, 29);
    }
    #[test]
    fn rectangle_area() {
        let larger = Rectangle {
            width: 20, height: 10,
        };
        let smaller = Rectangle {
            width: 10, height: 5,
        };
        assert!(smaller.area(&larger));
    }
}

