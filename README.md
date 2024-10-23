# stm32f439zi_embassy_examples

```bash
# build
$ cargo build --bin blinky [--release]

# build & flash memory
$ cargo run --bin blinky [--release]
$ probe-rs run --chip STM32F439ZITx target/thumbv7em-none-eabihf/debug/blinky

# attach
$ probe-rs attach --chip STM32F439ZITx target/thumbv7em-none-eabihf/debug/blinky
```
