


use mspm0l130x as pac;

use crate::timg::TIMG0; // the rust crate created for the MSPM0L for peripherals access
use crate::timg::TIMG1; // the rust crate created for the MSPM0L for peripherals access

use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;


generate_set_functions!(GPIOA, fsub_0, fsub_1, fpub_0, fpub_1, pwren, rstctl, clkovr, pdbgctl, evt_mode, dout31_0, doe31_0, polarity15_0);//MANY missing regs polarity 31_16 is most noticable since 15_0 is there but every register after is also missing
generate_get_functions!(GPIOA, fsub_0, fsub_1, fpub_0, fpub_1, pwren, stat, clkovr, pdbgctl, evt_mode, desc, dout31_0, doe31_0, din31_0, polarity15_0);//MANY missing regs polarity 31_16 is most noticable since 15_0 is there but every register after is also missing


pub fn enable(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let gpioa = peripherals.GPIOA;
        gpioa.rstctl.write(|w|w.bits(0xb1000003));// reset gpio info
        gpioa.pwren.write(|w|w.bits(0x26000001));// enable gpio with reset key
        }
}



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






pub struct GPIOA;

impl GPIOA {
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

    gpioa.doe31_0.write(|w|w.bits(gpioa.doe31_0.read().bits() | ((1u32)<<self.pin))); // set own pin to enable outputs
    gpioa.dout31_0.write(|w|w.bits(gpioa.dout31_0.read().bits() | ((1u32)<<self.pin)));// setting own pin to ouput of 1
    }
}
pub fn set_low(&self){
    unsafe{
    let peripherals = pac::Peripherals::steal();
    let gpioa = peripherals.GPIOA;


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
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x04000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x04000081);
        self.erase_pin().set_high();
    }

    pub fn configure_pwm(&self){
        self.erase_pin().set_mac(0x04000084);
    
        TIMG1::set_ccctl_01_0(0x00000000);
    
        TIMG1::set_octl_01_0(0x00000000);
    
        TIMG1::set_ccact_01_0(0x00000280);
    
        TIMG1::set_octl_01_0(0x00000000);

        TIMG1::set_cc_01_0(((((0 as f32) / 100.0)) * (TIMG1::get_load() as f32)) as u32);
    }

    pub fn setpwm(&self, val: u32){
        TIMG1::set_cc_01_0(((1.0 - ((val as f32) / 100.0)) * (TIMG1::get_load() as f32)) as u32);
    }

}

impl PA1 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 1 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA2 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 2 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA3 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 3 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA4 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 4 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA5 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 5 }
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }

    pub fn configure_pwm(&self){
        self.erase_pin().set_mac(0x00000082);
    
        TIMG0::set_ccctl_01_0(0x00000000);
    
        TIMG0::set_octl_01_0(0x00000000);
    
        TIMG0::set_ccact_01_0(0x00000280);
    
        TIMG0::set_octl_01_0(0x00000000);

        TIMG0::set_cc_01_0(((1.0 - ((0 as f32) / 100.0)) * (TIMG0::get_load() as f32)) as u32);
    }

    pub fn setpwm(&self, val: u32){
        TIMG0::set_cc_01_0(((1.0 - ((val as f32) / 100.0)) * (TIMG0::get_load() as f32)) as u32);
    }

}

impl PA6 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 6 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA7 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 7 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA8 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 8 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
    pub fn config_tx(&self){
        self.erase_pin().set_mac(0x00000082);
    }
}

impl PA9 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 9 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
    pub fn config_rx(&self){
        self.erase_pin().set_mac(0x00040082);
    }
}

impl PA10 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 10 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA11 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 11 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA12 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 12 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
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
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA14 { // sw
    pub fn erase_pin(&self) -> PA {
        PA { pin: 14 }
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
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
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA16 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 16 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA17 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 17 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA18 {//sw
    pub fn erase_pin(&self) -> PA {
        PA { pin: 18 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn get_input(&self) -> bool{
        self.erase_pin().set_mac(0x00050081);
        return self.erase_pin().get_input();
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA19 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 19 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA20 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 20 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA21 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 21 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA22 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 22 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA23 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 23 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA24 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 24 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA25 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 25 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
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
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
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
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA28 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 28 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA29 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 29 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA30 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 30 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}

impl PA31 {
    pub fn erase_pin(&self) -> PA {
        PA { pin: 31 }
    }
    pub fn set_mac(&self, value: u32){
        self.erase_pin().set_mac(value);
    }
    pub fn set_low(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_low();
    }
    pub fn set_high(&self){
        self.erase_pin().set_mac(0x00000081);
        self.erase_pin().set_high();
    }
}








