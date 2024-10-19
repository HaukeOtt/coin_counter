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
	let (tx, rx): (
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
			let mut signal = Signal {
				time: 0,
				pin: 0,
				lvl: rppal::gpio::Level::Low,
			};

			let (i_pin, pin_lev) = Gpio::new()
				.expect("Gpio::new()")
				.poll_interrupts(&[&pin1, &pin2, &pin3], true, None)
				.unwrap()
				.unwrap();
			signal.pin = i_pin.pin();
			signal.lvl = pin_lev;

			match signal.pin {
				23 => {
					signal.time = timer23.elapsed().as_millis();
					timer23 = Instant::now()
				}
				24 => {
					signal.time = timer24.elapsed().as_millis();
					timer24 = Instant::now()
				}
				25 => {
					signal.time = timer25.elapsed().as_millis();
					timer25 = Instant::now()
				}
				_ => signal.time = 0,
			}

			tx.send(signal).unwrap();

			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	});

	let (tx2, rx2): (
		std::sync::mpsc::Sender<Signal>,
		std::sync::mpsc::Receiver<Signal>,
	) = mpsc::channel();
	std::thread::spawn(move || {
		let mut timer = Instant::now();
		let mut signal2: Signal;
		let mut signal_send = false;
		let mut signal_send_by_thread;
		loop {
			signal_send_by_thread = false;
			let tx2c = tx2.clone();
			println!("Reset");
			std::thread::spawn(move || {
				std::thread::sleep(std::time::Duration::from_millis(202));
				if !signal_send_by_thread && (timer.elapsed().as_millis() > 150) {
					tx2c.send(Signal {
						time: 0,
						pin: 23,
						lvl: rppal::gpio::Level::Low,
					})
					.unwrap();
					signal_send_by_thread= true;
					println!("{}",timer.elapsed().as_millis());
				}
			});
			signal2 = rx.recv().unwrap();
			signal_send = true;
			tx2.send(signal2).unwrap();
			timer = Instant::now();
		}
	});

	let mut cou_signal: u8 = 0;
	let mut cent: u32 = 0;
	let mut signal: Signal;
	loop {
		//wartet auf signal

		signal = rx2.recv().unwrap();

		// gibt signal zum stdout aus
		pr_signal(&signal);
		// guckt ob vergangene Zeit zum vorherigen signal zwischen 69 und 71 liegt
		if signal.time < 72 && signal.time > 68 {
			// zählt counter +1
			cou_signal += 1;
		//  guckt ob vergangene Zeit zum vorherigen signal zwischen 99 und 101 liegt
		} else if !(signal.time < 102 && signal.time > 98) {
			//erhöht cent wert den counter-wert entsprechend
			match cou_signal {
				1 => cent += 10,
				2 => cent += 20,
				3 => cent += 50,
				4 => cent += 100,
				5 => cent += 200,
				_ => cent = cent,
			}
			//setzt counter auf 0
			cou_signal = 0;
			// gibt eingeworfenen betrag
			//println!("cent: {}", cent);
		}
	}

	//Ok(())
}
