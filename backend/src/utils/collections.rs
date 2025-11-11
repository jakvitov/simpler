
///Return mutable reference to the row1 and immutable reference to row2
pub fn get_two_rows_mut<T>(matrix: &mut Vec<Vec<T>>, row1: usize, row2: usize)
                       -> (&mut Vec<T>, &Vec<T>) {
    debug_assert!(row1 != row2);
    debug_assert!(row1 < matrix.len() && row2 < matrix.len());

    if row1 < row2 {
        let (left, right) = matrix.split_at_mut(row2);
        (&mut left[row1], &right[0])
    } else {
        let (left, right) = matrix.split_at_mut(row1);
        (&mut right[0], &left[row2])
    }
}

///Return mutable reference to the element1 and immutable reference to element2
pub fn get_two_elements_mut<T>(vec: &mut Vec<T>, element1: usize, element2: usize)
    -> (&mut T, &T) {
    debug_assert!(element1 != element2);
    debug_assert!(element1 < vec.len() && element2 < vec.len());

    if element1 < element2 {
        let (left, right) = vec.split_at_mut(element2);
        (&mut left[element1], &right[0])
    } else {
        let (left, right) = vec.split_at_mut(element1);
        (&mut right[0], &left[element2])
    }
}