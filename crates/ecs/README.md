# ECS

A tiny ECS library with no unsafe code.

## Todo

### Immediate

- [ ] Iterate over live entities.
  - Sacrifice 1 bit of the generation to mark in-use identifiers.
- [ ] Re-check API related to live entities (contracts & performance).

## Later

- [ ] Commands.
- [ ] Events.
- [ ] Eliminate all panics (except where user facing and clearly documented).
