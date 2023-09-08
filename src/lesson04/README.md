## homework04

红绿灯 light.rs

```rust
// 1. 为枚举交通信号灯实现一个trait，trait中包含一个返回时间的方法，不同的灯持续的时间不同
enum TrafficLight {
    Red,
    Green,
    Yellow,
}
trait TrafficLightDuration {
    fn duration(&self) -> u32;
}
impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Green => 50,
            TrafficLight::Red => 20,
            TrafficLight::Yellow => 5,
        }
    }
}

fn traffic_light() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("🚥 Red light duration: {} seconds", red.duration());
    println!("🚥 Green light duration: {} seconds", green.duration());
    println!("🚥 Yellow light duration: {} seconds", yellow.duration());
}

```

求和 sum.rs

```rust
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

```

计算面积 area.rs

```rust
// 3. 实现一个打印图形面积的函数，接受一个可以计算面积的类型作为参数，比如圆形、三角形、正方形，需要使用到泛型和泛型约束
trait Area {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
struct Rect {
    width: f64,
    height: f64,
}
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
impl Area for Rect {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
fn print_area<T: Area>(shape: T) {
    let area = shape.area();
    println!("The area is: {}", area);
}
fn area() {
    let circle = Circle { radius: 5.0 };
    let triangle = Rect {
        width: 4.0,
        height: 7.0,
    };
    print_area(circle);
    print_area(triangle);
}

```
