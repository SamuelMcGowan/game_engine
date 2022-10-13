# ECS

A tiny ECS library with no unsafe code.

## Todo

### Immediate

- Clean up imports.
- Delete an entity from all component storages.
  - Will require erased storage storing trait objects in order to know how to delete entities.
  - Look into casting between any and trait objects.
- Iterate over live entities.
  - Sacrifice 1 bit of the generation to mark in-use identifiers.
- Re-check API related to live entities (contracts & performance).
- Despawn entities.

## Later

- Commands.
- Events.
- Eliminate all panics (except where user facing and clearly documented).
  