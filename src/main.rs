#![no_std]
#![no_main]


mod gpio;
mod uart;
mod exceptions;


use panic_halt as _; // used to allow rust to run so it knows what to do if program crashes

use cortex_m::{self, interrupt::{self, InterruptNumber, Nr}, peripheral::{syst::SystClkSource, NVIC}}; // not currently used idk if we will need it but no reason to remove

use cortex_m_rt::{entry, exception}; // I think this is what allows us to create main


use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access

static mut testing:i16 = 0;

use rtic;


/* 

#[exception]
fn SysTick() {
    /* */
    static mut COUNT: u32 = 0;
 
    let gpio = gpio::PortA.split();

    if *COUNT == 0 {
        unsafe{
        if testing < 0 {
            gpio.pa13.set_high();
            testing +=1;
        }
        }
        *COUNT = 1;
    }else{
        gpio.pa13.set_low();
        *COUNT = 0;
    }


}



#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    let gpio = gpio::PortA.split();
    testing = _irqn;
    if _irqn > 0{
        testing = -1 * _irqn;
    }
    if _irqn == 1{
        //gpio.pa27.set_high();

    }
   // gpio.pa27.set_high();
    // Handle unexpected interrupts here
}

*/
#[interrupt]
fn ADC0() {
    // Your interrupt handler code here
}


#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    let gpio = gpio::PortA.split();

    if _irqn == -1{
        gpio.pa27.set_high();

    }

}






#[entry]
fn main() -> ! {



    unsafe {
        NVIC::unmask(pac::Interrupt::ADC0);
    }


    unsafe{
        let peripherals = pac::Peripherals::steal();
        let gpioa = peripherals.GPIOA;
        gpioa.rstctl.write(|w|w.bits(0xb1000003));// reset gpio info
        gpioa.pwren.write(|w|w.bits(0x26000001));// enable gpio with reset key
        }


    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the MSPM0L which has a default CPU clock of 32 MHz
    syst.set_reload(32_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    unsafe{
        let peripherals = pac::Peripherals::steal();
        let gpioa = peripherals.GPIOA;
       

        //gpioa.evt_mode.write(|w|w.bits(0x00000028));
        let EXTI0_INTERRUPT_NUMBER: i16 = 6;

        gpio::PortA.split().pa14.set_mac(0x04060081);

        gpioa.int_event0_imask.write(|w|w.bits(0x00004000));

        gpioa.polarity15_0.write(|w|w.bits(0x30000000));

        let nvic = pac::CorePeripherals::steal().NVIC;
        nvic.iser[0].write(0x00000002);
        //nvic.icer[0].write(0x00000002);
        //NVIC::unmask(<Nr as cortex_m::interrupt::InterruptNumber>::number(3));

        //gpioa.int_event0_iset.write(|w|w.bits(0x00004000));
        }
    


    loop {}







    /* 
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let gpioa = peripherals.GPIOA;
        gpioa.rstctl.write(|w|w.bits(0xb1000003));// reset gpio info
        gpioa.pwren.write(|w|w.bits(0x26000001));// enable gpio with reset key
        }
    
    let peripherals=unsafe{pac::Peripherals::steal()};

    let gpio = gpio::PortA.split();
    let uart0 = peripherals.UART0;

    uart::IOMUX_UART_TX_Pin_Config();
    uart::IOMUX_UART_RX_Pin_Config();
    
    //uart::confg_clk();
    
    uart::UART_SetUp();
    //uart::UART_SetUp_Regs();

    
//    uart::IOMUX_UART_TX_Pin_Config();
//    uart::IOMUX_UART_RX_Pin_Config();
    
    //uart::UART_Clock();

    let data = 0b0000_1101;

// random garbage loop for the end of main just incase an old bug still exists which requires the loop not to be empty for rust not to crash
let mut x =1;
let mut int = 3;
    loop {   
        if (uart0.stat.read().bits() & 0x00000040) == 0 {
            gpio.pa27.set_high();
        }else{
            gpio.pa27.set_low();
        }
        
        if x == 0{
            uart:: UART_fillTXFIFO(data);
        }
        
        if gpio.pa18.get_input(){
            x = -10;
            gpio.pa13.set_high();

        }else{
            gpio.pa13.set_low();
        }

        if gpio.pa14.get_input(){
            if int == 5{
                int = 4;
            uart0.ctl0.write(|w| unsafe {w.bits(0x00020011)});
           // gpio.pa27.set_high();
            }

        }else{
            if int == 4{
                int = 5;
            uart0.ctl0.write(|w| unsafe {w.bits(0x00020001)});
            //gpio.pa27.set_low();
            }
        }


/* 
        if gpio.pin14.get_input(){
            //gpio.pin13.set_high();
            gpio.pin13.set_low();

        }else{
            gpio.pin13.set_high();
        }/*
        if gpio.pin18.get_input(){
            gpio.pin27.set_high();

        }else{
            gpio.pin27.set_low();
        }
        
        */
        
*/


    x = x+1;
        continue;
    }


    */
}





