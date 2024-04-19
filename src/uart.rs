
use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;


generate_set_functions!(UART0, pwren, rstctl, clkcfg, clkdiv, clksel, pdbgctl, evt_mode, ctl0, lcrh, ifls, ibrd, fbrd, gfctl, txdata, lincnt, linctl, linc0, linc1, irctl, amask, addr, clkdiv2);//, fsctl, gctl missing regs
generate_get_functions!(UART0, pwren, stat, clkdiv, clksel, pdbgctl, evt_mode, ctl0, lcrh, desc, ifls, ibrd, fbrd, gfctl, txdata, rxdata, lincnt, linctl, linc0, linc1, irctl, amask, addr, clkdiv2);//, fsctl, gctl missing regs

//iidx, imask, ris, mis iset, iclr, intctl, ccpd, odis, cclkctl
use crate::generate_set_functions_list;

//generate_set_functions_list!(TIMG0, cc_01, 0, cc_01, 1, cc_23, 0, cc_23, 1, cc_45, 0, cc_45, 1, ccctl_01, 0, ccctl_01, 1, ccctl_23, 0, ccctl_23, 1, ccctl_45, 0, ccctl_45, 1, octl_01, 0, octl_01, 1, octl_23, 0, octl_23, 1, ccact_01, 0, ccact_01, 1, ccact_23, 0, ccact_23, 1, ifctl_01, 0, ifctl_01, 1, ifctl_23, 0, ifctl_23, 1);




pub struct UART0;

pub fn IOMUX_UART_TX_Pin_Config(){
    unsafe{
    let peripherals = pac::Peripherals::steal();
    let iomux = peripherals.IOMUX;
    iomux.pincm[8].write(|w| unsafe {w.bits(0x00000082)})//in put in iomux
   //iomux.pincm[8].write(|w| w
    //.PINCM_PF_W().bits(0x2)
   //)
   //let p = cortex_m::Peripherals::take().unwrap();
}
}


pub fn IOMUX_UART_RX_Pin_Config(){
    unsafe{
    let peripherals = pac::Peripherals::steal();
    let iomux = peripherals.IOMUX;
    iomux.pincm[9].write(|w| unsafe {w.bits(0x00040082)})//check 4 here in iomux receiver
   //iomux.pincm[8].write(|w| w
    //.PINCM_PF_W().bits(0x2)
   //)
}
}


pub fn UART_fillTXFIFO(value: u32){// 8 bits  not 32 bits with a value
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let uart0 = peripherals.UART0;
         UART0::set_txdata(value);
        //uart0.txdata.write(|w| unsafe {w.bits(value)});

    }
}

pub fn UART_Initialization(){//multiple parameters needed , enums to describe the bit values 
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let peph = pac::CorePeripherals::steal();
        let uart0 = peripherals.UART0;
        let clockcfg = peripherals.SYSCTL;
        let gpioa = peripherals.GPIOA;
        let nvic = peph.NVIC;
        //let clockcfg: pac::SYSCTL = peripherals.SYSCTL;
//Bonus steps from Code Composer
 //iinitiate power

   //Reset UART with GPRCM
   UART0::set_rstctl(0xB1000003);
   //uart0.rstctl.write(|w| unsafe {w.bits(0xB1000003)});
  
   //enable power of uart
   UART0::set_pwren(0x26000001);
   //uart0.pwren.write(|w| unsafe {w.bits(0x26000001)});
  
   //delay cycles?
//initiate gpio
   //initiae output peripheral function(yes)Pin 8: fill with 00000082 not 4000082
   //initiate input peripheral function(yes) Pin 9 receive this one is right it is 40000082
   IOMUX_UART_TX_Pin_Config();
   IOMUX_UART_RX_Pin_Config();
//initiate SYSCTL
   //borthreshold
   //set sysocofreq
      //all set
//intiate uart
   //configure clock clk sel to 00008(yes)  no clkdiv
  UART0::set_clksel(0x0000008);
   //uart0.clksel.write(|w| unsafe {w.bits(0x0000008)});
   
   //intiate uart again: ctlo to 00038
   UART0::set_ctl0(0x000038);
 //  uart0.ctl0.write(|w| unsafe {w.bits(0x000038)});//to enable by Or for making sure one bit is changing and not all of them

   //set LCHRn to 00030(yes)
   UART0::set_lcrh(0x30);
   //uart0.lcrh.write(|w| unsafe {w.bits(0x30)});

   //set oversampling
   uart0.ctl0.write(|w| unsafe {w.bits(0x000038)});

   //set baud rate: ibrd:   0x000000D0 fbrd:0x00000015
   uart0.ibrd.write(|w| unsafe {w.bits(0xD0)});
   uart0.fbrd.write(|w| unsafe {w.bits(0x15)});//(yes)
   //enable ineterrupt(see if it works without it)// event0 0x00000400
   uart0.int_event0_imask.write(|w| unsafe {w.bits(0x00000400)});//
   //enable main uart
   uart0.ctl0.write(|w| unsafe {w.bits(0x000039)});

   //EVENT0 RIS 0x00004020
   //now make ctlo to 0x00039
   //VIC_ClearPendingIRQ( NVIC-IRQ[0U]
   // fill NVIC ISER and ICER to x00008000
   //nvic.iser[0].write(|w| unsafe {w.bits(0x15)});//(yes))

/* 
        //Step 1: Configure pin function
        IOMUX_UART_TX_Pin_Config();
        IOMUX_UART_RX_Pin_Config();

        //Step 2: Reset Peripheral
        uart0.rstctl.write(|w| unsafe {w.bits(0xB1000003)});

        //Step 3: Enable Power to UART
        uart0.pwren.write(|w| unsafe {w.bits(0x26000001)});

        //Step 4: Clock, dont need a division
        clockcfg.sysosccfg. write(|w| unsafe {w.bits(0x0000000)});
        uart0.clksel.write(|w| unsafe {w.bits(0x0000008)});

        //Step 5: Disable the UART by clearing enable bit
        uart0.ctl0.write(|w| unsafe {w.bits(0x00000)});

        //Step 6,7,8: Use baud rate eqauition to caluclate UART intger and fraction
        let brd = 32000000 / (16*9600);
        let fbrdd: f32 = (0.333333*64.0)+0.5;
     
        uart0.fbrd.write(|w| unsafe {w.bits(0x15)});//(yes)
        uart0.ibrd.write(|w| unsafe {w.bits(brd)});

        //Step 9: Oversampling and FIFO configuration
        uart0.ctl0.write(|w| unsafe {w.bits(0x20000)});

        //Step 10: Serial parmeters
        uart0.lcrh.write(|w| unsafe {w.bits(0x30)});


        //Step 11: Enable the UART
        uart0.ctl0.write(|w| unsafe {w.bits(0x20001)});
*/
    }
}

    pub fn putonled(){
        unsafe{
            let peripherals = pac::Peripherals::steal();
            let iomux = peripherals.IOMUX;
            iomux.pincm[0].write(|w| unsafe {w.bits(0x00000081)});//check 4 here in iomux receiver

        }
}

