use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

struct Game {
    team1: i32,
    team2: i32,
    max_bags: i32,
    winning_score: i32,
}

impl Game {
    fn add_round_score(&mut self, p1_hole: i32, p1_board: i32, p2_hole: i32, p2_board: i32) {
        let p1_points = (p1_hole * 3) + (p1_board * 1);
        let p2_points = (p2_hole * 3) + (p2_board * 1);
        let diff = p1_points - p2_points;

        if diff > 0 {
            self.team1 += diff;
        } else if diff < 0 {
            self.team2 += -diff;
        }

        if self.team1 > self.winning_score {
            self.team1 /= 2;
        }
        if self.team2 > self.winning_score {
            self.team2 /= 2;
        }
    }

    fn show_score(&self) {
        // Keeping the original message (including the small typo) to match the Go output
        println!("Curent Score Team1 {} Team 2 {}", self.team1, self.team2);
    }

    fn simulate_team(&self) -> (i32, i32) {
        let mut holes = 0;
        let mut board = 0;
        let mut rng = rand::thread_rng();

        for _ in 0..self.max_bags {
            let r: i32 = rng.gen_range(0..100);
            if r < 20 {
                holes += 1;
            } else if r < 60 {
                board += 1;
            }
        }
        (holes, board)
    }

    fn check_winner(&self) -> bool {
        if self.team1 >= self.winning_score && self.team1 > self.team2 {
            println!("Team 1 wins");
            true
        } else if self.team2 >= self.winning_score && self.team2 > self.team1 {
            println!("Team 2 wins");
            true
        } else {
            false
        }
    }
}

fn main() {
    println!("This is init."); // mirrors Go's init()

    let mut game = Game {
        team1: 0,
        team2: 0,
        max_bags: 4,
        winning_score: 11,
    };

    let mut round = 1;
    loop {
        println!("Round {}", round);
        let (p1_hole, p1_board) = game.simulate_team();
        let (p2_hole, p2_board) = game.simulate_team();
        game.add_round_score(p1_hole, p1_board, p2_hole, p2_board);
        game.show_score();
        if game.check_winner() {
            break;
        }
        round += 1;
        sleep(Duration::from_secs(1));
    }
}
