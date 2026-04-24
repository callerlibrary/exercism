#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    current_frame_rolls: Vec<u16>, // 记录当前格已投中的瓶数，用于校验
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::new(),
            current_frame_rolls: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // 1. 检查瓶数是否合法
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // 2. 检查游戏是否已经结束
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        // 3. 检查当前格内逻辑（不能超过10瓶，除了第10格的特殊情况）
        let frame_count = self.get_frame_count();
        if frame_count < 10 {
            if !self.current_frame_rolls.is_empty() && self.current_frame_rolls[0] + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        } else {
            // 第10格逻辑：处理补分球 (Fill Balls)
            let last_frame = &self.current_frame_rolls;
            if last_frame.len() == 1 {
                if last_frame[0] < 10 && last_frame[0] + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if last_frame.len() == 2 {
                // 如果前两投是 XX，第三投不能超过10；如果前两投是 X 5，第三投不能让 5 + pins > 10
                if last_frame[0] == 10 && last_frame[1] < 10 && last_frame[1] + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
        }

        // 记录投球
        self.rolls.push(pins);
        self.current_frame_rolls.push(pins);

        // 切换格子逻辑
        if frame_count < 10 {
            if pins == 10 || self.current_frame_rolls.len() == 2 {
                self.current_frame_rolls.clear();
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let mut total_score = 0;
        let mut roll_idx = 0;

        for _ in 0..10 {
            if self.rolls[roll_idx] == 10 {
                // Strike: 10 + 后两次投球
                total_score += 10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2];
                roll_idx += 1;
            } else if self.rolls[roll_idx] + self.rolls[roll_idx + 1] == 10 {
                // Spare: 10 + 后一次投球
                total_score += 10 + self.rolls[roll_idx + 2];
                roll_idx += 2;
            } else {
                // Open Frame
                total_score += self.rolls[roll_idx] + self.rolls[roll_idx + 1];
                roll_idx += 2;
            }
        }

        Some(total_score)
    }

    // 辅助方法：判断游戏是否完成
    fn is_complete(&self) -> bool {
        let mut roll_idx = 0;
        for frame in 0..10 {
            if roll_idx >= self.rolls.len() { return false; }

            if self.rolls[roll_idx] == 10 { // Strike
                roll_idx += 1;
            } else { // Spare or Open
                if roll_idx + 1 >= self.rolls.len() { return false; }
                roll_idx += 2;
            }

            if frame == 9 { // 第10格特殊校验
                // 如果是 Strike 或 Spare，需要填满补分球
                if self.rolls[roll_idx - 1] == 10 || (roll_idx >= 2 && self.rolls[roll_idx-1] + self.rolls[roll_idx-2] == 10 && self.is_spare_at(roll_idx-1)) {
                    // Strike 或 Spare 至少需要第3个球（即 roll_idx 位置要有值）
                    // 这里的逻辑稍微复杂，简单化处理：
                }
            }
        }

        // 最终检查：根据计分逻辑所需的投球数是否已满足
        self.check_final_completion()
    }

    fn is_spare_at(&self, idx: usize) -> bool {
        // 辅助判断是否是补中
        idx > 0 && self.rolls[idx] + self.rolls[idx-1] == 10
    }

    fn get_frame_count(&self) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < self.rolls.len() && count < 10 {
            if self.rolls[i] == 10 {
                i += 1;
            } else {
                i += 2;
            }
            count += 1;
        }
        if count == 0 { 1 } else { count }
    }

    fn check_final_completion(&self) -> bool {
        let mut idx = 0;
        for frame in 0..10 {
            if idx >= self.rolls.len() { return false; }
            if frame == 9 {
                // 第10格
                if self.rolls[idx] == 10 { // 第10格第一投Strike
                    return self.rolls.len() == idx + 3;
                } else if idx + 1 < self.rolls.len() && self.rolls[idx] + self.rolls[idx+1] == 10 { // Spare
                    return self.rolls.len() == idx + 3;
                } else { // Open
                    return self.rolls.len() == idx + 2;
                }
            }
            if self.rolls[idx] == 10 { idx += 1; }
            else { idx += 2; }
        }
        true
    }
}