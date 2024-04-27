use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;


generate_set_functions!(OPA0, pwren, rstctl, clkovr, pwrctl, ctl, cfg); 
generate_get_functions!(OPA0, pwren, stat, clkovr, pwrctl, ctl, cfg);

//iidx, imask, ris, mis iset, iclr, intctl, ccpd, odis, cclkctl
use crate::generate_set_functions_list;

//generate_set_functions_list!(TIMG0, cc_01, 0, cc_01, 1, cc_23, 0, cc_23, 1, cc_45, 0, cc_45, 1, ccctl_01, 0, ccctl_01, 1, ccctl_23, 0, ccctl_23, 1, ccctl_45, 0, ccctl_45, 1, octl_01, 0, octl_01, 1, octl_23, 0, octl_23, 1, ccact_01, 0, ccact_01, 1, ccact_23, 0, ccact_23, 1, ifctl_01, 0, ifctl_01, 1, ifctl_23, 0, ifctl_23, 1);




pub struct OPA0;
pub fn initialize_opa(){
    OPA0::set_pwren(0x26000001);
    //enable control bit in CTL0 register
    //VREF::set_ctl0(0x1);
    //Select internal voltage with BuFConfig control bit in CTL0
    //VREF::set_ctl0(0x81);
    //Ready bit in CTL1 register to indicate it is ready

    //ask about external reference and start up time
}
pub fn turnoff_OPA(){
    //this to save power
    OPA0::set_pwren(0x26000000);
}