# BMM Brainstorming Session — 2025-11-30T10:55:00Z

Project: consensus

Facilitator: Analyst (BMad)

Goal: Create a leaderless, asynchronous, BFT consensus mechanism.

Techniques used:
- Divergent listing (generate many ideas quickly)
- Trade-off table (capture pros/cons)
- Constraint reframing (assume asynchronous, leaderless, BFT)

Top candidate approaches (high-level):

1) Asynchronous threshold-signed committees
- Form rotating committees using verifiable random functions (VRFs) or epoch-based sampling.
- Use threshold signatures for agreement without a single leader.
- Pros: strong finality once threshold reached; avoids global leader.
- Cons: committee formation in fully asynchronous settings needs careful liveness reasoning.

2) Pure gossip + randomized sampling (probabilistic agreement)
- Nodes gossip transactions and samples converge probabilistically (e.g., metastable consensus family).
- Pros: highly scalable, simple peer-to-peer operation.
- Cons: Probabilistic finality (not strict BFT finality) unless augmented.

3) Asynchronous BFT protocols (inspired by Honey Badger BFT style)
- Use asynchronous reliable broadcast and binary agreement primitives composed for consensus.
- Pros: Proven asynchronous BFT constructions exist; leaderless variants possible.
- Cons: Complexity and communication cost; careful batching needed.

4) DAG-based agreement with virtual voting
- Build a DAG of messages; use scoring / virtual voting to select ordered set of transactions.
- Pros: High throughput and natural concurrency.
- Cons: Requires careful liveness guarantees; mapping to BFT finality needs extra mechanisms.

5) Hybrid: committee + DAG + threshold sigs
- Use committee sampling periodically to finalise DAG heads using threshold signatures.
- Pros: Balances scalability and strong finality.
- Cons: More moving parts; engineering complexity.

Risks & open questions to resolve in Research/PRD:
- Adversary model specifics: network partitions, message delays, adaptive adversary?
- Assumed synchrony bounds for liveness vs. purely asynchronous liveness guarantees.
- Target safety model: probabilistic vs. absolute finality.
- Cryptographic primitives: VRF availability, threshold signature scheme choice.
- Node membership model: permissioned vs. permissionless, join/leave handling.

Suggested next steps (for PRD & Research):
1. Clarify threat model and deployment assumptions.
2. Decide target finality semantics (probabilistic vs. deterministic BFT finality).
3. Shortlist candidate algorithms (Honey Badger variants, Avalanche-like approaches, committee + DAG hybrids).
4. Research concrete threshold signature libraries and VRF implementations suitable for target languages.
5. Prototype small-scale simulation to validate liveness under adversarial delays.

Selected highlights (for PRD inclusion):
- Leaderless, committee-based threshold signing is promising for combining finality and scale.
- DAG with virtual voting gives throughput but needs finality layer.

— End of brainstorming session —
