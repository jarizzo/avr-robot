use arduino_hal::{
    hal::port,
    port::{mode, Pin},
    simple_pwm::Timer1Pwm
};

pub enum ServoPin {
    Servo1(Pin<mode::PwmOutput<Timer1Pwm>, port::PB2>),
    Servo2(Pin<mode::PwmOutput<Timer1Pwm>, port::PB1>),
}

impl ServoPin {
    pub fn enable(&mut self) {
        match self {
            Self::Servo1(pin) => {
                pin.enable();
            }
            Self::Servo2(pin) => {
                pin.enable();
            }
        }
    }

    pub fn disable(&mut self) {
        match self {
            Self::Servo1(pin) => {
                pin.disable();
            }
            Self::Servo2(pin) => {
                pin.disable();
            }
        }
    }

    pub fn set_angle(&mut self, angle: u8) {
        match self {
            Self::Servo1(pin) => {
                pin.set_duty(angle);
            }
            Self::Servo2(pin) => {
                pin.set_duty(angle);
            }
        }
    }
}

pub struct Servo {
    pin: ServoPin,
}

impl Servo {
    pub fn new(pin: ServoPin) -> Self {
        Servo { pin }
    }

    pub fn enable(&mut self) {
        self.pin.enable();
    }

    pub fn disable(&mut self) {
        self.pin.disable();
    }

    pub fn set_angle(&mut self, angle: u8) {
        self.pin.set_angle(angle);
    }
}