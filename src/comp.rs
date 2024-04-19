use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;


generate_set_functions!(COMP0, pwren, fsub_0, fsub_1, fpub_1, rstctl, clkcfg, ctl0, ctl1, ctl2, ctl3); 
generate_get_functions!(COMP0, pwren, fsub_0, fsub_1, fpub_1, clkcfg, stat, desc, ctl0, ctl1, ctl2, ctl3);//, fsctl, gctl missing regs

//iidx, imask, ris, mis iset, iclr, intctl, ccpd, odis, cclkctl
use crate::generate_set_functions_list;

//generate_set_functions_list!(TIMG0, cc_01, 0, cc_01, 1, cc_23, 0, cc_23, 1, cc_45, 0, cc_45, 1, ccctl_01, 0, ccctl_01, 1, ccctl_23, 0, ccctl_23, 1, ccctl_45, 0, ccctl_45, 1, octl_01, 0, octl_01, 1, octl_23, 0, octl_23, 1, ccact_01, 0, ccact_01, 1, ccact_23, 0, ccact_23, 1, ifctl_01, 0, ifctl_01, 1, ifctl_23, 0, ifctl_23, 1);

pub struct COMP0;

pub fn initialize_comp_led_demo(){
    COMP0::set_pwren(0x26000001);
    //enable control bit in CTL0 register
   // VREF::set_ctl0(0x1);
    //Select internal voltage with BuFConfig control bit in CTL0
    //VREF::set_ctl0(0x81);
    //Ready bit in CTL1 register to indicate it is ready

    //ask about external reference and start up time

    unsafe{ 
        let peripherals = pac::Peripherals::steal();
        let gpioa = peripherals.GPIOA;
        let vref = peripherals.VREF;
        let iomux = peripherals.IOMUX;
        let comp0 = peripherals.ADC0;
        let sys = peripherals.SYSCTL;
     
        gpioa.rstctl.write(|w|w.bits(0xb1000003));// reset gpio info
        vref.rstctl.write(|w|w.bits(0xb1000003));// reset gpio info
        COMP0::set_rstctl(0xB1000003);

        gpioa.pwren.write(|w|w.bits(0x26000001));// enable gpio with reset key
        vref.pwren.write(|w|w.bits(0x26000001));// reset gpio info
        COMP0::set_pwren(0x26000001);

        iomux.pincm[0].write(|w| unsafe {w.bits(0x00000081)});//check 4 here in iomux receiver
        iomux.pincm[19].write(|w| unsafe {w.bits(0x00060082)});//check 4 here in iomux receiver
        iomux.pincm[20].write(|w| unsafe {w.bits(0x00050082)});//check 4 here in iomux receiver

        gpioa.dout31_0.write(|w| unsafe {w.bits(0x0000001)});
        gpioa.doe31_0.write(|w| unsafe {w.bits(0x0000001)});
        gpioa.pdbgctl.write(|w| unsafe {w.bits(0x0000001)});
       
        sys.sysosccfg.write(|w| unsafe {w.bits(0x00020000)});

        //vref set up
        vref.clksel.write(|w|w.bits(0x08));// reset gpio info
        vref.ctl0.write(|w|w.bits(0x000001));// reset gpio info
        vref.ctl1.write(|w|w.bits(0x000001));// reset gpio info

        COMP0::set_ctl0(0x80000000);        
        COMP0::set_ctl1(0x060);
        COMP0::set_ctl1(0x0560);
        COMP0::set_ctl2(0x00010008);
        COMP0::set_ctl1(0x0561);
        COMP0::set_ctl3(0x000000);
        COMP0::set_ctl3(0x0000007F);

    }
}
pub fn turnoff_comp(){
    //this to save power
    COMP0::set_pwren(0x26000000);
}