## Revised Scope Management Plan: Weighted Entropy Equilibrium Paper

**Paper Title (Suggestion):** Weighted Entropy Equilibrium: A Unique Refinement for Zero-Sum Games

*(Optional introductory sentence mentioning the informal project name "entrolibrium" for context/repository link, if desired.)*

**Focus:** This paper introduces and analyzes a unique equilibrium refinement for two-player, zero-sum, perfect-information finite games based on a *weighted* entropy score used for lexicographical tie-breaking. It establishes the equilibrium's properties and its connection to perturbed games.

**Core Scope (Central Observations):**

1.  **Introduction & Motivation:**
    * Need for refinements in zero-sum games.
    * Introduce entropy as a tie-breaker, motivating player-specific *weighting*.
    * Define game context (2-player, zero-sum, perfect info, finite).

2.  **The Weighted Entropy Refinement Model `(P, T_w)`:**
    * Define Primary Outcome `P`.
    * Define the "distribution game".
    * Define the **Weighted Entropy Balance Score** `T_w` (`T_A = w_A Σ H(A) - w_B Σ H(B)`, `w_A, w_B > 0`).
    * Define the player's objective: Lexicographically maximize `(E[P], E[T_w])`. Define this equilibrium concept formally (e.g., "Weighted Entropy Equilibrium" - WEE).

3.  **Existence and Uniqueness of Weighted Entropy Equilibrium:**
    * Prove existence and uniqueness via **backward induction** for finite perfect information games.
    * Emphasize the role of the strictly concave `w_i H(D_i)` term in ensuring unique optimal distributions `D*` at tie-breaks.

4.  **Structure of Optimal Distributions:**
    * Derive the **softmax/Gibbs form** of the optimal distribution `D*` at tie-breaks: `p(m)* ∝ exp(E[T_{w, future} | m] / w_i)`, highlighting `w_i`'s role as an inverse temperature.

5.  **Connection to Perturbed Games (Limit `ε -> 0`)**:
    * Define an auxiliary game with combined payoff `Payoff_ε = E[P] + ε E[T_w]` for a small `ε > 0`.
    * Prove (or sketch proof) that the **Weighted Entropy Equilibrium is the limit of the Nash equilibria of the `Payoff_ε` game as `ε -> 0+`**. This provides an alternative characterization and theoretical link to standard techniques.

6.  **Computational Aspects:**
    * Discuss direct computation of WEE via backward induction for perfect information games, leveraging the softmax structure for efficiency.
    * Briefly contrast this with the *limit approach* (`ε -> 0`), noting the direct method is possible due to perfect information.

7.  **Example:**
    * Illustrate with Tic-Tac-Toe or similar, showing `(P, T_w)` calculation and potential effect of weights `w_A, w_B`.

8.  **Discussion & Conclusion:**
    * Summarize the WEE concept and its properties (uniqueness, structure, limit characterization).
    * Discuss interpretation of weights `w_A, w_B`.
    * Mention sensitivity to `P` perturbations (implicit `S`).
    * Briefly contrast motivation with other refinements.


**Tangential / Explicitly Future Work (To be mentioned briefly):**

1.  **Imperfect Information Games:** The `lim ε -> 0` characterization (Section 5) potentially provides a basis/computational avenue for defining and finding WEE in games without perfect information using existing algorithms for solving perturbed games.
2.  **Secondary Scores (`S`):** Formal extension to `(P, S, T_w)` model.
3.  **Finite Temperature / Heuristics:** Application in heuristic search (MCTS) potentially related to the weights `w_i` or the `ε` perturbation.
4.  **Alternative Entropy/Complexity Measures.**
5.  **Specific Game Analyses.**
6.  **Non-Zero-Sum Extensions.**

**Explicitly Out of Scope:**

* Detailed algorithms for imperfect information games.
* Full implementation details for search or tablebases.
* Deep comparisons with all other refinements.
* Formal treatment of infinite games.

This revised plan incorporates the valuable connection to the `ε -> 0` limit, strengthening the theoretical basis and highlighting a path for extensions (especially imperfect information), while keeping the core focus on the definition and properties of the Weighted Entropy Equilibrium computed via backward induction for the perfect information case. It also adopts more formal nomenclature.
