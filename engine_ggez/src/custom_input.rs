use ggez::{Context, input::mouse, mint::Point2};

#[derive(Copy, Clone)]
pub enum MouseInput {
    Idle,
    Clicked {
        pos: Point2<f32>,
    },
    Holding {
        pos: Point2<f32>,
        dur: f32,
    },
    Dragging {
        start_pos: Point2<f32>,
        curr_pos: Point2<f32>,
    },
}

pub struct MouseState {
    pub state: MouseInput,
    click_taimer: f32,
    click_to_hold_interval: f32,
    click_to_drag_distance: f32,
    start_pos: Point2<f32>,
    curr_pos: Point2<f32>,
    key_down: bool,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            state: MouseInput::Idle,
            click_taimer: 0.0,
            click_to_hold_interval: 0.2,
            click_to_drag_distance: 0.5,
            start_pos: Point2 { x: 0.0, y: 0.0 },
            curr_pos: Point2 { x: 0.0, y: 0.0 },
            key_down: false,
        }
    }
}

impl MouseState {
    fn decide_state(&mut self, ctx: &Context) {
        if self.click_taimer > self.click_to_hold_interval {
            self.state = MouseInput::Holding {
                pos: mouse::position(ctx),
                dur: self.click_taimer,
            }
        } else {
            self.state = MouseInput::Clicked {
                pos: self.start_pos,
            };
        }

        let distance = (self.start_pos.x - self.curr_pos.x).powi(2)
            + (self.start_pos.y - self.curr_pos.y).powi(2);

        if distance.sqrt() > self.click_to_drag_distance {
            self.state = MouseInput::Dragging {
                start_pos: self.start_pos,
                curr_pos: self.curr_pos,
            }
        }
    }

    pub fn determine_input(&mut self, delta_time: f32, ctx: &Context) {
        if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
            if !self.key_down {
                self.start_pos = mouse::position(ctx);
                self.curr_pos = mouse::position(ctx);
                self.key_down = true;
            }
            self.click_taimer += delta_time;
            self.curr_pos = mouse::position(ctx);

            self.decide_state(ctx);
        } else {
            self.decide_state(ctx);

            self.click_taimer = 0.0;
            self.key_down = false;
            self.state = MouseInput::Idle;
        }
    }
}
