# baremetal_microcontroller
A bare metal application written in Rust for the STM32F411x microcontroller.

## Het quickstart project
Dit project is gebaseerd op de repo van [Cortex-M-Quickstart](https://github.com/rust-embedded/cortex-m-quickstart).

De volgende dingen zijn aangepast en toegevoegd:
- [x] Target is ingesteld voor de STM32F411 naar thumbv7em-none-eabihf in  [config.toml](.cargo/config.toml)
- [x] Memory map is aangepast naar de STM32F411 in [memory.x](memory.x)
- [x] SVD bestand is toegevoegd voor de STM32F411, dit beschrijf alle registers. [stm32f411.svd](.vscode/stm32f411.svd)
- [x] VS code task is aangepast om main te kunnen compileren en debuggen in [tasks.json](.vscode/tasks.json) 
- [x] ARM semihosting is aangezet voor openocd in [launch.json](.vscode/launch.json)
- [x] Juiste dependencies aangezet voor heap allocation
- [x] Allocator.rs voorbeeld aangepast voor gebruik Box()
