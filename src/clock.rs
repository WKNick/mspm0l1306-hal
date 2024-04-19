use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;


generate_set_functions!(SYSCTL, sysosccfg, mclkcfg, genclkcfg, genclken, pmodecfg, sysosctrimuser, sramboundary, systemcfg, writelock, resetlevel, resetcmd, borthreshold, borclrcmd, sysoscfclctl, shdniorel, exrstpin, swdcfg, fcccmd, pmuopamp);//, fsctl, gctl missing regs
generate_get_functions!(SYSCTL, sysosccfg, mclkcfg, genclkcfg, genclken, pmodecfg, fcc, sysosctrimuser, sramboundary, systemcfg, writelock, rstcause, resetlevel, borthreshold, pmuopamp);//, fsctl, gctl missing regs
//sysstatusclr,  clkstatus, sysstatus,
//iidx, imask, ris, mis iset, iclr, intctl, ccpd, odis, cclkctl 
use crate::generate_set_functions_list;

pub struct SYSCTL;


//ULPClK/MCLK 32 MHz
//All default dont set anything
pub fn use_ulpclk(){
    SYSCTL::set_sysosccfg(0x00);
    SYSCTL::set_mclkcfg(0x0);
}
//MFCLK 4 MHz
pub fn use_mfclk(){
// MFCLK can be enabled in software by setting USEMFTICK in the
//MCLKCFG register in SYSCTL. MFCLK is active in RUN, SLEEP, and STOP power modes only, and 
//SYSOSC must be enabled for MFCLK to operate.

//Set the SYSOSC frequency to 4 MHz (set the FREQ field in the SYSOSCCFG register to 0x01)
    SYSCTL::set_sysosccfg(0x01);
    SYSCTL::set_mclkcfg(4096);

}

//LFCLK
  //Source SYSOC to LFCLK
    //To switch MCLK from SYSOSC to LFCLK in RUN mode:
    //1. Verify that MCLK is sourced from SYSOSC (CLKSTATUS.CURMCLKSEL is cleared)
    //2. If SYSOSC is not running at base frequency, and SYSOSC is to be left enabled when switching MCLK to
    //LFCLK, set SYSOSC to base frequency before proceeding
    //3. Set MCLKCFG.USELFCLK to switch MCLK to LFCLK and leave SYSOSC enabled, or set
    //SYSOSCCFG.DISABLE to switch MCLK to LFCLK and disable SYSOSC
  

pub fn clock_test(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let iomux = peripherals.IOMUX;
       
        iomux.pincm[7].write(|w| unsafe {w.bits(0x00040083)});//output why you put 40083
    //    1. Configure IOMUX to select the CLK_OUT function on the device pin with CLK_OUT.
      //  2. Select the desired clock source in the EXCLKSRC field of the GENCLKCFG register.
      //0011
       // 3. Set the desired clock divider, if necessary, in the EXCLKDIVVAL field of the GENCLKCFG register, and
        //enable the divider by setting the EXCLKDIVEN bit.
        SYSCTL::set_genclkcfg(0x80);
      //  4. Enable the external clock output by setting the EXCLKEN bit in the GENCLKEN register.
        SYSCTL::set_genclken(1);
    }
}
 
/* pub fn UART_Clock(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let uart0 = peripherals.UART0;
        let clockcfg = peripherals.SYSCTL;

        clockcfg.sysosccfg. write(|w| unsafe {w.bits(0x0000000)});
        uart0.clksel.write(|w| unsafe {w.bits(0x0000008)});
        
        // set baud rate

        let brd = 32000000 / (16*9600);
        let fbrdd: f32 = (0.333333*64.0)+0.5;
     
        //let b = (a / 100000.0) as i64;
        uart0.fbrd.write(|w| unsafe {w.bits(0x15)});

        uart0.ibrd.write(|w| unsafe {w.bits(brd)});//double check
                //let sysctl = peripherals.SYSCTL;
        //sysctl.mclkcfg.write(|w| unsafe {w.bits(0x00040082)})
       //iomux.pincm[8].write(|w| w
        //.PINCM_PF_W().bits(0x2)
}*/
 