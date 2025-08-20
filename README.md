# Puzzle Engine

Create your own puzzle games

## Workspace crates

- **rulery** — library for rule schema and evaluation
- **crazy-puzzle** — example game built with Rulery and Bevy
- **rulery-editor** — GUI editor for Rulery

## DSL Quick Look

Check if a position is empty or enemy:

```ron
If(
    // condition
    PosOccupied(r, c),
    
    // occupied -> must be enemy
    Not(ColorEqual(ColorAtPos(r, c), your_color)),
    
    // empty -> ok
    True,
)
```
> Note: r, c, and your_color are placeholders. In real rules, replace them with context variables or with nested expressions.

## Example: Chess rook movement

A compact rook movement rule using `If` and `CountInRect`:

```ron
And([
  // 1) Must move in a straight line: same row or same column
  Or([
    Equal(TargetRow, SourceRow),
    Equal(TargetCol, SourceCol),
  ]),

  // 2) Branch on whether the target square is occupied
  If(
    PosOccupied(TargetRow, TargetCol),

    // then: target occupied → must be enemy and path contains exactly 1 piece (the target)
    And([
      Not(ColorEqual(ColorAtPos(TargetRow, TargetCol), MovingColor)),
      Equal(CountInRect((SourceRow, SourceCol), (TargetRow, TargetCol)), Const(1)),
    ]),

    // otherwise: target empty → path contains 0 pieces
    Equal(CountInRect((SourceRow, SourceCol), (TargetRow, TargetCol)), Const(0)),
  ),
])
```
