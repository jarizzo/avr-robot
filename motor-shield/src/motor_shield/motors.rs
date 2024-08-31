use arduino_hal::{
    hal::port,
    port::{mode, Pin},
    simple_pwm::{Timer0Pwm,Timer2Pwm}
};

use super::digital_output::DigitalOutput;


pub enum MotorCommands {
    FORWARD,
    BACKWARD,
    RELEASE,
}
pub enum MotorPin {
    Motor1(Pin<mode::PwmOutput<Timer2Pwm>, port::PB3>),
    Motor2(Pin<mode::PwmOutput<Timer2Pwm>, port::PD3>),
    Motor3(Pin<mode::PwmOutput<Timer0Pwm>, port::PD6>),
    Motor4(Pin<mode::PwmOutput<Timer0Pwm>, port::PD5>),
}

impl MotorPin {
    fn get_ab(&self) -> (u8, u8) {
        match self {
            // #define MOTOR1_A 2 0x0000_0100
            // #define MOTOR1_B 3 0x0000_1000
            Self::Motor1(_) => (1 << 2, 1 << 3),
            // #define MOTOR2_A 1 0x0000_0010
            // #define MOTOR2_B 4 0x0001_0000
            Self::Motor2(_) => (1 << 1, 1 << 4),
            // #define MOTOR3_A 5 0x0010_0000
            // #define MOTOR3_B 7 0x1000_0000
            Self::Motor3(_) => (1 << 5, 1 << 7),
            // #define MOTOR4_A 0 0x0000_0001
            // #define MOTOR4_B 6 0x0100_0000
            Self::Motor4(_) => (1 << 0, 1 << 6),
        }
    }

    fn enable(&mut self) {
        match self {
            Self::Motor1(pin) => {
                pin.enable();
            }
            Self::Motor2(pin) => {
                pin.enable();
            }
            Self::Motor3(pin) => {
                pin.enable();
            }
            Self::Motor4(pin) => {
                pin.enable();
            }
        }
    }

    fn disable(&mut self) {
        match self {
            Self::Motor1(pin) => {
                pin.disable();
            }
            Self::Motor2(pin) => {
                pin.disable();
            }
            Self::Motor3(pin) => {
                pin.disable();
            }
            Self::Motor4(pin) => {
                pin.disable();
            }
        }
    }

    fn set_speed(&mut self, speed: u8) {
        match self {
            Self::Motor1(pin) => {
                pin.set_duty(speed);
            }
            Self::Motor2(pin) => {
                pin.set_duty(speed);
            }
            Self::Motor3(pin) => {
                pin.set_duty(speed);
            }
            Self::Motor4(pin) => {
                pin.set_duty(speed);
            }
        }
    }
}

pub struct Motor {
    pin: MotorPin,
    output: *mut DigitalOutput,
}

impl Motor {
    pub fn new(pin: MotorPin, output: *mut DigitalOutput,) -> Self {
        Self {
            pin,
            output
        }
    }

    pub fn run(&self, command: MotorCommands) {
        let (a, b) = self.pin.get_ab();
        let output = unsafe { self.output.as_mut().unwrap()};

        match command {
            MotorCommands::FORWARD => {
                output.or(a);
                output.and_not(b);
            }
            MotorCommands::BACKWARD => {
                output.and_not(a);
                output.or(b);
            }
            MotorCommands::RELEASE => {
                output.and_not(a);
                output.and_not(b);
            }
        }

        output.transmit();
    }

    pub fn enable(&mut self) {
        self.pin.enable()
    }

    pub fn disable(&mut self) {
        self.pin.disable();
    }

    pub fn speed(&mut self, speed: u8) {
        self.pin.set_speed(speed);
    }
}