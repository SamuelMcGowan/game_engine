# ECS

A tiny ECS library with no unsafe code.

## Todo

### Immediate

- [x] Clean up imports.
- [x] Delete an entity from all component storages.
- [x] Automatically register component storages.
- [ ] Iterate over live entities.
  - Sacrifice 1 bit of the generation to mark in-use identifiers.
- [ ] Re-check API related to live entities (contracts & performance).
- [ ] Despawn entities.

## Later

- [ ] Commands.
- [ ] Events.
- [ ] Eliminate all panics (except where user facing and clearly documented).
