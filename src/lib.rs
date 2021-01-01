pub mod insertion;
pub mod quick;

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_insertion_sort() {
        
        let mut array_to_sort = vec![3,6,12,4,31,1,3,5,12,34,5,1,22,55];

        let mut copy_array_to_sort = array_to_sort.clone();

        copy_array_to_sort.sort();

        insertion::sort(&mut array_to_sort);

        assert_eq!(array_to_sort,copy_array_to_sort);
    
    }

    #[test]
    fn check_quick_sort_unique_values() {

        let mut array_to_sort = vec![3,6,4,31,12,34,5,1,22,55];

        let mut copy_array_to_sort = array_to_sort.clone();

        copy_array_to_sort.sort();

        quick::sort(&mut array_to_sort);

        assert_eq!(array_to_sort,copy_array_to_sort);

    }


   #[test]
   fn check_quick_sort_repeating_values() {

        let mut array_to_sort = vec![3,6,12,4,31,1,3,5,12,34,5,1,22,55];

        let mut copy_array_to_sort = array_to_sort.clone();

        copy_array_to_sort.sort();
        quick::sort(&mut array_to_sort);

        assert_eq!(array_to_sort,copy_array_to_sort);

    }

   #[test]
   fn check_quick_sort_v2_unique_values() {

        let mut array_to_sort = vec![3,6,4,31,12,34,5,1,22,55];

        let mut copy_array_to_sort = array_to_sort.clone();

        copy_array_to_sort.sort();
        quick::sort_v2(&mut array_to_sort);

        assert_eq!(array_to_sort,copy_array_to_sort);

   }


   #[test]
   fn check_quick_sort_v2_repeating_values() {

        let mut array_to_sort = vec![3,6,12,4,31,1,3,5,12,34,5,1,22,55];

        let mut copy_array_to_sort = array_to_sort.clone();

        copy_array_to_sort.sort();
        quick::sort_v2(&mut array_to_sort);

        assert_eq!(array_to_sort,copy_array_to_sort);

    }

}
