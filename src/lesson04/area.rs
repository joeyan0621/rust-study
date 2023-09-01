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
