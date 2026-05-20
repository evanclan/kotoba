# Governance

This document describes how Kotoba is run.

## Mission

Kotoba exists to make language learning **owned by the learner**, **transparent in its algorithms**, and **interoperable with whatever tools the future brings**.

## Roles

- **Maintainers** — write access. Approve and merge PRs, set milestones, enforce the Code of Conduct, steward the architecture.
- **Triagers** — issue-only access. Label, close stale issues, route discussion. Often a stepping-stone to maintainership.
- **Codeowners** — listed in [`.github/CODEOWNERS`](../.github/CODEOWNERS). Auto-requested for review on PRs touching their area. Codeowners are not necessarily maintainers, and vice versa.
- **Contributors** — anyone who's had a PR merged. Listed automatically by GitHub.

## Decision-making

- **Trivial changes** (docs typos, deck additions, small bug fixes): one maintainer review, merge.
- **Standard changes** (most features, refactors): two maintainer reviews, merge.
- **Significant changes** (new public APIs, algorithm changes, breaking changes, format changes): an RFC in `docs/rfcs/`, ≥7-day discussion window, two maintainer +1s, no maintainer -1.
- **Disagreements between maintainers**: discussion-first, vote if needed (simple majority of active maintainers; ties broken by the BDFL — see below — until v1.0, then by community-elected steering committee).

## BDFL clause (until v1.0)

Until Kotoba reaches v1.0, the founding maintainer holds tiebreak authority. After v1.0, this is replaced by a steering committee elected from active maintainers, with terms specified in a then-current `docs/governance-steering.md`.

## How to become a maintainer

1. Make substantive contributions over time. Quality and consistency matter more than count.
2. Show up in reviews and discussions. Help triage.
3. An existing maintainer nominates you privately; the maintainer team confirms.
4. We grow the team intentionally rather than competitively.

There is no application form. There is no "become a maintainer" button. We invite you.

## Removal

Maintainers may be removed for:

- Repeated Code of Conduct violations
- Sustained absence (>6 months, no communication)
- Acting against the project mission (e.g. shipping closed-source forks under the project name)

Removal requires a supermajority (2/3) of active maintainers.

## Project values (binding)

These are not aspirational; they are constraints on what the project will and will not do.

1. **Local-first, plain text, open formats — forever.**
2. **Privacy by default.** No telemetry without granular opt-in consent.
3. **Open algorithms, open data.** No proprietary scheduling, no closed dictionaries.
4. **AI as a peer.** Always optional, always swappable, always inspectable.
5. **No dark patterns.** No manipulative streak shaming, no engagement-bait, no nagware.
6. **No CLA.** Contributions stay under their author's copyright with the project license. We use the [Developer Certificate of Origin](https://developercertificate.org/) (`Signed-off-by:` lines).
7. **Public roadmap, public RFCs, public retros.**

## Funding

Kotoba accepts donations and corporate sponsorships ([FUNDING.yml](../.github/FUNDING.yml)). Funds are used to:

1. Pay reviewers for sustained contribution work
2. Fund infrastructure (CI minutes, domains, signing certs)
3. Fund accessibility, internationalization, and a11y audits

**No funder gets product-direction privileges.** Sponsorship is acknowledgement, not influence.

If commercial offerings are introduced (e.g. hosted sync for schools), they are governed by a separate entity that licenses the trademark from the project. The open core remains free, MIT-licensed, and unaffected.

## Trademark

The name "Kotoba" and the project logo are project marks. Forks may use the codebase under MIT but must not use the name in a way that implies endorsement.

## Amending this document

Changes to governance require an RFC, a 14-day discussion window, and supermajority (2/3) maintainer approval.
