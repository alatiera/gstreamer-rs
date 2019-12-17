extern crate glib;
extern crate gstreamer as gst;
use gst::prelude::*;
use std::io::Write;

extern crate failure;
use failure::Error;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer
    gst::init()?;

    // Build the pipeline
    let uri =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gst::parse_launch(&format!("playbin uri={}", uri))?;

    // Start playing
    let res = pipeline.set_state(gst::State::Playing)?;
    let is_live = res == gst::StateChangeSuccess::NoPreroll;

    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let pipeline_weak = pipeline.downgrade();
    let bus = pipeline.get_bus().expect("Pipeline has no bus");
    bus.add_watch(move |_, msg| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };
        let main_loop = &main_loop_clone;
        match msg.view() {
            gst::MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                let _ = pipeline.set_state(gst::State::Ready);
                main_loop.quit();
            }
            gst::MessageView::Eos(..) => {
                // end-of-stream
                let _ = pipeline.set_state(gst::State::Ready);
                main_loop.quit();
            }
            gst::MessageView::Buffering(buffering) => {
                // If the stream is live, we do not care about buffering
                if is_live {
                    return glib::Continue(true);
                }

                let percent = buffering.get_percent();
                print!("Buffering ({}%)\r", percent);
                match std::io::stdout().flush() {
                    Ok(_) => {}
                    Err(err) => eprintln!("Failed: {}", err),
                };

                // Wait until buffering is complete before start/resume playing
                if percent < 100 {
                    let _ = pipeline.set_state(gst::State::Paused);
                } else {
                    let _ = pipeline.set_state(gst::State::Playing);
                }
            }
            gst::MessageView::ClockLost(_) => {
                // Get a new clock
                let _ = pipeline.set_state(gst::State::Paused);
                let _ = pipeline.set_state(gst::State::Playing);
            }
            _ => (),
        }
        glib::Continue(true)
    })
    .expect("Failed to add bus watch");

    main_loop.run();

    bus.remove_watch()?;
    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    match tutorials_common::run(tutorial_main) {
        Ok(_) => {}
        Err(err) => eprintln!("Failed: {}", err),
    }
}
