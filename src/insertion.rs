pub fn ins() {
    let mut arr = [1, 14, 77, 45, 7, 3, 44, 25, 46, 4, 6, 59, 89, 119];
    let l = arr.len();
    let mut j = 0;
    let mut key;


    for i in 1..l {
        key = arr[i];
        j = i - 1;

        while j >= 0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key
    }


    for i in 0..l {
        print!(" {}", arr[i]);
    }
}