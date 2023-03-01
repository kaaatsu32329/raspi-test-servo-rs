/*
 * This source code is based on below.
 * https://github.com/golemparts/rppal/blob/master/examples/gpio_servo_softpwm.rs
 */

mod opt;

use crate::opt::Opt;
use rppal::gpio::Gpio;
use std::io::Write;
use structopt::StructOpt;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_PIN_SERVO: u8 = 23;

// Servo configuration. Change these values based on your servo's verified safe
// minimum and maximum values.
//
// Period: 20 ms (50 Hz). Pulse width: min. 1200 µs, neutral 1500 µs, max. 1800 µs.
const PERIOD_MS: u64 = 20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let neutral_microseconds = (0.5 * (opt.min_microseconds + opt.max_microseconds) as f64) as u64;

    let mut servo_pin = Gpio::new()?.get(GPIO_PIN_SERVO)?.into_output();

    servo_pin.set_pwm(
        std::time::Duration::from_millis(PERIOD_MS),
        std::time::Duration::from_millis(opt.max_microseconds),
    )?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    servo_pin.set_pwm(
        std::time::Duration::from_millis(PERIOD_MS),
        std::time::Duration::from_micros(opt.min_microseconds),
    )?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    for pulse in (opt.min_microseconds..=opt.max_microseconds).step_by(25) {
        println!("Set PWM microsecound: {}", pulse);
        servo_pin.set_pwm(
            std::time::Duration::from_millis(PERIOD_MS),
            std::time::Duration::from_micros(pulse),
        )?;
        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    // Set neutral position
    servo_pin.set_pwm(
        std::time::Duration::from_millis(PERIOD_MS),
        std::time::Duration::from_micros(neutral_microseconds),
    )?;
    std::thread::sleep(std::time::Duration::from_millis(500));

    loop {
        let mut buffer = String::from("0");

        print!("Input control value: ");
        std::io::stdout().flush().unwrap();

        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                match buffer.trim_end().to_owned().parse::<i64>() {
                    Ok(input) => {
                        if input < 0 {
                            println!("\x1b[38;5;1m[MESSAGE] Received negative num. Finish program.\x1b[m");
                            break;
                        } else if input >= 1200 && input <= 3000 {
                            servo_pin.set_pwm(
                                std::time::Duration::from_millis(PERIOD_MS),
                                std::time::Duration::from_micros(input as u64),
                            )?;
                        }
                    }
                    Err(e) => {
                        println!("\x1b[38;5;3m[WARNING] {e}.\x1b[m");
                    }
                }
            }
            Err(_) => {
                eprintln!("Standard input error.");
                // TODO: Error handling.
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok(())
}
