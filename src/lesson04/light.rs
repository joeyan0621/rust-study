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
