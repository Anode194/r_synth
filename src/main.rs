use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use rand::prelude::*;
use sqlite::State;
use std::env;
use std::f64::consts::PI;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
    match args[1].as_ref() {
        "-p" => {
            nannou::app(model).run();
        }
        "-h" | "--help" | _ => {
            println!("{}", &args[0]);
            println!("R_synth is a keyboard focused audio creation tool");
            println!("-h --help \t\t'prints this help message'");
        }
    }
    //nannou::app(model).run();
    }
}
struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    phase: f64,
    hz: f64,
    volume: f32,
}

fn model(app: &App) -> Model {
    // create notes structs
    let octaves = Notes::new();
    // Create a window to receive key pressed events.
    app.new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    // Initialise the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialise the state that we want to live on the audio thread.
    let model = Audio {
        phase: 0.0,
        hz: 440.0,
        volume: 0.0,
    };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();
    Model { stream }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play a simple sine wave at the audio's current frequency in `hz`.

fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * audio.volume;
        }
    }
}

struct Notes {
    pub c: f64,
    pub c_sharp: f64,
    pub d: f64,
    pub d_sharp: f64,
    pub e: f64,
    pub f: f64,
    pub f_sharp: f64,
    pub g: f64,
    pub g_sharp: f64,
    pub a: f64,
    pub a_sharp: f64,
    pub b: f64,
    pub c1: f64,
    pub c_sharp1: f64,
    pub d1: f64,
    pub d_sharp1: f64,
    pub e1: f64,
    pub f1: f64,
    pub f_sharp1: f64,
    pub g1: f64,
    pub g_sharp1: f64,
    pub a1: f64,
    pub a_sharp1: f64,
    pub b1: f64,
    pub c2: f64,
}
impl Notes {
    //this is C2 - C4 on a piano.
    fn new() -> Notes {
        let octave_2_3 = Notes {
            c: 65.41,
            c_sharp: 69.30,
            d: 73.42,
            d_sharp: 77.78,
            e: 77.78,
            f: 87.31,
            f_sharp: 92.50,
            g: 98.0,
            g_sharp: 103.83,
            a: 110.0,
            a_sharp: 116.54,
            b: 123.47,
            c1: 130.81,
            c_sharp1: 130.81,
            d1: 146.83,
            d_sharp1: 155.56,
            e1: 164.81,
            f1: 174.61,
            f_sharp1: 185.0,
            g1: 196.0,
            g_sharp1: 207.65,
            a1: 220.0,
            a_sharp1: 233.08,
            b1: 246.94,
            c2: 261.94,
        };
        octave_2_3
    }
}
impl Notes {
    fn new_from_db_random() -> Notes {
        let mut rng = rand::thread_rng();
        let oct = rng.gen_range(1, 8);
        let connection = sqlite::open("r_synth").unwrap();
        let mut statement = connection
            .prepare("Select * from octave where rowid = ?")
            .unwrap();
        let mut chosen_octave = Notes::new();
        statement.bind(1, oct).unwrap();
        while let State::Row = statement.next().unwrap() {
            chosen_octave.c = statement.read::<f64>(0).unwrap();
            chosen_octave.c_sharp = statement.read::<f64>(1).unwrap();
            chosen_octave.d = statement.read::<f64>(2).unwrap();
            chosen_octave.d_sharp = statement.read::<f64>(3).unwrap();
            chosen_octave.e = statement.read::<f64>(4).unwrap();
            chosen_octave.f = statement.read::<f64>(5).unwrap();
            chosen_octave.f_sharp = statement.read::<f64>(6).unwrap();
            chosen_octave.g = statement.read::<f64>(7).unwrap();
            chosen_octave.g_sharp = statement.read::<f64>(8).unwrap();
            chosen_octave.a = statement.read::<f64>(9).unwrap();
            chosen_octave.a_sharp = statement.read::<f64>(10).unwrap();
            chosen_octave.b = statement.read::<f64>(11).unwrap();
        }
        let mut statement = connection
            .prepare("Select * from octave where rowid = ?")
            .unwrap();
        statement.bind(1, oct + 1).unwrap();
        while let State::Row = statement.next().unwrap() {
            chosen_octave.c1 = statement.read::<f64>(0).unwrap();
            chosen_octave.c_sharp1 = statement.read::<f64>(1).unwrap();
           chosen_octave.d1 = statement.read::<f64>(2).unwrap();
            chosen_octave.d_sharp1 = statement.read::<f64>(3).unwrap();
            chosen_octave.e1 = statement.read::<f64>(4).unwrap();
            chosen_octave.f1 = statement.read::<f64>(5).unwrap();
            chosen_octave.f_sharp1 = statement.read::<f64>(6).unwrap();
            chosen_octave.g1 = statement.read::<f64>(7).unwrap();
            chosen_octave.g_sharp1 = statement.read::<f64>(8).unwrap();
            chosen_octave.a1 = statement.read::<f64>(9).unwrap();
            chosen_octave.a_sharp1 = statement.read::<f64>(10).unwrap();
            chosen_octave.b1 = statement.read::<f64>(11).unwrap();
        }
        chosen_octave
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    let notes = Notes::new();
    match key {
        // Pause or unpause the audio when Space is pressed.
        Key::Space => {
            if model.stream.is_playing() {
                model.stream.pause().unwrap();
            } else {
                model.stream.play().unwrap();
            }
        }
        // Raise the frequency when the up key is pressed.
        Key::Up => {
            model
                .stream
                .send(|audio| {
                    audio.volume += 10.0;
                })
                .unwrap();
        }
        // Lower the frequency when the down key is pressed.
        Key::Down => {
            model
                .stream
                .send(|audio| {
                    audio.volume -= 10.0;
                })
                .unwrap();
        }
        Key::Z => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.c;
                })
                .unwrap();
        }
        Key::X => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.d;
                })
                .unwrap();
        }
        Key::C => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.e;
                })
                .unwrap();
        }
        Key::V => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.f;
                })
                .unwrap();
        }
        Key::B => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.g;
                })
                .unwrap();
        }
        Key::S => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.c_sharp;
                })
                .unwrap();
        }
        Key::D => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.d_sharp;
                })
                .unwrap();
        }
        Key::N => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.a;
                })
                .unwrap();
        }
        Key::M => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.b;
                })
                .unwrap();
        }
        Key::G => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.f_sharp;
                })
                .unwrap();
        }
        Key::H => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.g_sharp;
                })
                .unwrap();
        }
        Key::J => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.a_sharp;
                })
                .unwrap();
        }
        Key::Comma => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.c1;
                })
                .unwrap();
        }
        Key::Period => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.d1;
                })
                .unwrap();
        }
        Key::Slash => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.e1;
                })
                .unwrap();
        }
        Key::Semicolon => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.c_sharp1;
                })
                .unwrap();
        }
        Key::Apostrophe => {
            model
                .stream
                .send(move |audio| {
                    audio.hz = notes.d_sharp1;
                })
                .unwrap();
        }
        _ => {}
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
}
