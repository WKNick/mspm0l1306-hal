
use cortex_m::peripheral::syst::SystClkSource; // not currently used idk if weneed it but no reason to remove

use cortex_m_rt::exception; // I think this is what allows us to create main


use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access

static mut testing:i16 = 0;

use mspm0l130x::interrupt;
use crate::gpio;
use crate::uart;

#[exception]
fn SysTick() {
    /* */
    static mut COUNT: u32 = 0;
 
    let gpio = gpio::GPIOA.split();

    if *COUNT == 0 {
        unsafe{
            gpio.pa13.set_high();
            testing +=1;
        }
        *COUNT = 1;
    }else{
        gpio.pa13.set_low();
        *COUNT = 0;
    }


}




#[interrupt]
fn INT_GROUP0() {
    let gpio = gpio::GPIOA.split();

    gpio.pa13.set_high();
}
/* 
#[interrupt]
fn INT_GROUP1() { // interrupt for sw2

    uart::UART_fillTXFIFO(3);
    
}
*/


#[interrupt]
fn ADC0() {
    // Your interrupt handler code here
}

#[interrupt]
fn UART0() {
    // Your interrupt handler code here
}


#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    let gpio = gpio::GPIOA.split();

    gpio.pa27.set_high();


}





pub fn interruptsetupgpio(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let gpioa = peripherals.GPIOA;
       

        //gpioa.evt_mode.write(|w|w.bits(0x00000028));
        let EXTI0_INTERRUPT_NUMBER: i16 = 6;

        gpio::GPIOA.split().pa14.set_mac(0x04060081);

        gpioa.int_event0_imask.write(|w|w.bits(0x00004000));

        gpioa.polarity15_0.write(|w|w.bits(0x30000000));

        let nvic = pac::CorePeripherals::steal().NVIC;
        nvic.iser[0].write(0x00000002);
        //nvic.icer[0].write(0x00000002);
        //NVIC::unmask(<Nr as cortex_m::interrupt::InterruptNumber>::number(3));

        //gpioa.int_event0_iset.write(|w|w.bits(0x00004000));
        }   
}

pub fn interruptsetupsysttick(){

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the MSPM0L which has a default CPU clock of 32 MHz
    syst.set_reload(32_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();


   // let mut nvic = p.NVIC;
   // nvic.ipr[].read();
}