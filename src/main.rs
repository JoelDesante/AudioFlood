use sdl2::audio::AudioSpecDesired;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use ringbuf::HeapRb;

const SAMPLE_FREQUENCY: i32 = 48000;

fn generate_next_sample(sample_number: i64, f: f64) -> i16 {
    let x = sample_number as f64 / SAMPLE_FREQUENCY as f64;
    let frequency = f;
    let volume = 1000 as f64;

    let sample = (frequency * x).sin() * volume;
    sample as i16
}

pub fn main() {
    
    let mut sample_number: i64 = 0;
    let mut fre: f64 = 0.0;
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    //let audio_buffer = HeapRb::<i16>::new(SAMPLE_FREQUENCY);
    //let (mut audio_buffer_producer, mut audio_buffer_consumer) = rb.split();

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),
        samples: None
    };

    let device = audio_subsystem.open_queue::<i16, _>(None, &desired_spec).unwrap();
    device.resume();

    while true {
        let sample = generate_next_sample(sample_number, fre);
        let data = [sample];
        device.queue(&data);
        sample_number = sample_number + 1;
        fre = fre + 0.001;
    }

}