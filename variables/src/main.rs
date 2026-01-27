const STARTING_MISSILES :i32 = 8;
const READY_AMOUNT :i32 = 2;

fn main() {
    let _missiles :i32 = STARTING_MISSILES;
    let _ready :i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", _ready, _missiles);

    //mut _missiles;
    let mut _missiles :i32 = _missiles - _ready;
    println!("{} missiles left", _missiles);
}
