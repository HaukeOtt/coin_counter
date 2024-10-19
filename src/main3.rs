/* fn main() {
    println!("Hello, holger!");
}
*/

use std::error::Error;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use rppal::gpio::Trigger;
use std::thread;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
//const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {

	println!("Started program");    
	
	const pin_nums:[u8; 3] = [23,24,25];
 	const pin_num0:u8 = 23;
 	const pin_num1:u8 = 24;
 	const pin_num2:u8 = 25;
 		
	thread::spawn(
		move||{  
  			let mut pin0 = Gpio::new().unwrap().get(pin_num0).unwrap().into_input_pulldown();
			pin0.set_interrupt(Trigger::RisingEdge);
		
			loop{
				let pinout = pin0.poll_interrupt(true,None);
				println!("pooled 0: {:?}", pinout);
			}
		}
	);

	thread::spawn(
		move||{
  			let mut pin1 = Gpio::new().unwrap().get(pin_num1).unwrap().into_input_pulldown();
			pin1.set_interrupt(Trigger::RisingEdge);

			loop{
				let pinout = pin1.poll_interrupt(true,None);
				println!("pooled 1: {:?}", pinout);
			}
		}
	);


	let mut pin2 = Gpio::new().unwrap().get(pin_num2).unwrap().into_input_pulldown();
	//pin2.set_interrupt(Trigger::RisingEdge);		

	loop{
		let pinout = pin2.poll_interrupt(true,None);
		println!("pooled 2: {:?}", pinout);
	}

	println!("ended");
   	Ok(())
}
