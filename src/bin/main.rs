#![no_std]
#![no_main]

use core::{cell::{Cell, RefCell}, panic::PanicInfo};
use chrono::Timelike;

use critical_section::Mutex;
use embedded_graphics::{image::Image, prelude::Point, Drawable};
use esp_hal::{
    clock::CpuClock, delay::MicrosDurationU64, gpio::{Event, Input, Io, Pull}, handler, i2c::master::I2c, interrupt::InterruptConfigurable, ram, rtc_cntl::Rtc, timer::{systimer::{Alarm, SystemTimer}, Timer}, Blocking, main
};

use esp_println::println;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use espwatch::clockface;
use tinybmp::Bmp;

type WatchDisplay<'a> = Ssd1306<I2CInterface<I2c<'a, Blocking>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

static DISPLAY: Mutex<RefCell<Option<WatchDisplay>>> = Mutex::new(RefCell::new(None));
static RENDER_TIMER: Mutex<RefCell<Option<Alarm>>> = Mutex::new(RefCell::new(None));
static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<Rtc>>> = Mutex::new(RefCell::new(None));

static PAGE: Mutex<Cell<Page>> = Mutex::new(Cell::new(Page::Clock));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Clock,
    Settings
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    println!("PANIC!");

    loop {}
}

fn render(page: Page) {
    match page {
        Page::Clock => {
            render_clock()
        },
        Page::Settings => {
            render_settings();
        }
    }
}

fn render_settings() {
    critical_section::with(|cs| {
        let mut display_ref = DISPLAY.borrow_ref_mut(cs);
        let display  = display_ref.as_mut().unwrap();

        display.clear_buffer();

        display.flush().unwrap();
    });
}

fn render_clock() {
    critical_section::with(|cs| {
        let mut rtc_ref = RTC.borrow_ref_mut(cs);
        let rtc  = rtc_ref.as_mut().unwrap();

        let mut display_ref = DISPLAY.borrow_ref_mut(cs);
        let display  = display_ref.as_mut().unwrap();
        
        display.clear_buffer();

        let time = rtc.current_time().and_utc().time();

        clockface::render_clockface(display, time.hour(), time.minute(), time.second());
        
        display.flush().unwrap();
    });
}

#[handler]
#[ram]
fn handler() {
    if critical_section::with(|cs| {
        let mut button_ref = BUTTON
            .borrow_ref_mut(cs);
        let button = button_ref
            .as_mut()
            .unwrap();
        button.is_interrupt_set() && button.is_low()
    }) {
        critical_section::with(|cs| {
            let page = PAGE.borrow(cs);

            match page.get() {
                Page::Clock => {
                    page.set(Page::Settings);
                },
                Page::Settings => {
                    page.set(Page::Clock);
                }
            }

            let page = page.get();

            render(page);
        })
    }

    if critical_section::with(|cs| {
        RENDER_TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .is_interrupt_set()
    }) {
        critical_section::with(|cs| {
            let page = PAGE.borrow(cs).get();
            
            if page == Page::Clock {
                render(page)
            }
        })
    }

    critical_section::with(|cs| {
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();

        RENDER_TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt()
    })
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    let rtc = Rtc::new(peripherals.LPWR);
    critical_section::with(|cs| {
        RTC.borrow_ref_mut(cs).replace(rtc);
    });

    let i2c = I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default(),
    )
    .unwrap()
    .with_sda(peripherals.GPIO5)
    .with_scl(peripherals.GPIO6);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();
    display.init().unwrap();

    display.set_brightness(Brightness::DIMMEST).unwrap();

    critical_section::with(|cs| {
        let page = PAGE.borrow(cs).get();

        DISPLAY.borrow_ref_mut(cs).replace(display);

        render(page)
    });

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(handler);

    let mut button = Input::new(peripherals.GPIO1, Pull::Up);
    critical_section::with(|cs| {
        button.listen(Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button);
    });

    let syst = SystemTimer::new(peripherals.SYSTIMER);
    let mut alarm = syst.alarm0;
    alarm.set_interrupt_handler(handler);
    alarm.enable_interrupt(true);
    alarm.enable_auto_reload(true);
    alarm.load_value(MicrosDurationU64::Hz(1)).unwrap();
    alarm.start();
    critical_section::with(|cs| {
        RENDER_TIMER.borrow_ref_mut(cs).replace(alarm);
    });

    loop {}
}