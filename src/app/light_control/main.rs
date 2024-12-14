// imports
use light_control_app::app::light_control::light_control::LightControl;


fn main() {
    let light_control = LightControl::new();
    light_control.start();
}