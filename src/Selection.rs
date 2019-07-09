pub fn sel() {
    let mut arr = [1, 5, 27, 45, 77, 43, 44, 25, 6, 14, 6, 91];
    let l = arr.len();
    let mut min_index;


    for i in 0..(l - 1) {
        min_index = i;

        for j in (i + 1)..l {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        let temp = arr[min_index];
        arr[min_index] = arr[i];
        arr[i] = temp;
    }
    for i in 0..l {
        print!(" {}", arr[i]);
    }
}