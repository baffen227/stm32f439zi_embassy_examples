#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
	bind_interrupts,
	can::{
		filter::Mask32, Can, ExtendedId, Fifo, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, StandardId, TxInterruptHandler
	},
	gpio::{Input, Pull},
	peripherals::CAN1,
	rcc::{
		AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllSource,
		Sysclk,
	},
};
// use embassy_time::Instant;
use panic_probe as _;

use embassy_stm32::time::Hertz;
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
	CAN1_RX0 => Rx0InterruptHandler<CAN1>;
	CAN1_RX1 => Rx1InterruptHandler<CAN1>;
	CAN1_SCE => SceInterruptHandler<CAN1>;
	CAN1_TX => TxInterruptHandler<CAN1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
	info!("Hello World!");

	// Use the nixpkg: stm32cubemx for clock config

	// if want to use external oscillator (HSE), need to solder a crystal on the X3
	// solder points on the demo board
	// TODO: change the clock settings for more
	// precise delay
	let mut config = embassy_stm32::Config::default();
	// We configure the STM32 to use an external oscillator crystal as the internal
	// clock HSI is not precise and stable enough for CAN communications.
	// On the nucleo board we can use the bypass HSE from the ST-LINK MCO as the
	// clock source. But for production we should use a real oscillator for HSE and
	// capacitors on the X3 connector. Note: the ST-LINK MCO frequency is 8 Mhz on
	// our nucleo board.
	config.rcc.hsi = false;
	config.rcc.hse = Some(Hse {
		freq: Hertz::mhz(8),
		mode: HseMode::Oscillator,
	});
	config.rcc.pll_src = PllSource::HSE;
	config.rcc.sys = Sysclk::PLL1_P;
	config.rcc.pll = Some(Pll {
		prediv: PllPreDiv::DIV4,
		mul: PllMul::MUL180,
		divp: Some(PllPDiv::DIV2),
		divq: None, // used by SPI3. 100Mhz.
		divr: None,
	});
	config.rcc.ahb_pre = AHBPrescaler::DIV1;

	config.rcc.apb1_pre = APBPrescaler::DIV4;
	config.rcc.apb2_pre = APBPrescaler::DIV2;

	let mut p = embassy_stm32::init(config);

	// The next two lines are a workaround for testing without transceiver.
	// To synchronise to the bus the RX input needs to see a high level.
	// Use `mem::forget()` to release the borrow on the pin but keep the
	// pull-up resistor enabled.
	let rx_pin = Input::new(&mut p.PA11, Pull::Up);
	core::mem::forget(rx_pin);

	let mut can = Can::new(p.CAN1, p.PD0, p.PD1, Irqs);

	can.modify_filters()
		.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

	can.modify_config()
        .set_loopback(false) // Receive own frames
        .set_silent(false)
        .set_bitrate(1_000_000);

	can.enable().await;

	let mut data: u8;
	loop {
		//let tx_frame = Frame::new_data(unwrap!(StandardId::new(i as _)), &[i]).unwrap();
		//let tx_ts = Instant::now();
		//can.write(&tx_frame).await;

		let envelope = can.read().await.unwrap();
		info!(
			"frame {:#04x}",
			envelope.frame.data()[0]
		);

		data = envelope.frame.data()[0];
		data += 1;

		let tx_frame = Frame::new_data(unwrap!(ExtendedId::new(data as _)), &[data]).unwrap();
		can.write(&tx_frame).await;

		// We can measure loopback latency by using receive timestamp in the `Envelope`.
		// Our frame is ~55 bits long (exlcuding bit stuffing), so at 1mbps loopback
		// delay is at least 55 us. When measured with `tick-hz-1_000_000` actual
		// latency is 80~83 us, giving a combined hardware and software overhead of ~25
		// us. Note that CPU frequency can greatly affect the result.
		//let latency = envelope
		//	.ts
		//	.saturating_duration_since(tx_ts);
		//info!(
		//	"loopback frame {=u8}, latency: {} us",
		//	envelope.frame.data()[0],
		//	latency.as_micros()
		//);

		//i = i.wrapping_add(1);
        //Timer::after_millis(500).await;
	}
}
