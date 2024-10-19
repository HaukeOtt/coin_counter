use rppal::gpio::Gpio;
use rppal::gpio::Trigger;
use std::error::Error;
use std::sync::mpsc;
use std::time::Instant;
//use std::thread;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
//const GPIO_LED: u8 = 23;

struct Signal {
	time: u128,
	pin: u8,
	lvl: rppal::gpio::Level,
}

fn pr_signal(signal: &Signal) {
	println!(
		"Pin: {}\tSignal: {}\tLevel: {}",
		signal.pin, signal.time, signal.lvl
	);
}

fn main() -> Result<(), Box<dyn Error>> {
	let (tx23, rx23): (
		std::sync::mpsc::Sender<Signal>,
		std::sync::mpsc::Receiver<Signal>,
	) = mpsc::channel();
	std::thread::spawn(move || {
		let pin_num1 = 23;
		let pin_num2 = 24;
		let pin_num3 = 25;

		let mut pin1 = Gpio::new()
			.expect("Gpio::new()")
			.get(pin_num1)
			.expect("get()")
			.into_input_pulldown();
		let mut pin2 = Gpio::new()
			.expect("Gpio::new()")
			.get(pin_num2)
			.expect("get()")
			.into_input_pulldown();
		let mut pin3 = Gpio::new()
			.expect("Gpio::new()")
			.get(pin_num3)
			.expect("get()")
			.into_input_pulldown();

		pin1.set_interrupt(Trigger::Both).expect("set_interrupt()");
		pin2.set_interrupt(Trigger::Both).expect("set_interrupt()");
		pin3.set_interrupt(Trigger::Both).expect("set_interrupt()");

		let mut timer23 = Instant::now();
		let mut timer24 = Instant::now();
		let mut timer25 = Instant::now();

		loop {
			let (i_pin, pin_lev) = Gpio::new()
				.expect("Gpio::new()")
				.poll_interrupts(&[&pin1, &pin2, &pin3], true, None)
				.unwrap()
				.unwrap();

			match i_pin.pin() {
				23 => {
					let signal = Signal {
						time: timer23.elapsed().as_millis(),
						pin: 23,
						lvl: pin_lev,
					};
					timer23 = Instant::now();
					tx23.send(signal).unwrap();
				}
				24 => {
					let _signal = Signal {
						time: timer24.elapsed().as_millis(),
						pin: 24,
						lvl: pin_lev,
					};
					timer24 = Instant::now();
					//tx24.send(signal).unwrap();
				}
				25 => {
					let _signal = Signal {
						time: timer25.elapsed().as_millis(),
						pin: 25,
						lvl: pin_lev,
					};
					timer25 = Instant::now();
					//tx25.send(signal).unwrap();
				}
				_ => {
					let _signal = Signal {
						time: 0,
						pin: i_pin.pin(),
						lvl: pin_lev,
					};
					//tx25.send(signal).unwrap();
				}
			}
		}
	});
	let (tx_cent, rx_cent): (
		std::sync::mpsc::Sender<u16>,
		std::sync::mpsc::Receiver<u16>,
	) = mpsc::channel();
	std::thread::spawn(move || {
		let mut cou_signal: u8 = 0;
		let mut signal: Signal;

		loop {
			signal = rx23.recv().unwrap();
			if signal.time < 72 && signal.time > 68 {
				cou_signal += 1;

				loop {
					match rx23.recv_timeout(std::time::Duration::from_millis(150)) {
						Ok(signal) => {
							if signal.time < 72 && signal.time > 68 {
								cou_signal += 1;
							} else if signal.time > 102 && signal.time < 98 {
								break;
							}
						}
						Err(_) => break,
					}
				}
				match cou_signal {
					1 => tx_cent.send(10).unwrap(),
					2 => tx_cent.send(20).unwrap(),
					3 => tx_cent.send(50).unwrap(),
					4 => tx_cent.send(100).unwrap(),
					5 => tx_cent.send(200).unwrap(),
					_ => tx_cent.send(0).expect("test1"),
				}
				cou_signal = 0;
				
			}
		}
	});

	let mut cent = 0;
	loop{
	cent +=	rx_cent.recv().expect("test2");
	println!("{}.{}€", cent / 100, cent % 100);
	}
	//Ok(())
}
