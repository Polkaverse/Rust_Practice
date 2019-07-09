pub fn bub() {
    let mut arr = [1, 4, 7, 45, 7, 43, 44, 25, 6, 4, 6, 9];
    let l = arr.len();
    let i = 0;
    let j = 0;


    for i in 0..l {
        for j in 0..(l - i - 1) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    for i in 0..l {
        print!(" {}", arr[i]);
    }
}