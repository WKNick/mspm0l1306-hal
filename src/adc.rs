use mspm0l130x as pac; // the rust crate created for the MSPM0L for peripherals access

use core;
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


struct adc2;

//#[derive(Debug)]
struct adcinstruc;


//so the or can be used; operator overload for |
impl crate::adc::core::ops::BitOr<ADCInstructions> for ADCInstructions {
    type Output = adcinstruc;

    fn bitor(self, _rhs: ADCInstructions) -> adcinstruc {

        adcinstruc
    }
}

//enum for ADC instructions
enum ADCInstructions  {
    Add =0x1,
    Subtract = 0x2,
}


// Creates a type alias so it can be used in the functions below
type Commands = ADCInstructions;
//an example of how the type commmands works 
pub fn five() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Commands::Add | Commands::Subtract;
} 

//use crate::enums::commands;

//LED Demo
//For this, you will have to remove the jumper off of PA0 so that the led turns on when it is supposed to
// Here is the led demo code
/*
fn main(){
   adc::intialize_adc(); //initialize the adc
    let x = adc::read_voltage(); // this will read the amount of counts based on the voltage reference 3.3 V

   while x> 1735{// will put the led on if it is higher than 1735 counts
        adc::putonled();
}
 */
//now watch the led turn on 

pub fn read_voltage()->u32{
return ADC0::get_memres_0();
}
pub fn initialize_adc(){
        ADC0::set_rstctl(0x0);
        ADC0::set_pwren(0x26000001);
        //set clock Config for adc
        ADC0::set_clkcfg(0xA9000000);
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

pub fn putonled(){
    unsafe{
        let peripherals = pac::Peripherals::steal();
        let iomux = peripherals.IOMUX;
        iomux.pincm[0].write(|w| unsafe {w.bits(0x00000081)});//check 4 here in iomux receiver

    }
}