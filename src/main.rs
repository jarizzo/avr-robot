#![no_std]
#![no_main]

use arduino_hal::Adc;
use arduino_hal::hal::wdt;
use panic_halt as _;
use motor_shield::{init_ams};
use motor_shield::ShieldLayout;
use motor_shield::MotorShield;
use motor_shield::MotorPort;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);
    let a3 = pins.a3.into_analog_input(&mut adc);
    let a4 = pins.a4.into_analog_input(&mut adc);
    let a5 = pins.a5.into_analog_input(&mut adc);

    let mut motor_shield = init_ams!(
        ShieldLayout {
            port1: MotorPort::TwoMotors,
            port2: MotorPort::Empty,
        },
        dp,
        pins
    );

    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms4000).unwrap();

    motor_shield.enable_motors(&[0, 1]);

    loop {
        let infra: [u16; 6] = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
            a2.analog_read(&mut adc),
            a3.analog_read(&mut adc),
            a4.analog_read(&mut adc),
            a5.analog_read(&mut adc),
        ];

        let left = infra[0] + infra[1];
        let center = infra[2] + infra[3];
        let right = infra[4] + infra[5];

        if left > center && left > right {
            motor_shield.set_speeds(&[(255, 0), (0, 1)]);
        } else if right > center {
            motor_shield.set_speeds(&[(0, 255), (0, 1)]);
        } else {
            motor_shield.set_speeds(&[(255, 255), (0, 1)]);
        }
        watchdog.feed();
    }
}
