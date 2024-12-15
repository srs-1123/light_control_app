extern crate rppal;

use rppal::gpio::Gpio;
// use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
use std::error::Error;

fn readadc(abcnum: u8, clockpin: u8, mosipin: u8, misopin: u8, cspin: u8) -> Result<u16, Box<dyn Error>> {
    if abcnum > 7 {
        return Err("Invalid channel number".into());
    }

    let gpio = Gpio::new()?;

    let mut clock = gpio.get(clockpin)?.into_output();  
    let mut mosi = gpio.get(mosipin)?.into_output();    
    let mut cs = gpio.get(cspin)?.into_output();       

    let miso = gpio.get(misopin)?.into_input();  // misoを追加

    cs.set_high();
    clock.set_low();
    cs.set_low();

    let mut commandout = abcnum;
    commandout |= 0x18;
    commandout <<= 3;

    for _i in 0..5 {
        if commandout & 0x80 != 0 {
            mosi.set_high();
        } else {
            mosi.set_low();
        }
        commandout <<= 1;
        clock.set_high();
        clock.set_low();
    }

    let mut adcout: u16 = 0;
    for i in 0..13 {
        clock.set_high();
        clock.set_low();
        adcout <<= 1;
        if i > 0 && miso.is_high() {
            adcout |= 0x1;
        }
    }

    cs.set_high();
    Ok(adcout)
}

fn main() {
    let abcnum: u8 = 0;
    let clockpin: u8 = 17;
    let mosipin: u8 = 27;
    let misopin: u8 = 22;
    let cspin: u8 = 23;

    match readadc(abcnum, clockpin, mosipin, misopin, cspin) {
        Ok(adc_value) => println!("ADC Value: {}", adc_value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
