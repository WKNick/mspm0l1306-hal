use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access


use paste::paste;

use crate::generate_set_functions;
use crate::generate_get_functions;


generate_set_functions!(ADC0, fsub_0, fpub_1, pwren, rstctl, clkcfg, ctl0, clkfreq, scomp0, scomp1, ctl1, ctl2, ctl3, refcfg, wclow, wchigh); 
generate_get_functions!(ADC0, fsub_0, fpub_1, pwren, stat, clkcfg, desc, ctl0, scomp0, scomp1, ctl1, ctl2, ctl3, refcfg, wclow, wchigh);//, fsctl, gctl missing regs

//generate_get_functions!(ADC0_SVT, fifodata);//, fsctl, gctl missing regs
//put in clkfrq later

//use the adc0_svt. memres[] to get a value and adc0.memctl[] to read/write a value
//iidx, imask, ris, mis iset, iclr, intctl, ccpd, odis, cclkctl
use crate::generate_set_functions_list;
use crate::generate_get_functions_list;

generate_get_functions_list!(ADC0, memctl, 0, memctl, 1, memctl, 2, memctl, 3);
generate_set_functions_list!(ADC0, memctl, 0, memctl, 1, memctl, 2, memctl, 3);

use crate::generate_set_functions_list_specify;
use crate::generate_get_functions_list_specify;

generate_get_functions_list_specify!(ADC0, ADC0_SVT, memres, 0, memres, 1, memres, 2, memres, 3);




pub struct ADC0;

pub fn read_voltage()->u32{
return ADC0::get_memres_0();
}

pub fn initialize_adc(){

        
        ADC0::set_rstctl(0xB1000003);
        ADC0::set_pwren(0x26000001);

        //set clock Config for adc
        ADC0::set_clkcfg(0b10101001000000000000000000000000);
        ADC0::set_ctl0(0x03000000);
        ADC0::set_clkfreq(0x05);

        ADC0::set_ctl1(0b10000000000000000);
        ADC0::set_memctl_0(0b0000000000000000000000000);//trying 0 for the trigger
        ADC0::set_ctl1(0b000000100000000000000000);
        ADC0::set_ctl0(0x03010000);
        ADC0::set_scomp0(0x00000174);
        //1F4
        ADC0::set_ctl0(0x030100301);
        ADC0::set_ctl2(0b00000000000);
        ADC0::set_ctl1(0b00000000100000000100000000);
//worst case scenrio with temperature sensor
      //  Check if temperature readings are correct
       
       //tasks
       //turn off averaging and fifo
       //made VDD the voltage reference
       //After, check for not too much noise
       //check voltage 3.3, 2.5, 2, 1.5, 1, 500mV. 
       //make circuit 
       //check voltage/counts
       //remove jumper and cheeck datasheet with pin 
       //You can also use dc voltage for the reading
       // highlight specific code as a team
        //Initialize  VREF
        /* 
        vref.clksel.write(|w|w.bits(0x08));// reset gpio info
        vref.ctl0.write(|w|w.bits(0b0000001));// reset gpio info
        vref.ctl1.write(|w|w.bits(0x01));// reset gpio info
        */
// they turn off conversion off for some reason
// ADC0::set_ctl0(0x03000000);
    
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