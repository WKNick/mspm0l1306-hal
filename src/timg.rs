
use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;

generate_set_functions!(TIMG0, fsub_0, fsub_1, fpub_0, fpub_1, pwren, rstctl, clkdiv, clksel, pdbgctl, imask, iset, iclr, evt_mode, ccpd, odis, cclkctl, cps, cttrigctl, cttrig, ctr, ctrctl, load);//, fsctl, gctl missing regs
generate_get_functions!(TIMG0, fsub_0, fsub_1, fpub_0, fpub_1, pwren, stat, clkdiv, clksel, pdbgctl, iidx, imask, ris, mis, evt_mode, desc, ccpd, odis, cclkctl, cps, cpsv, cttrigctl, ctr, ctrctl, load);//, fsctl, gctl missing regs

generate_set_functions!(TIMG1, fsub_0, fsub_1, fpub_0, fpub_1, pwren, rstctl, clkdiv, clksel, pdbgctl, imask, iset, iclr, evt_mode, ccpd, odis, cclkctl, cps, cttrigctl, cttrig, ctr, ctrctl, load);//, fsctl, gctl missing regs
generate_get_functions!(TIMG1, fsub_0, fsub_1, fpub_0, fpub_1, pwren, stat, clkdiv, clksel, pdbgctl, iidx, imask, ris, mis, evt_mode, desc, ccpd, odis, cclkctl, cps, cpsv, cttrigctl, ctr, ctrctl, load);//, fsctl, gctl missing regs

generate_set_functions!(TIMG2, fsub_0, fsub_1, fpub_0, fpub_1, pwren, rstctl, clkdiv, clksel, pdbgctl, imask, iset, iclr, evt_mode, ccpd, odis, cclkctl, cps, cttrigctl, cttrig, ctr, ctrctl, load);//, fsctl, gctl missing regs
generate_get_functions!(TIMG2, fsub_0, fsub_1, fpub_0, fpub_1, pwren, stat, clkdiv, clksel, pdbgctl, iidx, imask, ris, mis, evt_mode, desc, ccpd, odis, cclkctl, cps, cpsv, cttrigctl, ctr, ctrctl, load);//, fsctl, gctl missing regs

generate_set_functions!(TIMG4, fsub_0, fsub_1, fpub_0, fpub_1, pwren, rstctl, clkdiv, clksel, pdbgctl, imask, iset, iclr, evt_mode, ccpd, odis, cclkctl, cps, cttrigctl, cttrig, ctr, ctrctl, load);//, fsctl, gctl missing regs
generate_get_functions!(TIMG4, fsub_0, fsub_1, fpub_0, fpub_1, pwren, stat, clkdiv, clksel, pdbgctl, iidx, imask, ris, mis, evt_mode, desc, ccpd, odis, cclkctl, cps, cpsv, cttrigctl, ctr, ctrctl, load);//, fsctl, gctl missing regs


use crate::generate_set_functions_list;
use crate::generate_get_functions_list;

generate_set_functions_list!(TIMG0, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);
generate_get_functions_list!(TIMG0, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);

generate_set_functions_list!(TIMG1, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);
generate_get_functions_list!(TIMG1, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);

generate_set_functions_list!(TIMG2, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);
generate_get_functions_list!(TIMG2, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);

generate_set_functions_list!(TIMG4, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);
generate_get_functions_list!(TIMG4, cc_01, 0, cc_01, 1, ccctl_01, 0, ccctl_01, 1, octl_01, 0, octl_01, 1, ccact_01, 0, ccact_01, 1, ifctl_01, 0, ifctl_01, 1);


pub struct TIMG0;
pub struct TIMG1;
pub struct TIMG2;
pub struct TIMG4;






impl TIMG0{
///enables peripheral 
    pub fn enable(){// resets timg0 registers and enables them
        TIMG0::set_rstctl(0xb1000003);// reset gpio info
        TIMG0::set_pwren(0x26000001);// enable gpio with reset key
    }

///configures registers to count up/down to 7D0 with reset enabled
    pub fn basic_config(){


        TIMG0::set_clksel(0x00000008);
    
        TIMG0::set_cclkctl(0x00000001);
    
        TIMG0::set_ctrctl(0x00000012);
    
        TIMG0::set_load(0x000007D0);
    
        TIMG0::set_ccpd(0x00000001);
    
        TIMG0::set_odis(0x00000000);
    
        TIMG0::set_ctrctl(0x00000013);
    
    }


}


impl TIMG1{
///enables peripheral 
    pub fn enable(){// resets timg0 registers and enables them
        TIMG1::set_rstctl(0xb1000003);// reset gpio info
        TIMG1::set_pwren(0x26000001);// enable gpio with reset key
    }

///configures registers to count up/down to 7D0 with reset enabled
    pub fn basic_config(){

        
        TIMG1::set_clksel(0x00000008);
    
        TIMG1::set_cclkctl(0x00000001);
    
        TIMG1::set_ctrctl(0x00000012);
    
        TIMG1::set_load(0x000007D0);
    
        TIMG1::set_ccpd(0x00000001);
    
        TIMG1::set_odis(0x00000000);
    
        TIMG1::set_ctrctl(0x00000013);
    
    }


}

impl TIMG2{
///enables peripheral 
    pub fn enable(){// resets timg0 registers and enables them
        TIMG2::set_rstctl(0xb1000003);// reset gpio info
        TIMG2::set_pwren(0x26000001);// enable gpio with reset key
    }

///configures registers to count up/down to 7D0 with reset enabled
    pub fn basic_config(){

        
        TIMG2::set_clksel(0x00000008);
    
        TIMG2::set_cclkctl(0x00000001);
    
        TIMG2::set_ctrctl(0x00000012);
    
        TIMG2::set_load(0x000007D0);
    
        TIMG2::set_ccpd(0x00000001);
    
        TIMG2::set_odis(0x00000000);
    
        TIMG2::set_ctrctl(0x00000013);
    
    }


}

impl TIMG4{
///enables peripheral 
    pub fn enable(){
        TIMG4::set_rstctl(0xb1000003);// reset gpio info
        TIMG4::set_pwren(0x26000001);// enable gpio with reset key
    }

///configures registers to count up/down to 7D0 with reset enabled
    pub fn basic_config(){

        
        TIMG4::set_clksel(0x00000008);
    
        TIMG4::set_cclkctl(0x00000001);
    
        TIMG4::set_ctrctl(0x00000012);
    
        TIMG4::set_load(0x000007D0);
    
        TIMG4::set_ccpd(0x00000001);
    
        TIMG4::set_odis(0x00000000);
    
        TIMG4::set_ctrctl(0x00000013);
    
    }


}