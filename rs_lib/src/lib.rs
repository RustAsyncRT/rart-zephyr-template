#![feature(core_ffi_c)]
#![no_std]
#![no_builtins]

extern crate panic_halt;
extern crate rart_rs;
extern crate rart_macros;
extern crate alloc;

use rart_rs::*;
use rart_macros::channel;

#[derive(Debug)]
pub enum BtnState {
    Press = 1,
    DoublePress = 2,
}

#[derive(Debug)]
enum LedState {
    On = 1,
    _Off = 0,
}

rart_macros::channel_pub_def!(btn_msgq, BtnState, 10);
rart_macros::channel_def!(led_msgq, LedState, 5);

async fn task1() -> TaskResult {
    loop {
        delay_secs(10).await;
        channel!(btn_msgq).send(BtnState::Press).await?;
    }
}

async fn task2() -> TaskResult {
    loop {
        let _led_state = channel!(led_msgq).recv().await;
        delay_secs(3).await;
    }
}

#[rart_macros::entry]
#[rart_macros::tasks(task1, task2)]
#[rart_macros::channels(btn_msgq, led_msgq)]
async fn main_task() -> TaskResult {
    loop {
        let _btn_state = channel!(btn_msgq).recv().await;
        channel!(led_msgq).send(LedState::On).await?;
    }
}
