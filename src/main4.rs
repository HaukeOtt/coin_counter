/* fn main() {
    println!("Hello, holger!");
}
*/

use std::error::Error;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use rppal::gpio::Trigger;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
//const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {

	println!("Blinking an LED on a {} ended.", DeviceInfo::new()?.model());    
	
	let pin_num = 23; 
  	let mut pin = Gpio::new()?.get(pin_num)?.into_input_pulldown();
	
	let pin_num2 = 24; 
  	let mut pin3 = Gpio::new()?.get(pin_num)?.into_input_pulldown();
	
	let pin_num3 = 25; 
  	let mut pin3 = Gpio::new()?.get(pin_num)?.into_input_pulldown();
	
	thread::spawn(||
		loop {
 		
		pin.set_interrupt(Trigger::RisingEdge);
		println!("pool output: {:?}", pin.poll_interrupt(true, Some(Duration::from_secs(10))));
  	
		}	
	
	);

	loop {
 		
		pin2.set_interrupt(Trigger::RisingEdge);
		println!("pool output: {:?}", pin2.poll_interrupt(true, Some(Duration::from_secs(10))));
  	
	}	
  	
	println!("Blinking an LED on a {} ended.", DeviceInfo::new()?.model());

    	Ok(())
}

