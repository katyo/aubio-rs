/*!
 * This example demonstrates how to implement simple music notes recognizer.
 *
 * The functionality is pretty similar as `aubionotes` commandline tool.
 *
 * Usage: cat music.wav | notes
 *
 * Only signed 16-bit mono wav files supported.
 */

use std::io::stdin;
use hound::WavReader;
use aubio_rs::Notes;

const BUF_SIZE: usize = 512;
const HOP_SIZE: usize = 256;

const I16_TO_F32: f32 = 1.0 / (1 << 16) as f32;

fn main() {
    let input = stdin();
    let mut reader = WavReader::new(input).unwrap();
    let format = reader.spec();

    let mut samples = reader.samples();
    let mut notes = Notes::new(BUF_SIZE, HOP_SIZE, format.sample_rate).unwrap();

    let period = 1.0 / format.sample_rate as f32;

    let mut time = 0.0;
    let mut offset = 0;

    loop {
        let block = samples
            .by_ref()
            .map(|sample| sample.map(|sample: i16| sample as f32 * I16_TO_F32))
            .take(HOP_SIZE)
            .collect::<Result<Vec<f32>, _>>()
            .unwrap();

        if block.len() == HOP_SIZE {
            for note in notes.do_result(block.as_slice().as_ref()).unwrap() {
                if note.velocity > 0.0 {
                    print!("{}\t{}\t", note.pitch, time);
                } else {
                    println!("{}", time);
                }
            }
        }

        offset += block.len();
        time = offset as f32 * period;

        if block.len() < HOP_SIZE {
            break;
        }
    }

    println!("{}", time);
}
