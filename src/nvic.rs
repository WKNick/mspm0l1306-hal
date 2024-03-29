
use cortex_m; // I think this is what allows us to create main

use cortex_m_rt::exception; // I think this is what allows us to create main

use mspm0l130x as pac;

use cortex_m::interrupt;
pub unsafe trait InterruptNumber: Copy {
    /// Return the interrupt number associated with this variant.
    ///
    /// See trait documentation for safety requirements.
    fn number(self) -> u16;
}


pub struct NVIC;

impl NVIC{
    

    pub unsafe fn set_priority<I>(&mut self, interrupt: I, prio: u8)
    where
        I: InterruptNumber,
    {

        pac::CorePeripherals::steal().NVIC.icer[1].write(0x03);
        //let abda = (0x01 as u32) as InterruptNumber;
        //let inmb: interrupt::InterruptNumber = cortex_m::interrupt::InterruptNumber::new(interrupt.number());
        //cortex_m::Peripherals::take().unwrap().NVIC.set_priority(cortex_m::interrupt::InterruptNumber::new(interrupt.number()), prio);
    }
    


    //macro for all iser icer ispr icpr values





}



#[macro_export]
macro_rules! generate_nvic_functions {
    ($($function:ident, $vall:expr),*) => {
        $(
            impl NVIC {
                
               pub fn set_ $function _$vall (value: u32) {
                    unsafe{
                    pac::Peripherals::steal().$struct_name.$function.write(|w| w.bits(value));
                    }
                }

                pub fn get_ $function _$vall (value: u32) {
                    unsafe{
                    pac::Peripherals::steal().$struct_name.$function.write(|w| w.bits(value));
                    }
                }

            

            }
        )*
    };
}