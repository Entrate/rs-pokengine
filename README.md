# rs-pokengine

A Rust battle engine for Pokemon Showdown's **[Gen 9] Random Doubles Battle**. It is a port of
Showdown's simulator, pinned to commit
[`7332b60`](https://github.com/smogon/pokemon-showdown/tree/7332b60e22b9e8194bb53549549eba241d73cc9a):
given the same seed, teams and choices, it produces the same battle log, requests and random draws
as Showdown. It has no dependencies.

The crate is named `engine`.

## Build

Install current stable Rust with [rustup](https://rustup.rs) (edition 2024, so Rust 1.85 or newer).
On Windows you also need a linker: the Visual Studio C++ build tools for MSVC, or MinGW GCC for
the GNU toolchain.

```sh
git clone https://github.com/Entrate/rs-pokengine.git
cd rs-pokengine
cargo build --release
```

Run the example battle:

```sh
cargo run --release --example battle
```

Run the tests:

```sh
cargo test --features trace-hooks,event-probes,event-trace,search
cargo test --all-features --test '*'
```

## Use it from another project

```toml
[dependencies]
engine = { git = "https://github.com/Entrate/rs-pokengine.git" }
```

Build your project in release mode; the debug build is much slower.

## Playing a battle

Teams use Showdown's packed team format (one team string per side, Pokemon separated by `]`).
Seeds are Showdown's four 16-bit seed words.

```rust
use engine::{Battle, ids::SideId, log::TextLog, state::choices::RequestKind};

let mut battle = Battle::with_log([1, 2, 3, 4], p1_packed, p2_packed, TextLog::default())?;
battle.start()?;

while battle.outcome().is_none() {
    for side in 0..2 {
        if matches!(battle.side_request_kind(SideId(side as u8)), RequestKind::Move | RequestKind::Switch) {
            // Same strings as Showdown's /choose: "move 1 1, move 2 -1", "switch 3", "default", ...
            battle.choose(side, "default")?;
        }
    }
}

for line in &battle.log.entries {
    println!("{line}");       // Showdown protocol lines, exactly as Showdown writes them
}
println!("{:?}", battle.outcome());  // winner (0, 1, or None for a tie), turns, Pokemon left
```

The full program is in [`examples/battle.rs`](examples/battle.rs). Once both sides have chosen,
`choose` runs the turn immediately, the same way Showdown does.

Other useful calls on `Battle`:

| Call | What it does |
| --- | --- |
| `Battle::new(seed, p1, p2)` | Battle with no log (fastest, for simulations) |
| `request_json(side)` | Showdown's request JSON for that side |
| `legal_actions(side)` | Legal moves, targets and switches for each active slot, without JSON |
| `choose_typed(side, &[SlotChoice])` | Choose with structs instead of strings |
| `is_legal_joint_choice(side, &[SlotChoice])` | Check a choice for both slots together |
| `clone()` / `restore_from(&other)` | Copy a battle state, e.g. to try out different choices |
| `reset(seed, p1, p2)` | Reuse an allocated battle for a new game |
| `state` | The full battle state (Pokemon, sides, field, queue) |

`TextLog` keeps the Showdown log. Use `log::NoLog` (the default for `Battle::new`) when you
don't need it.

## Features

All are off by default.

| Feature | Purpose |
| --- | --- |
| `search` | Fair-information search support: builds possible worlds from what a player has seen (`engine::search`). Adds a small cost to every random draw. |
| `reveal-audit` | Checks that every Pokemon named in the log is marked as revealed |
| `trace-hooks`, `event-probes`, `event-trace` | Debugging and tracing of the event system |

## Scope

Only Gen 9 Random Doubles is supported. Teams come from outside the engine: it has no random team
generator, so generate teams with Showdown (or any other source) and pass them in packed form.

## License

MIT. See [LICENSE](LICENSE). The engine is derived from
[Pokemon Showdown](https://github.com/smogon/pokemon-showdown), which is MIT licensed. Pokemon
names and data belong to their respective owners.
