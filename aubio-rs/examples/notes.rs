/*!
 * This example demonstrates how to implement simple music notes recognizer.
 *
 * The functionality is pretty similar as `aubionotes` commandline tool.
 *
 * Usage: cat music.wav | notes
 *
 * Only signed 16-bit mono wav files supported.
 */

use aubio_rs::{Notes, Smpl};
use hound::WavReader;
use std::io::stdin;

const BUF_SIZE: usize = 512;
const HOP_SIZE: usize = 256;

const I16_TO_SMPL: Smpl = 1.0 / (1 << 16) as Smpl;

fn main() {
    let input = stdin();
    let mut reader = WavReader::new(input).unwrap();
    let format = reader.spec();

    let mut samples = reader.samples();
    let mut notes = Notes::new(BUF_SIZE, HOP_SIZE, format.sample_rate).unwrap();

    let period = 1.0 / format.sample_rate as Smpl;

    let mut time = 0.0;
    let mut offset = 0;

    loop {
        let block = samples
            .by_ref()
            .map(|sample| sample.map(|sample: i16| sample as Smpl * I16_TO_SMPL))
            .take(HOP_SIZE)
            .collect::<Result<Vec<Smpl>, _>>()
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
        time = offset as Smpl * period;

        if block.len() < HOP_SIZE {
            break;
        }
    }

    println!("{}", time);
}
