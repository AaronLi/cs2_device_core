use std::time::{Duration, Instant};

pub enum BombState{
    Planting{input_buffer: [u8; 7], inputted_digits: usize},
    Defusable{start_time: Instant, maybe_defuse_start_time: Option<Instant>},
    Exploded,
    Defused
}

pub struct BombCore {
    state: BombState,
    defuse_time: Duration,
    fuse_time: Duration,
    on_state_transition: fn(from: &BombState, to: &BombState) -> ()
}

impl BombCore {
    pub fn new(defuse_time: Duration, fuse_time: Duration, on_state_transition: fn(from: &BombState, to: &BombState)) -> Self {
        BombCore {
            state: BombState::Planting {input_buffer: [0; 7], inputted_digits: 0 },
            defuse_time,
            fuse_time,
            on_state_transition,
        }
    }
    pub fn button_press(&mut self, button: u8) {
        match &mut self.state {
            BombState::Planting { input_buffer, inputted_digits } => {
                if *inputted_digits + 1 < input_buffer.len() {
                    input_buffer[*inputted_digits] = button;
                    *inputted_digits += 1;
                }else {
                    if *input_buffer == [7,3,5,5,6,0,8] {
                        self.transition_state(BombState::Defusable {start_time: Instant::now(), maybe_defuse_start_time: None});
                    }
                }
            }
            _ => {}
        }
    }

    pub fn start_defuse(&mut self) {
        if let BombState::Defusable {maybe_defuse_start_time, .. } = &mut self.state {
            *maybe_defuse_start_time = Some(Instant::now());
        }
    }

    pub fn abort_defuse(&mut self) {
        if let BombState::Defusable {maybe_defuse_start_time, .. } = &mut self.state {
            *maybe_defuse_start_time = None;
        }
    }

    pub fn update(&mut self) {
        match &mut self.state {
            BombState::Defusable { start_time, maybe_defuse_start_time } => {
                if start_time.elapsed() > self.fuse_time {
                    self.transition_state(BombState::Exploded)
                } else if let Some(maybe_defuse_start_time) = maybe_defuse_start_time && maybe_defuse_start_time.elapsed() >= self.defuse_time {
                    self.transition_state(BombState::Defused)
                }
            },
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.transition_state(BombState::Planting {input_buffer: [0; 7], inputted_digits: 0});
    }

    fn transition_state(&mut self, new_state: BombState) {
        (self.on_state_transition)(&self.state, &new_state);
        self.state = new_state;
    }
}
