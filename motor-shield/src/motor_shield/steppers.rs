use arduino_hal::{
    hal::port,
    port::{mode, Pin},
    simple_pwm::{Timer0Pwm, Timer2Pwm}, delay_ms
};

use super::digital_output::DigitalOutput;

const MICROSTEPS: u8 = 16;
const MICROSTEP_CURVE: [u8; 17] = [0, 25, 50, 74, 98, 120, 141, 162, 180, 197, 212, 225, 236, 244, 250, 253, 255];

#[derive(PartialEq, Clone, Copy)]
pub enum StepperStyle {
    SINGLE,
    DOUBLE,
    INTERLEAVE,
    MICROSTEP
}

#[derive(PartialEq, Clone, Copy)]
pub enum StepperDirection {
    FORWARD,
    BACKWARD,
}

pub enum StepperPin {
    Stepper1(
        (
            Pin<mode::PwmOutput<Timer2Pwm>, port::PB3>,
            Pin<mode::PwmOutput<Timer2Pwm>, port::PD3>,
        ),
    ),
    Stepper2(
        (
            Pin<mode::PwmOutput<Timer0Pwm>, port::PD6>,
            Pin<mode::PwmOutput<Timer0Pwm>, port::PD5>,
        ),
    ),
}

impl StepperPin {
    fn get_abcd(&self) -> (u8, u8, u8, u8) {
        match self {
            // STEPPER1_A 2 0x0000_0200
            // STEPPER1_C 1 0x0000_0010
            // STEPPER1_B 3 0x0000_1000
            // STEPPER1_D 4 0x0001_0000
            Self::Stepper1(_) => (
                1 << 2,
                1 << 1,
                1 << 3,
                1 << 4),
            // STEPPER2_A 5 0x0010_0000
            // STEPPER2_C 0 0x0000_0001
            // STEPPER2_B 7 0x1000_0000
            // STEPPER2_D 6 0x0100_0000
            Self::Stepper2(_) => (
                1 << 5,
                1 << 0,
                1 << 7,
                1 << 6),
        }
    }

    pub fn set_dutys(&mut self, duty1: u8, duty2: u8) {
        match self {
            Self::Stepper1((pin1, pin2)) => {
                pin1.set_duty(duty1);
                pin2.set_duty(duty2);
            }
            Self::Stepper2((pin1, pin2)) => {
                pin1.set_duty(duty1);
                pin2.set_duty(duty2);
            }
        }
    }
    pub fn enable(&mut self) {
        match self {
            Self::Stepper1((pin1, pin2)) => {
                pin1.enable();
                pin2.enable();
            }
            Self::Stepper2((pin1, pin2)) => {
                pin1.enable();
                pin2.enable();
            }
        }
    }

    pub fn disable(&mut self) {
        match self {
            Self::Stepper1((pin1, pin2)) => {
                pin1.disable();
                pin2.disable();
            }
            Self::Stepper2((pin1, pin2)) => {
                pin1.disable();
                pin2.disable();
            }
        }
    }
}

pub struct Stepper {
    pin: StepperPin,
    output: *mut DigitalOutput,
    // # steps per revolution
    revsteps: u16,
    usperstep: u32,
    steppingcounter: u32,
    currentstep: u8
}

impl Stepper {
    pub fn new(pin: StepperPin, steps: u16, output: *mut DigitalOutput) -> Self {
        let mut me = Self {
            pin,
            output,
            revsteps: steps,
            usperstep: 0,
            steppingcounter: 0,
            currentstep: 0
        };

        me.release();
        me.pin.set_dutys(255, 255);

        me
    }

    pub fn set_speed(&mut self, rpm: u16) {
        self.usperstep = (60_000_000_u32 / (self.revsteps * rpm) as u32).into();
        self.steppingcounter = 0;
    }

    pub fn release(&self) {
        let (a, b, c, d)= self.pin.get_abcd();
        let output = unsafe { self.output.as_mut().unwrap()};

        // all motor pins to 0
        output.and(!a & !b & !c & !d);
        output.transmit();
    }

    pub fn step(&mut self, mut steps: u32, dir: StepperDirection, style: StepperStyle) {
        let mut uspers: u32 = self.usperstep;

        match style {
            StepperStyle::INTERLEAVE => {
                uspers /= 2;
            }
            StepperStyle::MICROSTEP => {
                uspers /= MICROSTEPS as u32;
                steps *= MICROSTEPS as u32;
            }
            _ => { }
        }

        while steps > 0 {
            _ = self.onestep(dir, style);
            delay_ms((uspers / 1000) as u16); // in ms
            self.steppingcounter += uspers % 1000;
            if self.steppingcounter >= 1000 {
                delay_ms(1);
                self.steppingcounter -= 1000;
            }
            steps -= 1;
        }

        if let StepperStyle::MICROSTEP = style {
            let mut ret = self.onestep(dir, style);
            while ret != 0 && ret != MICROSTEPS {
                ret = self.onestep(dir, style);
                delay_ms((uspers / 1000) as u16); // in ms
                self.steppingcounter += uspers % 1000;
                if self.steppingcounter >= 1000 {
                    delay_ms(1);
                    self.steppingcounter -= 1000;
                }
            }
        }

    }

    pub fn onestep(&mut self, dir: StepperDirection, style: StepperStyle) -> u8 {
        let (a, b, c, d)= self.pin.get_abcd();
        let mut ocra: u8 = u8::max_value();
        let mut ocrb: u8 = u8::max_value();

        match style {
            StepperStyle::SINGLE => {
                if (self.currentstep / (MICROSTEPS / 2)) % 2 == 0 {
                    // Go to the next even step
                    self.currentstep = match dir {
                        StepperDirection::FORWARD => self.currentstep.wrapping_add(MICROSTEPS),
                        StepperDirection::BACKWARD => self.currentstep.wrapping_sub(MICROSTEPS),
                    };
                } else {
                    // We're at an odd step, weird
                    self.currentstep = match dir {
                        StepperDirection::FORWARD => self.currentstep.wrapping_add(MICROSTEPS / 2),
                        StepperDirection::BACKWARD => self.currentstep.wrapping_sub(MICROSTEPS / 2),
                    };
                }
            },
            StepperStyle::DOUBLE => {
                if ((self.currentstep / (MICROSTEPS / 2)) % 2) == 0 {
                    // We're at an odd step, weird
                    self.currentstep = match dir {
                        StepperDirection::FORWARD => self.currentstep.wrapping_add(MICROSTEPS / 2),
                        StepperDirection::BACKWARD => self.currentstep.wrapping_sub(MICROSTEPS / 2),
                    };
                } else {
                    // Go to the next even step
                    self.currentstep = match dir {
                        StepperDirection::FORWARD => self.currentstep.wrapping_add(MICROSTEPS),
                        StepperDirection::BACKWARD => self.currentstep.wrapping_sub(MICROSTEPS),
                    };
                }
            },
            StepperStyle::INTERLEAVE => {
                self.currentstep = match dir {
                    StepperDirection::FORWARD => self.currentstep.wrapping_add(MICROSTEPS / 2),
                    StepperDirection::BACKWARD => self.currentstep.wrapping_sub(MICROSTEPS / 2),
                }
            },
            StepperStyle::MICROSTEP => {
                self.currentstep = match dir {
                    StepperDirection::FORWARD => self.currentstep.wrapping_add(1),
                    StepperDirection::BACKWARD => self.currentstep.wrapping_sub(1),
                };

                self.currentstep = self.currentstep.wrapping_add(MICROSTEPS * 4) % (MICROSTEPS * 4);

                ocra = 0;
                ocrb = 0;
                if self.currentstep < MICROSTEPS {
                    ocra = MICROSTEP_CURVE[(MICROSTEPS - self.currentstep) as usize];
                    ocrb = MICROSTEP_CURVE[self.currentstep as usize];
                } else if self.currentstep < MICROSTEPS * 2 {
                    ocra = MICROSTEP_CURVE[(self.currentstep - MICROSTEPS) as usize];
                    ocrb = MICROSTEP_CURVE[(MICROSTEPS * 2 - self.currentstep) as usize];
                } else if self.currentstep < MICROSTEPS * 3 {
                    ocra = MICROSTEP_CURVE[(MICROSTEPS * 3 - self.currentstep) as usize];
                    ocrb = MICROSTEP_CURVE[(self.currentstep - MICROSTEPS * 2) as usize];
                } else if self.currentstep < MICROSTEPS * 4 {
                    ocra = MICROSTEP_CURVE[(self.currentstep - MICROSTEPS * 3) as usize];
                    ocrb = MICROSTEP_CURVE[(MICROSTEPS * 4 - self.currentstep) as usize];
                }
            }
        }

        self.currentstep = self.currentstep % (MICROSTEPS * 4);

        self.pin.set_dutys(ocra, ocrb);

        let output = unsafe { self.output.as_mut().unwrap()};
        // release all
        output.and(!a & !b & !c & !d); // all motor pins to 0

        if style == StepperStyle::MICROSTEP {
            match (self.currentstep / MICROSTEPS) % 4 {
                0 => output.or(a | b),
                1 => output.or(b | c),
                2 => output.or(c | d),
                3 => output.or(d | a),
                _ => { }
            }
        } else {
            match self.currentstep / (MICROSTEPS / 2) {
                0 => output.or(a),     // energize coil 1 only
                1 => output.or(a | b), // energize coil 1 + 2
                2 => output.or(b),     // energize coil 2 only
                3 => output.or(b | c), // energize coil 2 + 3
                4 => output.or(c),     // energize coil 3 only
                5 => output.or(c | d), // energize coil 3 + 4
                6 => output.or(d),     // energize coil 4 only
                7 => output.or(d | a), // energize coil 1 + 4
                _ => output.and(!a & !b & !c & !d) // all motor pins to 0
            }
        }

        output.transmit();

        self.currentstep
    }

    pub fn enable(&mut self) {
        self.pin.enable();
    }

    pub fn disable(&mut self) {
        self.pin.disable();
    }
}