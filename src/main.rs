use hound::{SampleFormat, WavSpec, WavWriter};
use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::BufWriter;

// Dual Tone Multi Frequency (DTMF)
//
// https://en.wikipedia.org/wiki/Dual-tone_multi-frequency_signaling
//
//       | 1209hz | 1336hz | 1477hz | 1633hz |
// 697hz | 1      | 2      | 3      | A      |
// 770hz | 4      | 5      | 6      | B      |
// 852hz | 7      | 8      | 9      | C      |
// 941hz | *      | 0      | #      | D      |

const X1: f32 = 1209.0;
const X2: f32 = 1336.0;
const X3: f32 = 1477.0;
const X4: f32 = 1336.0;

const Y1: f32 = 697.0;
const Y2: f32 = 770.0;
const Y3: f32 = 852.0;
const Y4: f32 = 941.0;

const TONE: [(f32, f32); 16] = [
    (X2, Y4), // 0
    (X1, Y1), // 1
    (X2, Y1), // 2
    (X3, Y1), // 3
    (X1, Y2), // 4
    (X2, Y2), // 5
    (X3, Y2), // 6
    (X1, Y3), // 7
    (X2, Y3), // 8
    (X3, Y3), // 9
    (X4, Y1), // A 10
    (X4, Y2), // B 11
    (X4, Y3), // C 12
    (X4, Y4), // D 13
    (X1, Y4), // * 14
    (X3, Y4), // # 15
];

const SAMPLE_RATE: u32 = 44100;
const AMPLITUDE: f32 = i16::MAX as f32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).unwrap_or_else(|| {
        panic!(
            "usage: {} [0123456789ABCD*#]",
            args.get(0).unwrap_or(&"phreaking".to_string())
        )
    });
    let input: Vec<usize> = input.chars().map(|c| tone_index(c)).collect();

    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create("tone.wav", spec).unwrap();

    let length = SAMPLE_RATE / 10;

    input.iter().for_each(|n| {
        write_tone(&mut writer, length, TONE[*n]);
        write_silence(&mut writer, length);
    });
}

fn tone_index(c: char) -> usize {
    match c.to_digit(10) {
        Some(d) => d as usize,
        None => match c {
            'a' | 'A' => 10,
            'b' | 'B' => 11,
            'c' | 'C' => 12,
            'd' | 'D' => 13,
            '*' => 14,
            '#' => 15,
            _ => panic!("invalid input"),
        },
    }
}

fn write_tone(writer: &mut WavWriter<BufWriter<File>>, length: u32, tone: (f32, f32)) {
    (0..length).for_each(|t| {
        let t = t as f32 / SAMPLE_RATE as f32;
        let x_sin = (t * tone.0 * 2.0 * PI).sin();
        let y_sin = (t * tone.1 * 2.0 * PI).sin();
        let sample = (x_sin + y_sin) / 2.0;
        writer.write_sample((sample * AMPLITUDE) as i16).unwrap();
    });
}

fn write_silence(writer: &mut WavWriter<BufWriter<File>>, length: u32) {
    (0..length).for_each(|_| writer.write_sample(0 as i16).unwrap());
}
