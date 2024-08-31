#![no_std]

mod motor_shield;

pub use crate::motor_shield::MotorShield;
pub use crate::motor_shield::layout::{ShieldLayout, MotorPort};
pub use crate::motor_shield::motors::MotorCommands;
pub use crate::motor_shield::steppers::{StepperDirection, StepperStyle};

#[macro_export]
macro_rules! init_ams {
    ($layout:expr, $p:expr, $pins:expr) => {
        MotorShield::new(
            $layout,
            $p.TC0, $p.TC1, $p.TC2, 
            $pins.d3, $pins.d4, $pins.d5, $pins.d6, $pins.d7, $pins.d8, $pins.d9, $pins.d10, $pins.d11, $pins.d12,
        )
    };
}