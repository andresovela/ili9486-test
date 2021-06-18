#![no_main]
#![no_std]

use display_interface_parallel_gpio::{
    PGPIO8BitInterface, Generic8BitBus
};

use embedded_hal as _;
use embedded_hal::blocking::delay::DelayMs;
use hal::{gpio::Level, Delay};
use ili9486::{Command, Orientation, ILI9486};
use my_app as _;
use nrf52832_hal as hal;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Program started");

    let cp = hal::pac::CorePeripherals::take().unwrap();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    let mut delay = Delay::new(cp.SYST);

    let ili9486_rsx = port0.p0_30.into_push_pull_output(Level::High);
    let ili9486_csx = port0.p0_29.into_push_pull_output(Level::High);

    let ili9486_wrx = port0.p0_04.into_push_pull_output(Level::High);
    let ili9486_dcx = port0.p0_28.into_push_pull_output(Level::High);
    let _ili9486_rdx = port0.p0_03.into_push_pull_output(Level::High);

    let ili9486_dbus = Generic8BitBus::new(
        (port0.p0_19.into_push_pull_output(Level::Low).degrade(),
        port0.p0_20.into_push_pull_output(Level::Low).degrade(),
        port0.p0_13.into_push_pull_output(Level::Low).degrade(),
        port0.p0_14.into_push_pull_output(Level::Low).degrade(),
        port0.p0_15.into_push_pull_output(Level::Low).degrade(),
        port0.p0_16.into_push_pull_output(Level::Low).degrade(),
        port0.p0_17.into_push_pull_output(Level::Low).degrade(),
        port0.p0_18.into_push_pull_output(Level::Low).degrade())
    ).unwrap();

    let ili9486_interface = PGPIO8BitInterface::new(ili9486_dbus, ili9486_dcx, ili9486_wrx);
    let mut display = ILI9486::new(ili9486_interface, ili9486_csx, ili9486_rsx).unwrap();
    defmt::info!("Display created");

    defmt::info!("Asserting reset");
    display.assert_reset().unwrap();

    // Wait minimum 10 us to reset
    delay.delay_ms(15u32);

    defmt::info!("Deasserting reset");
    display.deassert_reset().unwrap();

    // Wait 120 ms before sending any commands
    delay.delay_ms(15u32);

    defmt::info!("Enabling display");
    display.enable().unwrap();

    defmt::info!("Setting orientation");
    display.set_orientation(Orientation::PortraitUpsideDown).unwrap();

    defmt::info!("Setting pixel format");
    display.set_pixel_format().unwrap();

    defmt::info!("Sending sleep out");
    display.send_command(Command::SleepOut).unwrap();

    // Wait 5 ms after sleep out before sending any other commands
    delay.delay_ms(120u32);

    defmt::info!("Sending display on");
    display.send_command(Command::DisplayOn).unwrap();

    loop {
        defmt::info!("Setting fill 1");
        display.fill(0xF800).unwrap();
        delay.delay_ms(1000u32);

        defmt::info!("Setting fill 2");
        display.fill(0x07E0).unwrap();
        delay.delay_ms(1000u32);

        defmt::info!("Setting fill 3");
        display.fill(0x001F).unwrap();
        delay.delay_ms(1000u32);
    }
}
