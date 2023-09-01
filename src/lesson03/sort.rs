fn sort() {
    let mut arr_i32 = [45, 23, 78, 22, 3, 14, 12];
    let mut arr_str = [
        "ruby", "java", "BASIC", "nest", "Lisp", "rust", "Perl", "Erlang",
    ];
    println!("待排序的数组为：{:?}", copy_slice(&arr_i32));
    bubble_sort(&mut arr_i32);
    println!("排序后的数组为：{:?}", arr_i32);

    println!("待排序的数组为：{:?}", copy_slice(&arr_str));
    bubble_sort_tmp(&mut arr_str);
    println!("排序后的数组为：{:?}", arr_str);
}
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn copy_slice<T: Clone>(arr: &[T]) -> Vec<T> {
    arr.to_vec()
}

fn bubble_sort_tmp<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
