## Scope Management Plan: "Entrolibrium" Paper

**Paper Title (Suggestion):** Entrolibrium: A Weighted-Entropy Refinement for Equilibria in Zero-Sum Games

**Focus:** This paper introduces and analyzes a unique equilibrium refinement for two-player, zero-sum, perfect-information finite games based on a weighted entropy score used for lexicographical tie-breaking.

**Core Scope (Central Observations):**

1.  **Introduction & Motivation:**
    * Need for refinement concepts beyond standard minimax/Nash equilibrium in zero-sum games.
    * Introduce the idea of using move entropy as a natural tie-breaker, reflecting strategic considerations like unpredictability or maintaining options.
    * Motivate the need for player-specific *weighting* of entropy, especially in potentially asymmetric games or scenarios.
    * Define game context: 2-player, zero-sum, perfect information, finite horizon.

2.  **The Weighted Entropy Refinement Model `(P, T_w)`:**
    * Define the Primary Outcome `P` (scalar value representing win/loss/draw from minimax).
    * Define the "distribution game" where players choose probability distributions `D` over moves.
    * Define the **Weighted Entropy Balance Score** `T_w` as the secondary lexicographical objective:
        * `T_A = w_A Σ H(D_A) - w_B Σ H(D_B)`
        * `T_B = w_B Σ H(D_B) - w_A Σ H(D_A)` (where `T_A + T_B = 0`)
        * `w_A, w_B > 0` are player-specific weights. `H(D)` is Shannon entropy. Sums are over the game path.
    * Define the player's objective: Lexicographically maximize `(E[P], E[T_w])`.

3.  **Existence and Uniqueness of "Entrolibrium":**
    * Prove that the backward induction algorithm applied to the distribution game with the `(P, T_w)` lexicographical payoff yields a unique optimal strategy profile (unique optimal distribution `D*` at each decision node).
    * Highlight the role of the strictly concave term `w_i H(D_i)` (for `w_i > 0`) in guaranteeing a unique maximizer during the final tie-breaking step.

4.  **Structure of Optimal Distributions:**
    * Derive the mathematical form of the optimal distribution `D*` chosen at `P`-tie-breaks.
    * Show its connection to the **softmax function**, where the player's weight `w_i` acts inversely like a temperature parameter: `p(m)* ∝ exp(E[T_{w, future} | m] / w_i)`.
    * Discuss the interpretation of the weights `w_A, w_B` in influencing the "randomness" of the strategy.

5.  **Computational Aspects & Sensitivity:**
    * Discuss computability via backward induction (in principle for finite games).
    * Highlight the likely computational efficiency gained from the closed-form (softmax-like) nature of the optimal distribution calculation compared to generic optimization.
    * Briefly discuss the sensitivity of the resulting Entrolibrium strategy to small perturbations in the primary payoffs `P` (mentioning how this implicitly relates to unmodeled secondary scores `S`).

6.  **Example:**
    * Illustrate the concept with a small game, potentially Tic-Tac-Toe or the 4x4 pawn game. Show how the `(P, T_w)` values are calculated and how different weights `w_A, w_B` might influence the optimal starting distribution (if the game allows).

7.  **Discussion & Conclusion:**
    * Summarize the Entrolibrium concept and its properties (uniqueness).
    * Discuss potential interpretations and applications.
    * Briefly contrast the motivation with other refinements (e.g., error-based like THPE).


**Tangential / Future Work (To be mentioned briefly, not elaborated):**

1.  **Secondary Scores (`S`):** Formal extension to `(P, S, T_w)` model for multi-objective refinement.
2.  **Heuristic Search & Finite Temperature:** Application in practical game AI (e.g., MCTS) using `T_w` estimates and possibly relating player weights `w_i` or an external parameter to temperature in search algorithms.
3.  **Imperfect Information Games:** Exploring the role of entropy concepts in games involving beliefs and hidden states.
4.  **Alternative Entropy/Complexity Measures:** Investigating different functions beyond Shannon entropy for `T`.
5.  **Specific Game Analyses:** Deeper application to chess endgames, Go, etc.

**Explicitly Out of Scope:**

* Detailed implementation guides for tablebases or MCTS.
* Extensive comparisons with the full spectrum of game theory refinements.
* Non-zero-sum game extensions.
* Formal treatment of infinite games.

This revised scope centers the paper firmly on the **weighted entropy balance `T_w`** as the core novel contribution, presenting its definition, uniqueness proof, structure, and basic properties within the `(P, T_w)` lexicographical framework, while clearly signposting related ideas as future work.
