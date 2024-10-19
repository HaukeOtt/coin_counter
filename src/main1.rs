/*
fn main() {
    println!("Hello, holger!");
}
*/

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
//const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    // Blink the LED by setting the pin's logic level high for 500 ms.
    
/*    for x in 1..20 {
    	
    	let mut pin = Gpio::new()?.get(x)?.into_output();

    	println!("pin: {}", x);
    
    	pin.set_high();
    	thread::sleep(Duration::from_millis(1000));
    	pin.set_low();
    }	
*/

	let pin_num = 13; 
  	let mut pin = Gpio::new()?.get(pin_num)?.into_input();

    	println!("pin: {}", pin_num);
    
    	pin.set_high();
    	thread::sleep(Duration::from_millis(1000));
    	pin.set_low();
  
    println!("Blinking an LED on a {} ended.", DeviceInfo::new()?.model());

    Ok(())
}

