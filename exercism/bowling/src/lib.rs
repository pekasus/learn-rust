#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    frame: u16,
    pins_left: u16,
    throw: u16,

    prev_throw: u8,
    prev_prev_throw: u8,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            score: 0,
            frame: 1,
            pins_left: 10,
            throw: 0,
            prev_throw: 0,
            prev_prev_throw: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins_left < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.frame > 10 {
            return Err(Error::GameComplete);
        }
        self.pins_left -= pins;

        self.score_pins(pins);
        match (self.frame, self.throw, self.pins_left) {
            (10, 2, _) => self.frame = 11,
            (10, 1, 0) => {
                self.throw = 2;
                self.pins_left = 10;
            }
            (10, 1, _) => {
                if self.prev_prev_throw > 0 {
                    self.throw = 2;
                } else {
                    self.frame = 11;
                }
            }
            (10, 0, 0) => {
                self.throw = 1;
                self.pins_left = 10;
            }
            (10, 0, _) => self.throw = 1,
            (_, 1, _) => {
                self.frame += 1;
                self.throw = 0;
                self.pins_left = 10;
            }
            (_, 0, 0) => {
                self.frame += 1;
                self.pins_left = 10;
            }
            (_, 0, _) => self.throw = 1,
            (_, _, _) => unreachable!(),
        };
        Ok(())
    }

    fn score_pins(&mut self, pins: u16) {
        self.score += pins;
        if self.prev_prev_throw > 0 && self.frame != 10 {
            self.score += pins;
        }

        if self.prev_throw > 0 && self.frame != 10 {
            self.score += pins;
        }

        if self.frame == 10 {
            if self.throw == 0 {
                if self.prev_throw > 0 {
                    self.score += pins;
                }
                if self.prev_prev_throw == 1 {
                    self.score += pins;
                }
            }
            if self.throw == 1 {
                if self.prev_prev_throw == 1 {
                    self.score += pins;
                }
            }
        }

        if self.prev_throw == 2 {
            self.prev_prev_throw = 1;
        } else {
            self.prev_prev_throw = 0;
        }

        if self.pins_left == 0 {
            if self.throw == 0 {
                self.prev_throw = 2;
            } else {
                self.prev_throw = 1;
            }
        } else {
            self.prev_throw = 0;
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame == 11 {
            Some(self.score)
        } else {
            None
        }
    }
}
