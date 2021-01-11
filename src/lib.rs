pub mod insertion;
pub mod quick;
pub mod merge;

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

   #[test]
    fn check_merge_sort_unique_values() {

        let mut array_to_sort = vec![3,6,4,31,12,34,5,1,22,55];

        let mut copy_array_to_sort = array_to_sort.clone();

        copy_array_to_sort.sort();

        merge::sort(&mut array_to_sort);

        assert_eq!(array_to_sort,copy_array_to_sort);

    }


   #[test]
   fn check_merge_sort_repeating_values() {

       let mut array_to_sort = vec![3,6,12,4,31,1,3,5,12,34,5,1,22,55];
       let mut copy_array_to_sort = array_to_sort.clone();

       copy_array_to_sort.sort();
       merge::sort(&mut array_to_sort);

       assert_eq!(array_to_sort,copy_array_to_sort);

   }

   #[test]
   fn check_merge_sort_inplace_unique_values() {

        let mut array_to_sort = vec![3,6,4,31,12,34,5,1,22,55];

        let mut copy_array_to_sort = array_to_sort.clone();

        copy_array_to_sort.sort();

        merge::sort_inplace(&mut array_to_sort);

        assert_eq!(array_to_sort,copy_array_to_sort);

    }


   #[test]
   fn check_merge_sort_inplace_repeating_values() {

       let mut array_to_sort = vec![3,6,12,4,31,1,3,5,12,34,5,1,22,55];
       let mut copy_array_to_sort = array_to_sort.clone();

       copy_array_to_sort.sort();
       merge::sort_inplace(&mut array_to_sort);

       assert_eq!(array_to_sort,copy_array_to_sort);

   }

    #[test]
   fn check_surpasser() {

       let array_to_sort = vec![2,7,5,5,2,7,0,8,1];

       let (index_arr, s_arr) = merge::surpassers(&array_to_sort);


       assert_eq!(index_arr,vec![6,8,4,0,3,2,5,1,7]);
       assert_eq!(s_arr,vec![5,1,2,2,2,1,2,0,0]);


   }


}
