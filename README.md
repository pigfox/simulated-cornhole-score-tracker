# Simulated Cornhole Score Tracker

A simple Rust simulation of a **Cornhole** (bean bag toss) game between two teams.  
This project randomly generates each team’s throws and keeps track of scores until a winner is declared.

---

## 🎯 Game Rules in This Simulation

- Each **round**, both teams throw up to `max_bags` bean bags.
- For each bag:
    - **Hole** = 3 points
    - **On Board** = 1 point
    - **Miss** = 0 points
- Points for each team in a round are **compared**:
    - Only the **point difference** is added to the winning team’s total.
- If a team’s score exceeds the `winning_score` threshold, their score is **halved**.
- The first team to **reach or exceed** the `winning_score` **and** have more points than the opponent wins.

---

## ⚙️ Configuration

| Variable        | Meaning                                         |
|-----------------|-------------------------------------------------|
| `max_bags`      | Number of bags thrown per team per round        |
| `winning_score` | Score required to win the game                   |

These values are set in `main.rs`:
```rust
let mut game = Game {
    team1: 0,
    team2: 0,
    max_bags: 4,       // change here
    winning_score: 11, // change here
};
```
🎲 Randomness
The simulate_team method uses random numbers to determine outcomes:

20% chance: Hole (3 points)

40% chance: On Board (1 point)

40% chance: Miss (0 points)

🏁 How to Run

With Cargo (recommended)
```
cargo run
```


