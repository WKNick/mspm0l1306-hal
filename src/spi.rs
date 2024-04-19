
//use cortex_m; // not currently used idk if we will need it but no reason to remove

//use cortex_m_rt::entry; // I think this is what allows us to create main
//use cortex_m::interrupt;
use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;

generate_set_functions!(SPI0, pwren, rstctl, clkcfg, clkdiv, clksel, pdbgctl, evt_mode, ctl0, ctl1, clkctl, ifls, txdata);//, fsctl, gctl missing regs
generate_get_functions!(SPI0, pwren, clkcfg, stat, clkdiv, clksel, pdbgctl, evt_mode, ctl0, ctl1, clkctl, ifls, rxdata, txdata);//, fsctl, gctl missing regs
//generate_set_functions!(SPI0, pwren, rstctl, clkcfg, clkdiv, clksel, pdbgctl, imask, iset, iclr, evt_mode, intctl, ctl0, ctl1, clkctl, ifls, txdata);//, fsctl, gctl missing regs
//generate_get_functions!(SPI0, pwren, clkcfg, stat, clkdiv, clksel, pdbgctl, iidx, imask, ris, mis, evt_mode, intctl, ctl0, ctl1, clkctl, ifls, rxdata, txdata);//, fsctl, gctl missing regs

pub struct SPI0;


pub fn SPI_enable() { //enables SPI registers
    
        //Reset GpiO
        SPI0::set_rstctl(0xb1000003);// reset gpio info

        //enable the power of gpioa
        SPI0::set_pwren(0x26000001);// enable gpio with reset key

        // Example of setting registers
        unsafe{
        let peripherals = pac::Peripherals::steal();
        let iomux = peripherals.IOMUX;
        iomux.pincm[20].write(|w| unsafe {w.bits(0x50082)});
        iomux.pincm[19].write(|w| unsafe {w.bits(0x60082)});
        iomux.pincm[6].write(|w| unsafe {w.bits(0x83)});
        iomux.pincm[5].write(|w| unsafe {w.bits(0x83)});
        iomux.pincm[4].write(|w| unsafe {w.bits(0x40083)});
        iomux.pincm[7].write(|w| unsafe {w.bits(0x83)});
        
    }

        //SPI0::set_clksel(0x4);// mfclk
        SPI0::set_clksel(0x4);// sysclk
        SPI0::set_clkdiv(0x0);// clock division, currently set to no clock division
        SPI0::set_clkctl(0x1F);// serial clock divider

        SPI0::set_pdbgctl(0x3);
        SPI0::set_evt_mode(0x29);

        //SPI0::set_ctl1(0x814);
        SPI0::set_ctl0(0x07);
        SPI0::set_ctl1(0x815);// enable SPI

        SPI0::set_ifls(0x3F);

}

// Function to send data through MOSI
pub fn SPI_send(value: u32) {
    SPI0::set_txdata(value);
}

/*
pub fn SPI_test() {
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let spi: pac::SPI0 = peripherals.SPI0;
        
        SPI0::set_clksel(0x00000008);
        //spi.clksel.write(|w:| unsafe{w.bits(0x0000008)});

        SPI0::set_clkdiv(0x00000003);
        //spi.clkdiv.write(|w:| unsafe{w.bits(0x0000000)});

        SPI0::set_clkctl(0x00000000);

        spi.ctl1.write(|w| unsafe {w.bits(0x14)});

        spi.ctl0.write(|w| unsafe {w.bits(0x3)});

        spi.ctl1.write(|w| unsafe {w.bits(0x15)}); //enable SPI

        spi.txdata.write(|w|unsafe {w.bits(0x20)})
    }
}
*/


//use cortex_m_rt::interrupt;

/*
pub fn SPI_INIT() {
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let spi0: SPI0 = peripherals.SPI0;

    }
}
*/
