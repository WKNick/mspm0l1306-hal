



use panic_halt as _; // used to allow rust to run so it knows what to do if program crashes

use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access




use core::ptr;

pub fn confg_clk(){
    unsafe {
            const REGISTER_ADDRESS: *mut u32 = 0x400B0100 as *mut u32;

            // Value to write to the register
            let value_to_write:u32 = 0x01;

            
                // Use volatile write to prevent compiler optimizations
                ptr::write_volatile(REGISTER_ADDRESS, value_to_write);
        }
    unsafe {
        const REGISTER_ADDRESS: *mut u32 = 0x400B0104 as *mut u32;

        // Value to write to the register
        let value_to_write:u32 = 0x1200;

        
            // Use volatile write to prevent compiler optimizations
            ptr::write_volatile(REGISTER_ADDRESS, value_to_write);
        }
}










/*void DL_UART_configBaudRate(
    UART_Regs *uart, uint32_t clockFreq, uint32_t baudRate)
{
    uint32_t divisor;

    /*
     *  The baud rate divisor, brd, is calculated with the following formula:
     *  brd = ibrd.fbrd = clockOutput / (OVS * baudRate)
     *  where ibrd is the integer part, fbrd is the fractional part.
     *  Since fbrd is 6 bits, multiply brd by 64:
     *       64 * brd = (clockOutput * 64) / (16 * baudRate)
     *                = (clockOutput * 4) / baudRate
     *    add 1/2 to round the least significant bit of fbrd:
     *        64 * brd + 1/2 = (clockOutput * 8 / (2 * baudRate) + 1/2
     *        divisor = 64*brd+1/2  = [(clockOutput * 8)/ baudRate + 1] / 2
     *
     *  The lower 6 bits is fbrd, upper part is ibrd
     *  Note: If ibrd is 0, FBRD will be ignored and no data will be
     *  transferred.
     */

    /*  Calculate baud rate divisor based on OVS: */
    if ((baudRate * (uint32_t) 8) > clockFreq) {
        DL_UART_setOversampling(uart, DL_UART_OVERSAMPLING_RATE_3X);
        divisor = ((clockFreq * (uint32_t) 64) / (baudRate * (uint32_t) 3)) +
                  ((uint32_t) 1 / (uint32_t) 2);

    } else if ((baudRate * (uint32_t) 16) > clockFreq) {
        DL_UART_setOversampling(uart, DL_UART_OVERSAMPLING_RATE_8X);
        baudRate = baudRate / (uint32_t) 2;
        divisor  = (((clockFreq * (uint32_t) 8) / baudRate) + (uint32_t) 1) /
                  (uint32_t) 2;

    } else {
        DL_UART_setOversampling(uart, DL_UART_OVERSAMPLING_RATE_16X);
        divisor = (((clockFreq * (uint32_t) 8) / baudRate) + (uint32_t) 1) /
                  (uint32_t) 2;
    }

    /* Set the integer and fractional parts of the baud rate divisor */
    DL_UART_setBaudRateDivisor(
        uart, divisor >> (uint32_t) 6, divisor & (uint32_t) 0x3F);
}
*/
//Setting up pins on board to UART

pub fn IOMUX_UART_TX_Pin_Config(){
    unsafe{
    let peripherals = pac::Peripherals::steal();
    let iomux = peripherals.IOMUX;
    iomux.pincm[8].write(|w| unsafe {w.bits(0x00000082)})
   //iomux.pincm[8].write(|w| w
    //.PINCM_PF_W().bits(0x2)
   //)
}
}
pub fn IOMUX_UART_RX_Pin_Config(){
    unsafe{
    let peripherals = pac::Peripherals::steal();
    let iomux = peripherals.IOMUX;
    iomux.pincm[9].write(|w| unsafe {w.bits(0x000400082)})
   //iomux.pincm[8].write(|w| w
    //.PINCM_PF_W().bits(0x2)
   //)
}
}
//Setting up clock

pub fn UART_Clock(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let uart0 = peripherals.UART0;
        let clockcfg = peripherals.SYSCTL;

        clockcfg.sysosccfg.write(|w| unsafe {w.bits(0x00000001)});// 0 is 32Mhz default
        uart0.clksel.write(|w| unsafe {w.bits(0x0000004)}); // 4 is mclk
        
        // set baud rate

        let brd = 4000000 / (16*9600);
        let fbrdd: f32 = 0.041667;
        uart0.fbrd.write(|w| w.bits(0x00000003));
        uart0.ibrd.write(|w| w.bits(0x00000018));

        uart0.lcrh.write(|w|  w.bits(0x00000000));
                //let sysctl = peripherals.SYSCTL;
        //sysctl.mclkcfg.write(|w| unsafe {w.bits(0x00040082)})
       //iomux.pincm[8].write(|w| w
        //.PINCM_PF_W().bits(0x2)
}

}//UART Section
pub fn UART_SetUp_Regs(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let uart0 = peripherals.UART0;
        //first enable uart by enabling the power periph.reg.write(|w| unsafe { w.bits(rawbits) });
        uart0.rstctl.write(|w| unsafe {w.bits(0xB1000003)});
        uart0.pwren.write(|w| unsafe {w.bits(0x26000001)});
        uart0.clkcfg.write(|w| unsafe {w.bits(0xA9000100)});//make sure the key
        //uart0.ctl0.write(|w| unsafe {w.bits(0x00020039)});//0x00000019 //0x00020039
       // uart0.RESERVED0[2].write(|w| unsafe {w.bits()});
       


    }
}
// will combine the fill and transmit data pub fn UART_transmitData

pub fn UART_fillTXFIFO(value: u32){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let uart0 = peripherals.UART0;
        uart0.txdata.write(|w| unsafe {w.bits(value)});

    }
}



pub fn UART_SetUp(){// input
    let peripherals = unsafe{pac::Peripherals::steal()};
    let uart0 = peripherals.UART0;


    uart0.rstctl.write(|w| unsafe {w.bits(0xB1000003)}); // works
    uart0.pwren.write(|w| unsafe {w.bits(0x26000001)});// works
    peripherals.SYSCTL.sysosccfg.write(|w| unsafe {w.bits(0x00000000)});// 32mhz
    uart0.clksel.write(|w| unsafe {w.bits(0x0000008)}); // busclk = sysclk
    uart0.clkdiv.write(|w| unsafe {w.bits(0x0000000)});

    uart0.ctl0.write(|w| unsafe {w.bits(0x00000000)});
    uart0.fbrd.write(|w| unsafe {w.bits(0x00000015)});//03
    uart0.ibrd.write(|w| unsafe {w.bits(0x000000D0)});//018
    uart0.lcrh.write(|w| unsafe {w.bits(0x00000030)});
    uart0.ctl0.write(|w| unsafe {w.bits(0x00020011)});//0x00000019 //0x00020039
    uart0.ctl0.write(|w| unsafe {w.bits(0x00020011)});

}
pub fn UART_SetUp_output(){
    let peripherals = unsafe{pac::Peripherals::steal()};
    let uart0 = peripherals.UART0;


    uart0.rstctl.write(|w| unsafe {w.bits(0xB1000003)}); // works
    uart0.pwren.write(|w| unsafe {w.bits(0x26000001)});// works
    peripherals.SYSCTL.sysosccfg.write(|w| unsafe {w.bits(0x00000000)});// 32mhz
    uart0.clksel.write(|w| unsafe {w.bits(0x0000008)}); // busclk = sysclk
    uart0.clkdiv.write(|w| unsafe {w.bits(0x0000000)});

    uart0.ctl0.write(|w| unsafe {w.bits(0x00000000)});
    uart0.fbrd.write(|w| unsafe {w.bits(0x00000015)});//03
    uart0.ibrd.write(|w| unsafe {w.bits(0x000000D0)});//018
    uart0.lcrh.write(|w| unsafe {w.bits(0x00000030)});
    uart0.ctl0.write(|w| unsafe {w.bits(0x00020000)});//0x00000019 //0x00020039
    uart0.ctl0.write(|w| unsafe {w.bits(0x00020001)});

}
