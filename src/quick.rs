pub fn sort(input: &mut Vec<i32> ) {
    sort_util(input,0,input.len()-1);
}

fn sort_util(input: &mut Vec<i32>, start: usize, end: usize) {
	if start == end {
		return ();
	}
    let mut s = start;
    let mut e = end;
    let pivot_elem = input[(s+e)/2];

    while s <= e {   
        while input[s] < pivot_elem { s += 1 }
        while pivot_elem < input[e] { e -= 1 }

        if s <= e {
            let temp = input[s];
            input[s] = input[e];
            input[e] = temp;

            s += 1;
            e -= 1;
        }
    }

    if start <= e { sort_util(input, start, e) }
    if s <= end { sort_util(input, s, end) }

}

// A second function to place pivot at the correct place
// V1 don't do that, that method divide the elements into two subarrays
// one greated than pivot and other less than pivot
pub fn sort_v2(input: &mut Vec<i32>) {
    sort_util_v2(input,0,input.len()-1);
}

fn sort_util_v2(input: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {

        let partition = sort_and_partition(input, start, end);
        // element at partition is at correct position
        if start < partition { sort_util_v2(input, start, partition-1); }
        if end > partition { sort_util_v2(input, partition+1, end); }

    }
}

fn sort_and_partition(input: &mut Vec<i32>, start: usize, end: usize) -> usize {
    
    let pivot = input[start];
    let mut s = start;
    let mut e = end;

    while s < e {
        while input[s] <= pivot { s += 1 }
        while input[e] > pivot { e -= 1 }

        if s < e {
            let temp = input[s];
            input[s] = input[e];
            input[e] = temp;
        }
    }
    // above loop will terminate if s == e or s > e
    // in both cases e will be the position of the pivot
    input[start] = input[e];
    input[e] = pivot;

    return e;

}
