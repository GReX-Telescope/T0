use crate::common::{Payload, CHANNELS};
use crossbeam_channel::{Receiver, Sender};

pub type Stokes = [f32; CHANNELS];

#[allow(clippy::missing_panics_doc)]
pub fn downsample_thread(
    payload_recv: &Receiver<Payload>,
    stokes_send: &Sender<Stokes>,
    downsample_factor: u16,
) {
    println!("Starting downsample task");
    // Preallocate averaging vector
    let mut avg_buf = vec![[0u16; CHANNELS]; downsample_factor as usize];
    let mut idx = 0usize;
    loop {
        // Grab the next payload
        let payload = payload_recv.recv().unwrap();
        // Calculate stokes into the averaging buf
        avg_buf[idx] = payload.stokes_i();
        // If we're at the end, we're done
        if idx == downsample_factor as usize - 1 {
            // Find the average into an f32 (which is lossless)
            let mut avg = [0f32; CHANNELS];
            for chan in 0..CHANNELS {
                for avg_row in avg_buf.iter().take(downsample_factor as usize) {
                    avg[chan] += f32::from(avg_row[chan]);
                }
            }
            avg.iter_mut()
                .for_each(|v| *v /= f32::from(downsample_factor));
            // And send out
            stokes_send.send(avg).unwrap();
        }
        // Increment the idx
        idx = (idx + 1) % downsample_factor as usize;
    }
}
