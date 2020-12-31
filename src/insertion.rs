pub fn sort(input: &mut Vec<i32>) {
        
    if input.len() <= 1 {
        return ();
    }

    for i in 0..input.len() {
        let mut k = i;
        while k > 0 && input[k-1]>input[k] {
            let temp = input[k];
            input[k] = input[k-1];
            input[k-1] = temp;
            k -= 1;
        }
    }
}
