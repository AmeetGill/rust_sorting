pub fn sort(input: &mut Vec<i32>) {
   sort_util(input, 0, input.len()-1,false); 
}

pub fn sort_inplace(input: &mut Vec<i32>) {
    sort_util(input, 0, input.len()-1,true);
}

fn sort_util(input: &mut Vec<i32>, start: usize, end: usize, inplace: bool) {
    if start < end {

        let mid = (start + end)/2;

        sort_util(input, start, mid, inplace);
        sort_util(input, mid + 1, end, inplace);
        if inplace {
            merge_inplace(input, start, mid, end);
        } else {
            merge(input, start, mid, end);
        }

    }
}

fn merge(input: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    
    let mut counter_1 = start;
    let mut counter_2 = mid + 1;
    let mut i: usize = 0; 
    let mut arr = vec![0; end - start + 1];

    while counter_1 <= mid && counter_2 <= end {
        if input[counter_1] <= input[counter_2]{
            arr[i] = input[counter_1];
            counter_1 += 1;
        }else {
            arr[i] = input[counter_2];
            counter_2 += 1;
        }   
        i += 1;
    }

    while counter_1 <= mid {
        arr[i] = input[counter_1];
        counter_1 += 1;
        i += 1;
    }

    while counter_2 <= end {
        arr[i] = input[counter_2];
        counter_2 += 1;
        i += 1;
    }

    i = start;
    let mut j = 0;
    while i <= end {
        input[i] = arr[j];
        i += 1;
        j += 1;
    }
}


fn merge_inplace(input: &mut Vec<i32>, start: usize, mid: usize, end: usize) {

    let mut counter_1 = start;
    let mut mid_1 = mid;
    let mut counter_2 = mid + 1;
   // [11,12,20,3,9,31]
   // [11,11,12,20,9,31]

    while counter_1 <= mid_1 && counter_2 <= end {
        if input[counter_1] <= input[counter_2] {
            counter_1 += 1;
        } else {
            let temp = input[counter_2];
            right_shift_by_1(input, counter_1, counter_2);
            mid_1 += 1;
            input[counter_1] = temp;
            counter_2 += 1;
            counter_1 += 1;
        }
    }
}

fn right_shift_by_1(input: &mut Vec<i32>, start: usize, end: usize) {
    if start > end {
        panic!("cannot right shift this array");
    }

    let mut i = end;
    while i > start { 
        input[i] = input[i-1];
        i -= 1;
    }
    
}
