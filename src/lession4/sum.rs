// 2. 实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option，溢出时返回None
fn sum_u32s(numbers: &[u32]) -> Option<u32> {
    let mut result: Option<u32> = Some(0); // 初始结果为0
    for &num in numbers {
        match result {
            Some(current) => {
                // 使用checked_add方法进行加法并检查溢出
                match current.checked_add(num) {
                    Some(sum) => {
                        result = Some(sum);
                    }
                    None => {
                        // 溢出时返回None
                        result = None;
                        break;
                    }
                }
            }
            None => {
                // 如果之前已经溢出，不再执行加法
                break;
            }
        }
    }
    result
}

fn sum() {
    let numbers = [1, 2, 3, 4, 5];
    let result = sum_u32s(&numbers);

    match result {
        Some(sum) => {
            println!("Sum: {}", sum);
        }
        None => {
            println!("Overflow occurred.");
        }
    }
}
