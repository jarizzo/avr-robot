use arduino_hal::{
    hal::port,
    port::{mode, Pin}
};

pub struct DigitalOutput {
    enable: Pin<mode::Output, port::PD7>,
    latch: Pin<mode::Output, port::PB4>,
    data: Pin<mode::Output, port::PB0>,
    clock: Pin<mode::Output, port::PD4>,
    state: u8
}

impl DigitalOutput {
    pub fn new(
        pin_d4: Pin<mode::Input<mode::Floating>, port::PD4>,
        pin_d7: Pin<mode::Input<mode::Floating>, port::PD7>,
        pin_d8: Pin<mode::Input<mode::Floating>, port::PB0>,
        pin_d12: Pin<mode::Input<mode::Floating>, port::PB4>,
    ) -> Self {
        let mut me = Self {
            enable: pin_d7.into_output_high(),
            latch: pin_d12.into_output_high(),
            data: pin_d8.into_output_high(),
            clock: pin_d4.into_output_high(),
            state: 0,
        };

        me.transmit();
        me.enable.set_low();

        me
    }

    pub fn or(&mut self, bits: u8) {
        self.state |= bits;
    }

    pub fn and(&mut self, bits: u8) {
        self.state &= bits;
    }
    pub fn and_not(&mut self, bits: u8) {
        self.state &= !bits;
    }

    pub fn transmit(&mut self) {
        self.latch.set_low();

        for i in 0..8 {
            self.clock.set_low();
            self.data.set_state((self.state & (1 << (7 - i))) != 0);
            self.clock.set_high();
        }

        self.latch.set_high();
    }

}
// Extension for the `Pin` to simplify the setting of the pin state.
trait PinExt {
    fn set_state(&mut self, high: bool);
}

impl PinExt for Pin<mode::Output, port::PD4> {
    fn set_state(&mut self, high: bool) {
        if high {
            self.set_high();
        } else {
            self.set_low();
        }
    }
}

impl PinExt for Pin<mode::Output, port::PB0> {
    fn set_state(&mut self, high: bool) {
        if high {
            self.set_high();
        } else {
            self.set_low();
        }
    }
}