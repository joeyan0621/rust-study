fn main() {
    let mut arr_i32 = [45, 23, 78, 23, 3, 14];
    let mut arr_str = [
        "ruby", "java", "BASIC", "nest", "Lisp", "rust", "Perl", "Erlang",
    ];
    bubble_sort(&mut arr_i32);
    bubble_sort_tmp(&mut arr_str);
    println!("ordered: {:?}", arr_i32);
    println!("ordered: {:?}", arr_str);
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
