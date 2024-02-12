
/* =
pub union Vector {
     handler: unsafe extern "C" fn(),
     reserved: usize,
 }

 extern "C" {
     fn Sw();
 }

 #[link_section = ".vector_table.interrupts"]
 #[no_mangle]
 pub static __INTERRUPTS: [Vector; 5] = [
     // 0-1: Reserved
     Vector { reserved: 0 },
     Vector { handler: Sw },

     // 2: Foo
     Vector { reserved: 0 },

     // 3: Reserved
     Vector { reserved: 0 },

     // 4: Bar
     Vector { reserved: 0 },
 ];

 */
