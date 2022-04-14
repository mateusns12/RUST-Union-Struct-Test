#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]                   // Yes. I can't stand compiler warnings
#![allow(unused_assignments)]           // And yes, im purposely doing unsafe coding
#![allow(unused_unsafe)]
#![allow(unused_imports)]

use std::mem::ManuallyDrop;

#[derive(Debug, Copy, Clone)] // no sense made
#[repr(C)]
struct Dados{
    f:ManuallyDrop<f32>,
    g:ManuallyDrop<f32>
}
#[derive(Copy, Clone)]
union Packet { 
    buffer:[u8;std::mem::size_of::<Dados>()],
    dados: ManuallyDrop<Dados>  
}

impl Packet{
    // pretty strange
    fn set_temp(&mut self, temp:ManuallyDrop<f32>){
        unsafe {
            self.dados.g = temp
        };
        println!("'g' received {}", ManuallyDrop::into_inner(temp));
    }
    // this is way better
    fn set_lamp(&mut self, lamp:f32){ 
        unsafe {
            self.dados.g = ManuallyDrop::new(lamp);  // <- this can change union initialization
        };
        println!("'g' received {}", lamp);
    }
}

fn main() {
    // Init packet - Too much texto
    // Can be changed by using ManuallyDrop::new() inside union declaration (theory)
    let mut u = Packet { dados: ManuallyDrop::new(Dados{f:ManuallyDrop::new(0.0),g:ManuallyDrop::new(0.0)}) };
    println!("\n");
    println!("Reading all union fields : {:?}",unsafe{u.dados});
    println!("Reading union initial buffer byte values {:?}",unsafe { u.buffer });
    unsafe {
        // Borrowing ownership to write value
        let b1 = &mut u.dados;
        *b1.f = 5.8 // lifetime ended
    };

    let mut value:f32 = 0.0;                            // Unecessary data
    let mut buffer:[u8;std::mem::size_of::<Dados>()];   // Unecessary data

    unsafe{
        // Acessing union fields
        value = ManuallyDrop::into_inner(u.dados.f);  // using into_inner for unstable field
        buffer = u.buffer;                                 // acessing stable field directly
    };

    println!("\t--------------------------------------------------------");
    println!("Reading union field 'f' directly : {:?}",unsafe{u.dados.f});
    println!("Reading union field 'f' value : {:?}",value);
    println!("Reading union buffer byte values : {:?}",buffer);
    println!("\t--------------------------------------------------------");
    //assert_eq!(20,std::mem::size_of::<Dados>());
    println!("Displaying size of union Packet : {} bytes",std::mem::size_of::<Packet>());
    println!("Displaying size of struct Dados : {} bytes",std::mem::size_of::<Dados>());
    println!("\t--------------------------------------------------------");
    u.set_temp(ManuallyDrop::new(36.8));
    println!("Reading union field 'g' directly after 'temp' operation: {:?}",unsafe {u.dados.g});
    println!("\t--------------------------------------------------------");
    u.set_lamp(56.2);
    println!("Reading union field 'g' directly after 'lamp' operation: {:?}",unsafe {u.dados.g});
    println!("\t--------------------------------------------------------");
    unsafe {buffer = u.buffer};
    println!("Reading union buffer byte values : {:?}",unsafe {u.buffer});
}


/*
// Original Struct 

struct Dados{
    pub temperatura:f32,
    umidade:f32,
    luminosidade:f32,
    pressao:f32,
    ph:f32
}
*/