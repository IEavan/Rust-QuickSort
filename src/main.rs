// Quicksort implementation to get used to the idea of rust
pub fn qsort<F: PartialOrd + Clone>(list: &mut [F]) {
    // Check base case of the recursion
    if list.len() <= 1 {return}

    // Stage 1: Pick a pivot using Lomuto paritioning scheme
    let pivot_index: usize = list.len() - 1;

    // Stage 2: Partition based on pivot
    let mut i = 0;
    for j in 0..list.len() {
        if list[j] < list[pivot_index] {
            swap(list, j, i);
            i += 1;
        }
    }
    swap(list, i, pivot_index);

    // Stage 3: Recurse
    qsort(&mut list[0..i]);
    qsort(&mut list[(i + 1)..(pivot_index + 1)]);
}

fn swap<F: Clone>(list: &mut [F], position1: usize, position2: usize) {
    if position1 == position2 {return}
    let temp: F = (&list[position1]).clone();
    list[position1] = (&list[position2]).clone();
    list[position2] = temp;
}

fn main() {
    let mut list = [2,3,4,1,0];
    qsort(&mut list);
    println!("QuickSorting [2,3,4,1,0] results in {:?}", list);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qsort() {
        let mut list = [3,7,4,9,0,4,2];
        qsort(&mut list);
        assert_eq!(list, [0,2,3,4,4,7,9]);
    }

    #[test]
    fn test_swap() {
        let mut list = [1,2,3,4,5];
        swap(&mut list, 0, 2);
        assert_eq!(list, [3,2,1,4,5]);
    }
}
