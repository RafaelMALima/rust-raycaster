use sdl2::{EventPump, event::Event};

pub fn event_handler(event_queue:&mut EventPump)->bool{
    for event in event_queue.poll_iter(){
        match event {
            Event::Quit { timestamp } => return false,
            _ => (),
        }
    }
    true
}
