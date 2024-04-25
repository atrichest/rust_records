#[cfg(test)]
mod tests {
    use super::*;
    //Calling the sum method to get the total of all items in the iterator
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}