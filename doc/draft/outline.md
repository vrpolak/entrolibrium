# Paper Outline: Weighted Entropy Equilibrium

**Title Suggestion:** Weighted Entropy Equilibrium: A Unique Refinement for Zero-Sum Games

**Abstract:**
* Briefly state the problem: Need for selecting among multiple equilibria in two-player, zero-sum, perfect information games.
* Propose solution: A lexicographical refinement `(P, T_w)` using a primary payoff `P` and a novel weighted zero-sum entropy balance score `T_w = w_A ΣH(A) - w_B ΣH(B)` as a tie-breaker.
* Define the resulting equilibrium concept: Weighted Entropy Equilibrium (WEE).
* State main result: WEE exists and is unique for finite games, computable via backward induction.
* Highlight key properties: Optimal tie-breaking distributions follow a softmax/Gibbs structure with weights `w_i` acting as inverse temperatures. WEE is the `ε -> 0+` limit of equilibria in games with payoff `P + ε T_w`.
* Mention significance: Provides a principled, unique tie-breaking mechanism based on strategy structure.

---

**1. Introduction**
    * **1.1. The Equilibrium Selection Problem in Zero-Sum Games:** Discuss limitations of minimax value alone (non-unique optimal strategies). Need for refinement concepts.
    * **1.2. Motivation for Entropy:** Introduce Shannon entropy `H(D)` as a measure of choice complexity/randomness. Motivate its use as a tie-breaker based on its unique mathematical properties (e.g., additivity, decomposition invariance – state this clearly, explicitly distancing from direct physics analogies).
    * **1.3. Weighted Entropy Balance (`T_w`):** Motivate the need for player-specific weights (`w_A, w_B > 0`) to handle potential game asymmetries or player preferences. Define the zero-sum score `T_w`.
    * **1.4. Contributions and Paper Outline:** Introduce the Weighted Entropy Equilibrium (WEE) concept formally. State the paper's main contributions (definition, uniqueness proof, structural characterization, limit connection). Provide a roadmap for the subsequent sections. *(Optional: Briefly mention the informal project name "entrolibrium" used for the code repository here, if desired for context).*

**2. Preliminaries**
    * **2.1. Game Model:** Formal definition of two-player, zero-sum, perfect information, finite extensive form games. States, actions, terminal payoffs (`P`). Strategies (pure, mixed, behavioral/distribution-based).
    * **2.2. Equilibrium Concepts:** Nash Equilibrium, Minimax value and strategies for zero-sum games. Subgame Perfect Equilibrium (if needed, though backward induction covers it).
    * **2.3. Lexicographical Preferences:** Formal definition of comparing payoff vectors lexicographically.
    * **2.4. Shannon Entropy:** Definition `H(D) = -Σ p log p`. Key properties: non-negativity, maximality at uniform, strict concavity, decomposition invariance (proof in appendix or cite).

**3. The Weighted Entropy Equilibrium (WEE) Model**
    * **3.1. The Distribution Game Framework:** Define the game where players choose distributions `D` over available moves at each node. Expected payoffs.
    * **3.2. The `(P, T_w)` Lexicographical Objective:** Define the objective for each player: Maximize `(E[P], E[T_w])` lexicographically, where `E[T_w]` incorporates future expected values and the entropy `H(D)` of the current move choice, weighted by `w_i`.
        * Player A maximizes `(E[P_A], w_A H(D_A) + E[T_{w,A, future}])`
        * Player B maximizes `(E[P_B], w_B H(D_B) + E[T_{w,B, future}])`
    * **3.3. Formal Definition of WEE:** Define WEE as the strategy profile (specifying the optimal distribution `D*` at each node) resulting from this optimization process, typically computed via backward induction.

**4. Existence and Uniqueness of WEE**
    * **4.1. Backward Induction for `(P, T_w)`:** Describe the recursive algorithm, starting from terminal nodes. Detail how the optimal distribution `D*` and the value vector `(E[P], E[T_w])` are computed at each node based on successor values.
    * **4.2. Uniqueness Theorem:** State and prove the main theorem: For any finite, two-player, zero-sum game with perfect information and weights `w_A, w_B > 0`, the backward induction algorithm yields a unique WEE strategy profile and a unique value vector `(E[P], E[T_w])`. (Proof relies on standard backward induction logic + uniqueness of maximizer for the strictly concave `w_i H(D_i)` term during tie-breaking).

**5. Structure of Optimal WEE Strategies**
    * **5.1. Tie-Breaking Distribution Form:** Derive the explicit form of the optimal distribution `D*` chosen when `P` is tied. Show it follows `p(m)* ∝ exp(E[T_{w, future} | m] / w_i)`, a restricted softmax/Gibbs distribution.
    * **5.2. Interpretation of Weights as Inverse Temperatures:** Discuss how `w_i` scales the relevance of future entropy differences, controlling the stochasticity of the tie-breaking choice. Higher `w_i` leads to more uniform mixing (higher temperature); lower `w_i` leads to more deterministic choices based on `E[T_w, future]` (lower temperature).

**6. Connection to Perturbed Games (Limit Characterization)**
    * **6.1. The Auxiliary `P + ε T_w` Game:** Define a standard game with a single scalar payoff `Payoff_ε = E[P] + ε E[T_w]` for `ε > 0`.
    * **6.2. Limit Equivalence:** State and prove/sketch that the unique WEE strategy profile is the limit of the Nash equilibrium strategy profiles of the `Payoff_ε` game as `ε -> 0+`. (Standard argument based on dominance of the `P` term for small `ε`).
    * **6.3. Significance:** Discuss how this connects WEE to standard utility theory and provides a potential avenue for extending the concept (computationally) to other game classes like those with imperfect information.

**7. Computational Aspects**
    * **7.1. Direct Computation for PI Games:** Reiterate computation via backward induction. Emphasize efficiency due to the likely closed-form (softmax-like) calculation for `D*` versus generic optimization. Note computational cost still depends heavily on state space size.
    * **7.2. Sensitivity Analysis:** Briefly discuss how the P-optimal set, over which `T_w` operates, is sensitive to the definition or small perturbations of `P` (implicitly acknowledging the role secondary factors `S` could play if modeled as perturbations `P' = P + δS`).

**8. Example: Tic-Tac-Toe**
    * **8.1. WEE Calculation:** Apply the backward induction to determine the optimal first move distribution `D*` for Player X. Present the calculated `E[T_w]` values for the 9 successor states.
    * **8.2. Results and Interpretation:** Show the final `p*(m)` probabilities. Discuss the result, noting potential effects (or lack thereof due to symmetry) of varying `w_A, w_B`.

**9. Discussion**
    * **9.1. Interpreting WEE:** What kind of behavior does it select? Relation to practical strategies, unpredictability, maintaining options.
    * **9.2. Interpreting Weights `w_A, w_B`:** How might they be set or interpreted? Asymmetry, risk preferences, exploration parameters?
    * **9.3. Relation to Other Refinements:** Briefly position WEE relative to major concepts like SPE (WEE refines *among* SPEs of the P-game) and THPE (different motivation: structural tie-breaking vs. error robustness).

**10. Related Work**
    * Brief overview of equilibrium selection/refinement literature (lexicographical games, THPE, SPE, Proper, QRE).
    * Mention relevant applications of entropy in game theory or decision theory (e.g., MaxEnt principle, entropy-regularized RL), while maintaining distance from direct physics motivation.
    * Mention computational game theory background (minimax, search, tablebases).

**11. Conclusion and Future Work**
    * **11.1. Summary:** Recap the definition of WEE, the key results (uniqueness, structure, limit connection), and its potential significance as a principled refinement.
    * **11.2. Future Work:** Explicitly list promising directions: formalizing `(P, S, T_w)`; application to imperfect information games (using limit approach); heuristic search methods (MCTS/finite temperature); alternative measures for `T`; deeper analysis of specific complex games.

**References**

**Appendix (Optional)**
* Proof of entropy decomposition invariance.
* Detailed mathematical derivations.
* Source code for example.
