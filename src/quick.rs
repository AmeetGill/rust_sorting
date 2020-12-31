pub fn sort(input: &mut Vec<i32> ) {
    sort_util(input,0,input.len()-1);
}

fn sort_util(input: &mut Vec<i32>, start: usize, end: usize) {
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
