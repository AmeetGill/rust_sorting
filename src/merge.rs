fn sort(input: &mut Vec<i32>){
   sort_util(input, 0, input.len()); 
}

fn sort_util(input: &mut Vec<i32>, start: usize, end: usize) {
    if start == end {
        return ();
    }

    let mid = (start + end)/2;

    sort_util(input, start, mid);
    sort_util(input, mid + 1, end);
    merge(input, start, mid, end);
}

fn merge(input: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    
    let mut counter_1 = start;
    let mut counter_2 = mid + 1;
    let mut i = 0;
    let arr: [i32, end-start + 1] = {};

    while counter_1 <= mid && counter_2 <= end {
        if input[counter_1] >= input[counter_2] {
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
        counter_2 += 2;
        i += 1;
    }
    i = start;
    while i <= end {
        input[i] = arr[i]
        i += 1;
    }
}
