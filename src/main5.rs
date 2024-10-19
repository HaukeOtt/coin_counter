
#[warn(unused_must_use)]
use std::error::Error;
use rppal::gpio::Gpio;
use rppal::gpio::Trigger;
use std::time::{Duration, Instant};

//use std::thread;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
//const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
		
	let pin_num1 = 23; 
	let pin_num2 = 24; 
	let pin_num3 = 25; 

  	let mut pin1 = Gpio::new()?.get(pin_num1)?.into_input_pulldown();	
  	let mut pin2 = Gpio::new()?.get(pin_num2)?.into_input_pulldown();	
  	let mut pin3 = Gpio::new()?.get(pin_num3)?.into_input_pulldown();	

	pin1.set_interrupt(Trigger::Both);
	pin2.set_interrupt(Trigger::Both);
	pin3.set_interrupt(Trigger::Both);
	
	let mut counter: u32 = 0;
	let mut last_pin:u8 = 0;
	
	loop {
		let timer = Instant::now();
				
		let (i_pin,pin_lev) = Gpio::new()?
			.poll_interrupts(&[&pin1,&pin2,&pin3],true, None)
			.unwrap().unwrap();
		let pin_num = i_pin.pin();
		
		if pin_num == last_pin{
			counter += 1;
		}else{
			counter = 0;
			last_pin = pin_num.clone();
		}
		
		if pin_num == 23{
  			println!("\x1b[32m█pin: {} \t{}\t{}\t{:?}\x1b[0m", pin_num,counter + 1, timer.elapsed().as_millis(),pin_lev); 
		}else {
			println!("█pin: {} \t{}\t{}\t{:?}", pin_num,counter + 1, timer.elapsed().as_millis(),pin_lev); 		
		}
		std::thread::sleep(std::time::Duration::from_millis(10));
	}
		
	//	
}
