// This is THE MOST convoluted wall of ifs I have ever designed.
//      I really wanted to realise this task without any special tagging
//      So, when I realised that I can score the result from the back and pass bonuses forward, I was locked-in

//TODO: Check if I can figure out, how to do this with persistent iterators (You can't go back in rounds anyway, right?)

// If not for the "Extra case" of Bonus rounds being counted differently (double counting, not double counting)
//      this task would have been much easier.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
// #![allow(clippy::cast_possible_wrap)]
// #![allow(clippy::cast_sign_loss)]

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    throw_id: usize,
    score_boxes: [[u8; 2]; 11],
}

impl BowlingGame {
    #[must_use]
    pub fn new() -> Self {
        BowlingGame::default()
    }

    // #[must_use]
    /// # Errors
    ///
    /// Will return `Err` if the move is illegal/impossible according to bowling rules
    /// This includes:
    ///     - Taking out more than 10 pins at once
    ///     - Taking out more than 10 pins in a frame
    ///     - The game has been completed (with bonuses realised)
    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if pins > 10 {
            // Check for illegal move
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.throw_id == 20 // First Bonus throw: Frame 11
                    && (self.score_boxes[9][0] + self.score_boxes[9][1]) == 10
        {
            // One bonus after (at least) spare
            self.score_boxes[self.throw_id / 2][0] = pins;
            self.throw_id += 1;
        } else if self.throw_id == 21 // Second bonus throw
                    && self.score_boxes[9][0] == 10
        {
            // Only after strike in first bonus
            // TODO: Check if error detection for bonus frame, cannot be moved outside of the bonus frame area.
            if self.score_boxes[10][0] != 10
                && (
                    pins == 10 // The_second_bonus_rolls_after_a_strike_in_the_last_frame_cannot_be_a_strike_if_the_first_one_is_not_a_strike
                    || self.score_boxes[10][0] + pins > 10
                    // two_bonus_rolls_after_a_strike_in_the_last_frame_cannot_score_more_than_10_points
                )
            {
                return Err(Error::NotEnoughPinsLeft);
            }
            self.score_boxes[self.throw_id / 2][1] = pins;
            self.throw_id += 1;
        } else if self.throw_id < 22 && (self.score_boxes[self.throw_id / 2][0] + pins) > 10 {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.throw_id < 20 {
            // Handling normal 20 throws for 10 frames
            if self.throw_id % 2 == 0 {
                // First throw
                if pins == 10 {
                    // Strike!
                    self.score_boxes[self.throw_id / 2][self.throw_id % 2] = pins;
                    self.throw_id += 1; // Note strike to first window
                } else {
                    // Score
                    self.score_boxes[self.throw_id / 2][self.throw_id % 2] = pins;
                }
                self.throw_id += 1;
            } else {
                // Second throw
                if self.throw_id <= 20 {
                    // Check if we are at the last shot
                    self.score_boxes[self.throw_id / 2][self.throw_id % 2] = pins;
                    self.throw_id += 1;
                }
            }
        } else {
            return Err(Error::GameComplete);
        }
        // TODO: Check if there are enough pins
        // TODO: Check if we are not finished with the game

        Ok(())
    }

    #[must_use]
    pub fn score(&self) -> Option<u16> {
        // Game done check
        if self.throw_id < 20 // Is it at least started && 10 frames filled
            || ( self.score_boxes[9][0] + self.score_boxes[9][1] == 10 // Do we get one bonus?
                && self.throw_id < 21 // Have we filled that bonus?
                || self.score_boxes[9][0] == 10 // Have we got two bonuses?
                && self.throw_id < 22 // Have we filled both bonuses?
            )
        {
            return None;
        }

        let mut bonus: [u8; 2] = [0, 0]; // How much to pass to next frame
        if (self.score_boxes[9][0] + self.score_boxes[9][1]) == 10 {
            // TODO: Check if this can be simplified!
            if self.score_boxes[9][0] == 10 {
                bonus = [self.score_boxes[10][0], self.score_boxes[10][1]];
            } else {
                bonus = [self.score_boxes[10][0], 0];
            }
        }

        Some(
            // After all the checks, we finally return the result!
            self.score_boxes
                .iter()
                .rev()
                .skip(1)
                .fold(0u16, |result: u16, frame| {
                    // Calc bonuses now
                    let bonuses: u8;
                    if frame[0] == 10 {
                        // strike
                        bonuses = bonus[0] + bonus[1];
                    } else if frame[0] + frame[1] == 10 {
                        // spare
                        bonuses = bonus[0];
                    } else {
                        // Open frame
                        bonuses = 0;
                    }

                    // Calc bonuses for next frames
                    if frame[0] == 10 {
                        // Strike, take bonus from this & next throw!
                        bonus[1] = bonus[0];
                        bonus[0] = 10;
                    } else if frame[0] + frame[1] == 10 {
                        // Spare, take bonus from this entire throw
                        bonus[1] = 10;
                        bonus[0] = frame[0];
                    } else {
                        // Open frame, take bonus only from first throw
                        bonus[1] = frame[1];
                        bonus[0] = frame[0];
                    }

                    // Return frame score
                    result + u16::from(bonuses + frame[0] + frame[1]) // This is safe, because we only ever add 20 + 10 + 10. Plenty enough for 255u8
                }),
        )
    }
}
