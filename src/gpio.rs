
use panic_halt as _; // used to allow rust to run so it knows what to do if program crashes


use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access






pub struct PA0;
pub struct PA1;
pub struct PA2;
pub struct PA3;
pub struct PA4;
pub struct PA5;
pub struct PA6;
pub struct PA7;
pub struct PA8;
pub struct PA9;
pub struct PA10;
pub struct PA11;
pub struct PA12;
pub struct PA13;
pub struct PA14;
pub struct PA15;
pub struct PA16;
pub struct PA17;
pub struct PA18;
pub struct PA19;
pub struct PA20;
pub struct PA21;
pub struct PA22;
pub struct PA23;
pub struct PA24;
pub struct PA25;
pub struct PA26;
pub struct PA27;
pub struct PA28;
pub struct PA29;
pub struct PA30;
pub struct PA31;






pub struct PortA;

impl PortA {
    pub fn split(self) -> PortAPins {
        PortAPins {
            pa0: PA0,
            pa1: PA1,
            pa2: PA2,
            pa3: PA3,
            pa4: PA4,
            pa5: PA5,
            pa6: PA6,
            pa7: PA7,
            pa8: PA8,
            pa9: PA9,
            pa10: PA10,
            pa11: PA11,
            pa12: PA12,
            pa13: PA13,
            pa14: PA14,
            pa15: PA15,
            pa16: PA16,
            pa17: PA17,
            pa18: PA18,
            pa19: PA19,
            pa20: PA20,
            pa21: PA21,
            pa22: PA22,
            pa23: PA23,
            pa24: PA24,
            pa25: PA25,
            pa26: PA26,
            pa27: PA27,
            pa28: PA28,
            pa29: PA29,
            pa30: PA30,
            pa31: PA31,
        }
    }
}

pub struct PortAPins {
    pub pa0: PA0,
    pub pa1: PA1,
    pub pa2: PA2,
    pub pa3: PA3,
    pub pa4: PA4,
    pub pa5: PA5,
    pub pa6: PA6,
    pub pa7: PA7,
    pub pa8: PA8,
    pub pa9: PA9,
    pub pa10: PA10,
    pub pa11: PA11,
    pub pa12: PA12,
    pub pa13: PA13,
    pub pa14: PA14,
    pub pa15: PA15,
    pub pa16: PA16,
    pub pa17: PA17,
    pub pa18: PA18,
    pub pa19: PA19,
    pub pa20: PA20,
    pub pa21: PA21,
    pub pa22: PA22,
    pub pa23: PA23,
    pub pa24: PA24,
    pub pa25: PA25,
    pub pa26: PA26,
    pub pa27: PA27,
    pub pa28: PA28,
    pub pa29: PA29,
    pub pa30: PA30,
    pub pa31: PA31,
}


/// A pin on port A.
pub struct PA {
    /// The pin number.
    pin: u32,
}

impl PA {
    pub fn erase_port(self) -> Pin {
        Pin {
            port: Port::A,
            pin: self.pin,
        }
    }
// general functions PA0-PA31 can inherit for basics
pub fn set_mac(&self,value: u32){
    
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let iomux = peripherals.IOMUX;
        iomux.pincm[self.pin as usize].write(|w|w.bits(value)); //info for iomux pin 13 gpio function for output pin 0x00000081 is for 13 to be output/gpio
        }

}
pub fn set_high(&self){

    unsafe{
    let peripherals = pac::Peripherals::steal();
    let gpioa = peripherals.GPIOA;
    //if self.pin_cm != 0x00000081{
        self.set_mac(0x00000081); // sets self as output
    //}
    gpioa.doe31_0.write(|w|w.bits(gpioa.doe31_0.read().bits() | ((1u32)<<self.pin))); // set own pin to enable outputs
    gpioa.dout31_0.write(|w|w.bits(gpioa.dout31_0.read().bits() | ((1u32)<<self.pin)));// setting own pin to ouput of 1
    }
}
pub fn set_low(&self){
    unsafe{
    let peripherals = pac::Peripherals::steal();
    let gpioa = peripherals.GPIOA;

   // if self.pin_cm != 0x00000081{
        self.set_mac(0x00000081); // sets self as output
    //}

    gpioa.doe31_0.write(|w|w.bits(gpioa.doe31_0.read().bits() | ((1u32)<<self.pin))); // set own pin to enable outputs
    gpioa.dout31_0.write(|w|w.bits(gpioa.dout31_0.read().bits() & !((1u32)<<self.pin)));// setting own pin to ouput of 0
    }
}

pub fn get_input(&self) -> bool{ // need something else to enable inputs mayby a diffrent mac??
    unsafe{
    let peripherals = pac::Peripherals::steal();
    let gpioa = peripherals.GPIOA;

    //if self.pin_cm != 0x00060081{
        //self.set_mac(0x00060081); // sets self as input
   // }
    gpioa.doe31_0.write(|w|w.bits(gpioa.doe31_0.read().bits() & !((1u32)<<self.pin))); // set own pin to disable outputs

    match gpioa.din31_0.read().bits() & ((1u32)<<self.pin){
        0x00000000 => return false,
        _ => return true,
    }
    }
}










}

pub struct Pin {
    port: Port,
    pin: u32,
    // (these fields can be packed to reduce the memory footprint)
}

enum Port {
    A,
}


impl PA0 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 0 }
    }
}

impl PA1 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 1 }
    }
}

impl PA2 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 2 }
    }
}

impl PA3 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 3 }
    }
}

impl PA4 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 4 }
    }
}

impl PA5 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 5 }
    }
}

impl PA6 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 6 }
    }
}

impl PA7 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 7 }
    }
}

impl PA8 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 8 }
    }
}

impl PA9 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 9 }
    }
}

impl PA10 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 10 }
    }
}

impl PA11 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 11 }
    }
}

impl PA12 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 12 }
    }
}

impl PA13 { // an led
    pub fn erase_pin(&self) -> PA {
        PA { pin: 13 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_high();
    }
}

impl PA14 { // sw
    pub fn erase_pin(&self) -> PA {
        PA { pin: 14 }
    }

    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }

    pub fn get_input(&self) -> bool{
        self.erase_pin().set_mac(0x04060081);
        return self.erase_pin().get_input();
    }
 
}

impl PA15 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 15 }
    }
}

impl PA16 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 16 }
    }
}

impl PA17 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 17 }
    }
}

impl PA18 {//sw
    pub fn erase_pin(&self) -> PA {
        PA { pin: 18 }
    }
    pub fn get_input(&self) -> bool{
        self.erase_pin().set_mac(0x00050081);
        return self.erase_pin().get_input();
    }
}

impl PA19 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 19 }
    }
}

impl PA20 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 20 }
    }
}

impl PA21 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 21 }
    }
}

impl PA22 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 22 }
    }
}

impl PA23 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 23 }
    }
}

impl PA24 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 24 }
    }
}

impl PA25 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 25 }
    }
}

impl PA26 { // an led
    pub fn erase_pin(&self) -> PA {
        PA { pin: 26 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_high();
    }
}

impl PA27 { // an led
    pub fn erase_pin(&self) -> PA {
        PA { pin: 27 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_high();
    }
}

impl PA28 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 28 }
    }
}

impl PA29 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 29 }
    }
}

impl PA30 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 30 }
    }
}

impl PA31 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 31 }
    }
}








