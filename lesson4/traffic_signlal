enum TrafficSignal {
    Red,
    Green,
    Yellow,
}

trait GetTime {
    fn time(self: &Self) -> u32;
}

impl GetTime for TrafficSignal {
    fn time(self: &Self) -> u32 {
        match self {
            TrafficSignal::Red=>60,
            TrafficSignal::Green=>120,
            TrafficSignal::Yellow=>10,
        }
    }
}

fn main() {
    let red = TrafficSignal::Red;
    let green = TrafficSignal::Green;
    let yellow = TrafficSignal::Yellow;
    println!("{}", red.time());
    println!("{}", green.time());
    println!("{}", yellow.time());
}
