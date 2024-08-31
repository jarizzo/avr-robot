use super::{steppers::Stepper, servos::Servo, motors::Motor};


pub enum MotorPort {
    TwoMotors,
    SingleStepper,
    SingleMotorFirst,
    SingleMotorSecond,
    Empty,
}

pub struct ShieldLayout {
    pub port1: MotorPort,
    pub port2: MotorPort,
}

pub struct Steppers {
    pub(crate) s1: Option<Stepper>,
    pub(crate) s2: Option<Stepper>,
}

impl Steppers {
    pub(crate) fn len(&self) -> usize { 2 }
}

pub struct Servos {
    pub(crate) s1: Option<Servo>,
    pub(crate) s2: Option<Servo>,
}

impl Servos {
    pub fn len(&self) -> usize { 2 }
}

pub struct Motors {
    pub m1: Option<Motor>,
    pub m2: Option<Motor>,
    pub m3: Option<Motor>,
    pub m4: Option<Motor>,
}

impl Motors {
    pub fn len(&self) -> usize { 4 }
}