#![allow(dead_code)]

// PIT ports
const PIT_CMD: u16 = 0x43;
const PIT_CANAL_0: u16 = 0x40;
const PIT_DIV_REPEAT: u8 = 0x36;

pub const MIN_FREQ: u32 = 19;
pub const MAX_FREQ: u32 = 1193180;

static mut TIMER: Timer = Timer { freq: 0, ticks: 0 };

pub fn timer_init(freq_hz: u32) {
    unsafe { TIMER.init(freq_hz) }
}

pub fn timer_handler() {
    unsafe { TIMER.ticks+=1 }
}

pub fn get_freq() -> u32 {
    unsafe { return TIMER.freq }
}

pub fn get_ticks() -> u32{
    unsafe { return TIMER.ticks }
}

pub fn sleep(ms: u32) {
    let duration = get_ticks() + (ms * unsafe { TIMER.freq } / 1000);
    loop {
        if get_ticks() >= duration {
            break;
        }
    }
}

struct Timer {
    freq: u32,
    ticks: u32
}
impl Timer {
    pub fn init(&mut self, freq_hz: u32) {
        use core::u32;
        match freq_hz {
            0...MIN_FREQ => self.freq = MIN_FREQ,
            MAX_FREQ...u32::MAX => self.freq =  MAX_FREQ,
            _ => self.freq = freq_hz
        }
        
        #[cfg(not(test))]
        unsafe {
            use pio::outb;
            let div = MAX_FREQ / self.freq;
            // divisor selection and repetition mode
            outb(PIT_CMD, PIT_DIV_REPEAT);
            // divisor LSB on canal 0
            outb(PIT_CANAL_0, (div & 0xFF) as u8);
            // divisor MSB on canal 0
            outb(PIT_CANAL_0, (div >> 8) as u8);
        }
    }

    pub fn get_freq(&mut self) -> u32 {
        return self.freq;
    }

    pub fn get_ticks(&mut self) -> u32{
        return self.ticks;
    }
}