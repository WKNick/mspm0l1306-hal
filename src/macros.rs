//! Macros for generating functions from the PAC
//! 
//! 
//! 
//! 
#[macro_export]
macro_rules! generate_set_functions {
    ($struct_name:ident, $($function:ident),*) => {
        $(
            impl $struct_name {
                paste! {
                
               pub fn [<set_ $function >] (value: u32) {
                    unsafe{
                    pac::Peripherals::steal().$struct_name.$function.write(|w| w.bits(value));
                    }
                }

            }

            }
        )*
    };
}

#[macro_export]
macro_rules! generate_get_functions {
    ($struct_name:ident, $($function:ident),*) => {
        $(
            impl $struct_name {
                paste! {
                
               pub fn [<get_ $function >] () -> u32 {
                    unsafe{
                    return pac::Peripherals::steal().$struct_name.$function.read().bits();
                    }
                }

            }

            }
        )*
    };
}


#[macro_export]
macro_rules! generate_set_functions_list {
    ($struct_name:ident, $($function:ident, $vall:expr),*) => {
        $(
            impl $struct_name {
                paste! {
                
               pub fn [<set_ $function _ $vall>] (value: u32) {
                    unsafe{
                    pac::Peripherals::steal().$struct_name.$function [$vall].write(|w| w.bits(value));
                    }
                }

            }

            }
        )*
    };
}

#[macro_export]
macro_rules! generate_get_functions_list {
    ($struct_name:ident, $($function:ident, $vall:expr),*) => {
        $(
            impl $struct_name {
                paste! {
                
               pub fn [<get_ $function _ $vall>] () -> u32 {
                    unsafe{
                    return pac::Peripherals::steal().$struct_name.$function [$vall].read().bits();
                    }
                }

            }

            }
        )*
    };
}


#[macro_export]
macro_rules! generate_set_functions_list_specify {
    ($struct_name:ident,$real_struct_name:ident, $($function:ident, $vall:expr),*) => {
        $(
            impl $struct_name {
                paste! {
                
               pub fn [<set_ $function _ $vall>] (value: u32) {
                    unsafe{
                    pac::Peripherals::steal().$real_struct_name.$function [$vall].write(|w| w.bits(value));
                    }
                }

            }

            }
        )*
    };
}

#[macro_export]
macro_rules! generate_get_functions_list_specify {
    ($struct_name:ident,$real_struct_name:ident, $($function:ident, $vall:expr),*) => {
        $(
            impl $struct_name {
                paste! {
                
               pub fn [<get_ $function _ $vall>] () -> u32 {
                    unsafe{
                    return pac::Peripherals::steal().$real_struct_name.$function [$vall].read().bits();
                    }
                }

            }

            }
        )*
    };
}