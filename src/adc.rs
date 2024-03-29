
use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;


generate_set_functions!(ADC0, fsub_0, fpub_1, pwren, rstctl, clkcfg, ctl0, clkfreq, scomp0, scomp1, ctl1, ctl2, ctl3, refcfg, wclow, wchigh); 
generate_get_functions!(ADC0, fsub_0, fpub_1, pwren, stat, clkcfg, desc, ctl0, scomp0, scomp1, ctl1, ctl2, ctl3, refcfg, wclow, wchigh);//, fsctl, gctl missing regs

generate_get_functions!(ADC0_SVT, fifodata);//, fsctl, gctl missing regs
//put in clkfrq later

//use the adc0_svt. memres[] to get a value and adc0.memctl[] to read/write a value
//iidx, imask, ris, mis iset, iclr, intctl, ccpd, odis, cclkctl
use crate::generate_set_functions_list;

//generate_set_functions_list!(TIMG0, cc_01, 0, cc_01, 1, cc_23, 0, cc_23, 1, cc_45, 0, cc_45, 1, ccctl_01, 0, ccctl_01, 1, ccctl_23, 0, ccctl_23, 1, ccctl_45, 0, ccctl_45, 1, octl_01, 0, octl_01, 1, octl_23, 0, octl_23, 1, ccact_01, 0, ccact_01, 1, ccact_23, 0, ccact_23, 1, ifctl_01, 0, ifctl_01, 1, ifctl_23, 0, ifctl_23, 1);




pub struct ADC0;
pub struct ADC0_SVT;

pub fn initialize_adc_led_demo(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let gpioa = peripherals.GPIOA;
        let vref = peripherals.VREF;
        let iomux = peripherals.IOMUX;
        let adc0 = peripherals.ADC0;
      
        gpioa.rstctl.write(|w|w.bits(0xb1000003));// reset gpio info
        ADC0::set_rstctl(0xB1000003);
        vref.rstctl.write(|w|w.bits(0xb1000003));// reset gpio info

        gpioa.pwren.write(|w|w.bits(0x26000001));// enable gpio with reset key
        ADC0::set_pwren(0x26000001);
        vref.pwren.write(|w|w.bits(0x26000001));// reset gpio info

        iomux.pincm[0].write(|w| unsafe {w.bits(0x00000081)});//check 4 here in iomux receiver
        gpioa.dout31_0.write(|w| unsafe {w.bits(0x0000001)});
        gpioa.doe31_0.write(|w| unsafe {w.bits(0x0000001)});

        //set clock Config for adc
        ADC0::set_clkcfg(0x00001);
        ADC0::set_ctl0(0x03000000);
        ADC0::set_clkfreq(0x05);
        adc0.memctl[0].write(|w| unsafe {w.bits(0x00000202)});
        ADC0::set_ctl0(0x03010000);
        ADC0::set_scomp0(0x000001F4);
        ADC0::set_ctl0(0x03010001);
//worst case scenrio with temperature sensor
      //  Check if temperature readings are correct
       
       //remove jumper and cheeck datasheet with pin 
       //You can also use dc voltage for the reading
       // highlight specific code as a team
        //Initialize  VREF
        vref.clksel.write(|w|w.bits(0x08));// reset gpio info
        vref.ctl0.write(|w|w.bits(0x01));// reset gpio info
        vref.ctl1.write(|w|w.bits(0x01));// reset gpio info
// they turn off conversion off for some reason
// ADC0::set_ctl0(0x03000000);

        }
    ADC0::set_pwren(0x26000001);
    
    //enable control bit in CTL0 register
    //VREF::set_ctl0(0x1);
    //Select internal voltage with BuFConfig control bit in CTL0
    //VREF::set_ctl0(0x81);
    //Ready bit in CTL1 register to indicate it is ready

    //ask about external reference and start up time
}

pub fn turnoff_adc(){
    //this to save power
    ADC0::set_pwren(0x26000000);
}