
pub mod motors;
pub mod steppers;
pub mod servos;
mod digital_output;
pub mod layout;

use arduino_hal::{
    hal::port,
    pac::{TC0, TC1, TC2},
    port::{mode, Pin},
    simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm, Timer2Pwm}
};

use crate::motor_shield::layout::ShieldLayout;

use self::{layout::{MotorPort, Steppers, Motors, Servos}, motors::{MotorPin, Motor}, steppers::{StepperPin, Stepper}, servos::{ServoPin, Servo}, digital_output::DigitalOutput};

pub struct MotorShield {
    steppers: Steppers,
    motors: Motors,
    servos: Servos,
}

impl MotorShield {
    pub fn new(
        layout: ShieldLayout,
        tc0: TC0,
        tc1: TC1,
        tc2: TC2,
        pin_d3: Pin<mode::Input<mode::Floating>, port::PD3>,
        pin_d4: Pin<mode::Input<mode::Floating>, port::PD4>,
        pin_d5: Pin<mode::Input<mode::Floating>, port::PD5>,
        pin_d6: Pin<mode::Input<mode::Floating>, port::PD6>,
        pin_d7: Pin<mode::Input<mode::Floating>, port::PD7>,
        pin_d8: Pin<mode::Input<mode::Floating>, port::PB0>,
        pin_d9: Pin<mode::Input<mode::Floating>, port::PB1>,
        pin_d10: Pin<mode::Input<mode::Floating>, port::PB2>,
        pin_d11: Pin<mode::Input<mode::Floating>, port::PB3>,
        pin_d12: Pin<mode::Input<mode::Floating>, port::PB4>,
    ) -> Self {
        let mut pwm_timer0 = Timer0Pwm::new(tc0, Prescaler::Prescale64);
        let mut pwm_timer1 = Timer1Pwm::new(tc1, Prescaler::Prescale256);
        let mut pwm_timer2 = Timer2Pwm::new(tc2, Prescaler::Prescale64);

        let digital_output = &mut DigitalOutput::new(pin_d4, pin_d7, pin_d8, pin_d12);

        let (s1, m1, m2) = match layout.port1 {
            MotorPort::SingleStepper => (
                Some(Stepper::new(
                    StepperPin::Stepper1((
                        pin_d11.into_output().into_pwm(&mut pwm_timer2),
                        pin_d3.into_output().into_pwm(&mut pwm_timer2),
                    )),
                    48,
                    digital_output,
                )),
                None,
                None,
            ),
            MotorPort::SingleMotorFirst | MotorPort::SingleMotorSecond | MotorPort::TwoMotors => {
                let motor1 = if matches!(layout.port1, MotorPort::SingleMotorFirst | MotorPort::TwoMotors) {
                    Some(Motor::new(MotorPin::Motor1(pin_d11.into_output().into_pwm(&mut pwm_timer2)), digital_output))
                } else {
                    None
                };

                let motor2 = if matches!(layout.port1, MotorPort::SingleMotorSecond | MotorPort::TwoMotors) {
                    Some(Motor::new(MotorPin::Motor2(pin_d3.into_output().into_pwm(&mut pwm_timer2)), digital_output))
                } else {
                    None
                };

                (None, motor1, motor2)
            },
            _ => (None, None, None),
        };

        let (s2, m3, m4) = match layout.port2 {
            MotorPort::SingleStepper => (
                Some(Stepper::new(
                    StepperPin::Stepper2((
                        pin_d6.into_output().into_pwm(&mut pwm_timer0),
                        pin_d5.into_output().into_pwm(&mut pwm_timer0),
                    )),
                    48,
                    digital_output,
                )),
                None,
                None,
            ),
            MotorPort::SingleMotorFirst | MotorPort::SingleMotorSecond | MotorPort::TwoMotors => {
                let motor3 = if matches!(layout.port2, MotorPort::SingleMotorFirst | MotorPort::TwoMotors) {
                    Some(Motor::new(MotorPin::Motor3(pin_d6.into_output().into_pwm(&mut pwm_timer0)), digital_output))
                } else {
                    None
                };

                let motor4 = if matches!(layout.port2, MotorPort::SingleMotorSecond | MotorPort::TwoMotors) {
                    Some(Motor::new(MotorPin::Motor4(pin_d5.into_output().into_pwm(&mut pwm_timer0)), digital_output))
                } else {
                    None
                };

                (None, motor3, motor4)
            },
            _ => (None, None, None),
        };

        Self {
            steppers: Steppers {
                s1,
                s2
            },
            motors: Motors {
                m1,
                m2,
                m3,
                m4,
            },
            servos: Servos {
                s1: Some(Servo::new(ServoPin::Servo1(pin_d10.into_output().into_pwm(&mut pwm_timer1)))),
                s2: Some(Servo::new(ServoPin::Servo2( pin_d9.into_output().into_pwm(&mut pwm_timer1))))
            },
        }
    }

    pub fn steppers_count(&mut self) -> usize {
        self.steppers.len()
    }

    pub fn stepper(&mut self, stepper_id: usize) -> Option<&mut Stepper> {
        match stepper_id {
            1 => self.steppers.s1.as_mut(),
            2 => self.steppers.s2.as_mut(),
            _ => panic!("invalid stepper index")
        }
    }

    pub fn motors_count(&mut self) -> usize {
        self.motors.len()
    }

    pub fn motor(&mut self, motor_id: usize) -> Option<&mut Motor> {
        match motor_id {
            1 => self.motors.m1.as_mut(),
            2 => self.motors.m2.as_mut(),
            3 => self.motors.m3.as_mut(),
            4 => self.motors.m4.as_mut(),
            _ => panic!("invalid motor index")
        }
    }

    pub fn servos_count(&mut self) -> usize {
        self.servos.len()
    }

    pub fn servo(&mut self, servo_id: usize) -> Option<&mut Servo> {
        match servo_id {
            1 => self.servos.s1.as_mut(),
            2 => self.servos.s2.as_mut(),
            _ => panic!("invalid servo index")
        }
    }

    pub fn enable_motors(&mut self, motor_ids: &[usize]) {
        for &id in motor_ids {
            if let Some(motor) = self.motor(id) {
                motor.enable();
            }
        }
    }

    pub fn set_speeds(&mut self, motor_speeds: &[(usize, u8)]) {
        for &(id, speed) in motor_speeds {
            if let Some(motor) = self.motor(id) {
                motor.speed(speed);
            }
        }
    }
}