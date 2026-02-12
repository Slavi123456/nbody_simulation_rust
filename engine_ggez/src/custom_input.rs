use ggez::{Context, input::mouse, mint::Point2};

#[derive(Debug, Copy, Clone)]
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

enum MouseDraggingRestriction {
    Yes { max_distance: f32 },
    No,
}

pub struct MouseState {
    pub latest_state: MouseInput,
    pub state: MouseInput,
    click_taimer: f32,
    click_to_hold_interval: f32,
    click_to_drag_distance: f32,
    start_pos: Point2<f32>,
    curr_pos: Point2<f32>,
    key_down: bool,
    drag_restriction: MouseDraggingRestriction,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            latest_state: MouseInput::Idle,
            state: MouseInput::Idle,
            click_taimer: 0.0,
            click_to_hold_interval: 0.2,
            click_to_drag_distance: 0.5,
            start_pos: Point2 { x: 0.0, y: 0.0 },
            curr_pos: Point2 { x: 0.0, y: 0.0 },
            key_down: false,
            drag_restriction: MouseDraggingRestriction::No,
        }
    }
    pub fn set_drag_restriction(mut self, distance: f32) -> Self {
        self.drag_restriction = MouseDraggingRestriction::Yes {
            max_distance: distance,
        };
        self
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

            self.latest_state = self.state;
            self.decide_state(ctx);
        } else {
            // self.decide_state(ctx);
            self.latest_state = self.state;
            self.click_taimer = 0.0;
            self.key_down = false;
            self.state = MouseInput::Idle;
        }
        // println!("Latest state {:?} curr {:?}", self.latest_state, self.state);
    }

    pub fn is_restricted(&self) -> bool {
        match self.drag_restriction {
            MouseDraggingRestriction::No => false,
            MouseDraggingRestriction::Yes { .. } => true,
        }
    }

    pub fn is_draggin(&self) -> bool {
        match self.state {
            MouseInput::Dragging { .. } => true,
            _ => false,
        }
    }

    pub fn start_pos(&self) -> Point2<f32> {
        self.start_pos
    }
    pub fn curr_pos(&self) -> Point2<f32> {
        self.curr_pos
    }
    pub fn just_released_after_drag(&self) -> bool {
        matches!(self.latest_state, MouseInput::Dragging { .. })
            && matches!(self.state, MouseInput::Idle)
    }

    pub fn distance_in_drag(&self) -> engine_core::glam::Vec2 {
        match self.latest_state {
            MouseInput::Dragging { .. } => {
                let drag_vec_x = self.start_pos.x - self.curr_pos.x;
                let drag_vec_y = self.start_pos.y - self.curr_pos.y;

                let drag_vec = engine_core::glam::Vec2::new(drag_vec_x, drag_vec_y);

                drag_vec
            }
            _ => engine_core::glam::Vec2::new(0.0, 0.0),
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

        let distance = self.curr_mouse_distance();

        if distance.sqrt() > self.click_to_drag_distance {
            self.state = MouseInput::Dragging {
                start_pos: self.start_pos,
                curr_pos: self.curr_pos,
            };
            if let MouseDraggingRestriction::Yes { max_distance } = self.drag_restriction {
                let start = engine_core::glam::Vec2::new(self.start_pos.x, self.start_pos.y);
                let curr = engine_core::glam::Vec2::new(self.curr_pos.x, self.curr_pos.y);

                let drag_vec = curr - start;
                let distance = drag_vec.length();

                let final_vec = if distance > max_distance {
                    start + drag_vec.normalize() * max_distance
                } else {
                    curr
                };

                self.curr_pos = ggez::mint::Point2 {
                    x: final_vec.x,
                    y: final_vec.y,
                };
            }
        }
    }

    pub fn curr_mouse_distance(&self) -> f32 {
        let distance = (self.start_pos.x - self.curr_pos.x).powi(2)
            + (self.start_pos.y - self.curr_pos.y).powi(2);
        distance
    }
}
