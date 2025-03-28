// main.rs
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

const BOARD_SIZE: usize = 9;

// --- Player Enum ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    X,
    O,
}

impl Player {
    fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

// --- Board Struct ---
// Using u64 for bitboard might be faster, but Option<Player> is easier to read
// Manual Hash implementation needed because arrays > 32 elements don't auto-derive Hash
#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    cells: [Option<Player>; BOARD_SIZE],
    hash_cache: Option<u64>, // Cache the hash for performance
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.hash_cache {
            Some(hash) => state.write_u64(hash),
            None => {
                // Calculate hash if not cached (should ideally be done on creation/mutation)
                // Simple hash for demonstration: treat Option<Player> as 0, 1, 2
                let mut temp_hasher = DefaultHasher::new();
                for cell in &self.cells {
                     match cell {
                        None => 0u8.hash(&mut temp_hasher),
                        Some(Player::X) => 1u8.hash(&mut temp_hasher),
                        Some(Player::O) => 2u8.hash(&mut temp_hasher),
                    }
                }
                let calculated_hash = temp_hasher.finish();
                // This hashing is not ideal within hash() method due to mutable hasher state needed
                // Better approach is to precompute hash on board creation/modification
                // For now, just hash the raw cells data:
                self.cells.hash(state);
            }
        }
    }
}


impl Board {
    fn new() -> Self {
        Board { cells: [None; BOARD_SIZE], hash_cache: None } // Hash cache could be computed here
    }

    // Returns a new board with the move made
    fn make_move(&self, index: usize, player: Player) -> Option<Board> {
        if index < BOARD_SIZE && self.cells[index].is_none() {
            let mut new_cells = self.cells;
            new_cells[index] = Some(player);
            // In a real implementation, compute and store hash_cache here
            Some(Board { cells: new_cells, hash_cache: None })
        } else {
            None
        }
    }

    fn get_legal_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| if cell.is_none() { Some(i) } else { None })
            .collect()
    }

    // Check if the game has ended
    fn check_outcome(&self) -> Option<Outcome> {
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // cols
            [0, 4, 8], [2, 4, 6], // diagonals
        ];

        for line in lines {
            let first = self.cells[line[0]];
            if first.is_some() && first == self.cells[line[1]] && first == self.cells[line[2]] {
                 return Some(Outcome::Win(first.unwrap()));
            }
        }

        if self.cells.iter().all(|cell| cell.is_some()) {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
}

// --- Outcome Enum ---
#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Win(Player),
    Draw,
}

// --- Memoization ---
type Memo = HashMap<Board, (f64, f64)>; // Stores (E[P], E[T]) for player TO MOVE

// --- Minimax with Entropy ---
// Returns (E[P], E[T]) from the perspective of `player` needing to move on `board`
fn minimax_eos(board: &Board, player: Player, memo: &mut Memo) -> (f64, f64) {

    // 1. Check for Terminal State
    if let Some(outcome) = board.check_outcome() {
        return match outcome {
            Outcome::Win(p) => (if p == player { 1.0 } else { -1.0 }, 0.0), // T=0 at end
            Outcome::Draw => (0.0, 0.0), // P=0, T=0 at end
        };
    }

    // 2. Check Memoization Table
    // Note: Key should ideally include the player whose turn it is if not implicit
    // Here, the returned value is always for `player` moving on `board`
    if let Some(&result) = memo.get(board) {
        return result;
    }

    // 3. Recursive Exploration
    let legal_moves = board.get_legal_moves();
    let mut move_results = Vec::new(); // Stores (move_index, E[P]_current, E[T_future]_current)

    for m_idx in legal_moves {
        // Assume make_move always succeeds for legal moves
        let next_board = board.make_move(m_idx, player).unwrap();
        // Recursive call for the opponent on the next state
        let (opp_p, opp_t) = minimax_eos(&next_board, player.opponent(), memo);
        // Convert result to current player's perspective: V' = -V_opp
        let current_p = -opp_p;
        let current_t_future = -opp_t; // This is the E[T] from the next state onwards
        move_results.push((m_idx, current_p, current_t_future));
    }

    // 4. Find Optimal P-value
    let max_p = move_results
        .iter()
        .map(|(_, p, _)| *p)
        .fold(f64::NEG_INFINITY, f64::max);

    // 5. Filter moves achieving max P
    let optimal_moves: Vec<(usize, f64, f64)> = move_results
        .into_iter()
        .filter(|(_, p, _)| (*p - max_p).abs() < 1e-9) // Float comparison
        .collect();

    // 6. Calculate Node Value (E[P], E[T]) including current entropy H(D*)
    let final_e_t: f64;
    let optimal_distribution: HashMap<usize, f64>;

    if optimal_moves.len() == 1 {
        // Only one optimal move -> Deterministic choice D*
        let (m_idx, _, vt_future) = optimal_moves[0];
        final_e_t = 0.0 + vt_future; // H(D*=deterministic) = 0
        optimal_distribution = std::iter::once((m_idx, 1.0)).collect();

    } else {
        // Multiple P-optimal moves -> Entropy tie-breaking
        let move_data: Vec<(usize, f64)> = optimal_moves.iter().map(|(idx, _, vt_future)| (*idx, *vt_future)).collect();

        // Softmax calculation for optimal distribution D* = {p(m)*} based on E[T_future] values
        let max_vt = move_data.iter().map(|(_, vt)| *vt).fold(f64::NEG_INFINITY, f64::max);
        // Stabilize exponents
        let exps: Vec<f64> = move_data.iter().map(|(_, vt)| (vt - max_vt).exp()).collect();
        let sum_exps: f64 = exps.iter().sum();

        let probs: HashMap<usize, f64> = if sum_exps > 1e-9 { // Avoid division by zero/tiny numbers
             move_data.iter().zip(exps.iter()).map(|((idx, _), exp_val)| (*idx, exp_val / sum_exps)).collect()
        } else {
            // If all VTs are extremely low or equal leading to sum_exps ~ 0, use uniform
            let uniform_prob = 1.0 / move_data.len() as f64;
            move_data.iter().map(|(idx, _)| (*idx, uniform_prob)).collect()
        };
        optimal_distribution = probs;

        // Calculate H(D*) using natural logarithm (ln)
        let entropy = optimal_distribution.values().fold(0.0, |acc, &p| {
            if p > 1e-9 { acc - p * p.ln() } else { acc } // Handle p=0 case (lim p*ln(p) = 0)
        });

        // Calculate E[T_future] = Σ p(m)* V'_T(s_m)
        let expected_future_t = move_data.iter().fold(0.0, |acc, (idx, vt_future)| {
            acc + optimal_distribution.get(idx).unwrap_or(&0.0) * vt_future
        });

        // Total E[T] for this node = H(D*) + E[T_future]
        final_e_t = entropy + expected_future_t;
    }

    // 7. Store result in memo table before returning
    memo.insert(board.clone(), (max_p, final_e_t));

    (max_p, final_e_t)
}


// --- Main Function ---
fn main() {
    let initial_board = Board::new();
    let mut memo: Memo = HashMap::new();
    let mut initial_move_results : HashMap<usize, (f64, f64)> = HashMap::new(); // Store results for printing

    println!("Calculating EOS for Tic-Tac-Toe first move (Player X)...");

    // Calculate value for each possible first move
    for m in 0..BOARD_SIZE {
        // Assumes Player X moves first
        let next_board = initial_board.make_move(m, Player::X).unwrap();
        // Get value from O's perspective looking at the board after X's move `m`
        let (opp_p, opp_t) = minimax_eos(&next_board, Player::O, &mut memo);
        // Convert to X's perspective for the *outcome* of move `m`
        let current_p = -opp_p;
        let current_t_future = -opp_t; // This is E[T] expected from the state *after* move m
        initial_move_results.insert(m, (current_p, current_t_future));
        // Optional: Print intermediate results
        // println!("  Move {}: E[P] = {:.3}, E[T_future] = {:.3}", m, current_p, current_t_future);
    }

    // Find the best P value X can achieve (should be 0.0 for TicTacToe optimal play)
    let best_p = initial_move_results
        .values()
        .map(|(p, _)| *p)
        .fold(f64::NEG_INFINITY, f64::max);

     println!("\nMax E[P] achievable by X = {:.3}", best_p);

    // Identify all first moves that achieve this best P value
    let p_optimal_moves: Vec<(usize, f64)> = initial_move_results
        .iter()
        .filter(|(_, (p, _))| (*p - best_p).abs() < 1e-9)
        .map(|(m_idx, (_, et_future))| (*m_idx, *et_future) ) // Get (move_idx, E[T_future])
        .collect();

    // Calculate the optimal distribution D* over these P-optimal moves based on E[T_future]
    let final_probabilities: HashMap<usize, f64>;

    if p_optimal_moves.is_empty() {
        println!("Error: No optimal moves found?");
        final_probabilities = HashMap::new();
    } else if p_optimal_moves.len() == 1 {
         println!("Unique optimal first move (P-based): {}", p_optimal_moves[0].0);
         final_probabilities = std::iter::once((p_optimal_moves[0].0, 1.0)).collect();
    } else {
        println!("Multiple moves achieve max E[P] ({} moves). Calculating entropy-optimal distribution...", p_optimal_moves.len());

        // Softmax calculation D* = {p(m)*} using E[T_future] as the potential V'_T
        let max_vt = p_optimal_moves.iter().map(|(_, vt)| *vt).fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = p_optimal_moves.iter().map(|(_, vt)| (vt - max_vt).exp()).collect();
        let sum_exps: f64 = exps.iter().sum();

        let probs: HashMap<usize, f64> = if sum_exps > 1e-9 {
             p_optimal_moves.iter().zip(exps.iter()).map(|((idx, _), exp_val)| (*idx, exp_val / sum_exps)).collect()
        } else {
            let uniform_prob = 1.0 / p_optimal_moves.len() as f64;
            p_optimal_moves.iter().map(|(idx, _)| (*idx, uniform_prob)).collect()
        };
        final_probabilities = probs;
    }

    // --- Print Final Results ---
    println!("\nEntropy Optimal Strategy (First Move Probabilities):");
    println!("Move | E[T_future] | Probability (p*)");
    println!("-----|-------------|-----------------");

    for m in 0..BOARD_SIZE {
        // Get the E[T_future] calculated earlier for this move
        let (_, et_future) = initial_move_results.get(&m).unwrap_or(&(f64::NEG_INFINITY, f64::NEG_INFINITY));
        // Get the probability from the final optimal distribution
        let probability = final_probabilities.get(&m).unwrap_or(&0.0); // p*=0 if move m was not P-optimal
        println!("{:^4} | {:^11.3} | {:^15.5}", m, et_future, probability);
    }

    // --- Optional: Calculate Overall E[T] for X at the start ---
     if !p_optimal_moves.is_empty() {
         // Calculate H(D*) for the optimal first move distribution
         let entropy_at_root = final_probabilities.values().fold(0.0, |acc, &p| {
            if p > 1e-9 { acc - p * p.ln() } else { acc }
         });

         // Calculate E[T_future] = Σ p*(m) * E[T_future after move m]
         let expected_future_t_at_root = p_optimal_moves.iter().fold(0.0, |acc, (idx, vt_future)| {
            acc + final_probabilities.get(idx).unwrap_or(&0.0) * vt_future
         });

         // Overall E[T] for Player X if they play optimally from the start
         let overall_et_root = entropy_at_root + expected_future_t_at_root;
         println!("\nOverall E[T] for Player X from start: {:.5}", overall_et_root);
         println!("  (Calculated as H(D*_root) + E[T_future | D*_root] = {:.5} + {:.5})", entropy_at_root, expected_future_t_at_root);
     }
}
