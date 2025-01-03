#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
	exti::ExtiInput,
	gpio::{Input, Pull},
};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
	let p = embassy_stm32::init(Default::default());
	info!("Hello World!");

	// TODO: after upgrading to the following commit, we can use the new API
	// https://github.com/embassy-rs/embassy/commit/3387ee7238f1c9c4eeccb732ba543cbe38ab7ccd
	// let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);

	let button = Input::new(p.PC13, Pull::Down);
	let mut button = ExtiInput::new(button, p.EXTI13);

	info!("Press the USER button...");

	loop {
		button
			.wait_for_rising_edge()
			.await;
		info!("Pressed!");
		button
			.wait_for_falling_edge()
			.await;
		info!("Released!");
	}
}
