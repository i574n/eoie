# eoie

eoie is a generic agent tool written in Spiral. It knows no external project, team, path or user: teams are words from backlog tags or upper-case ID prefixes, and test fixtures use neutral domains.

## Style contract

Most of eoie predates this contract. Code you open here is not a style model unless its package's `standards.spi` declares `HostFree` and `LibReferenced` (checked by `eoie standards <package>`).

Copy from, in this order:
1. lib/spiral (`sm'`, `am'`, `listm'`, `optionm'`, `resultm`, `seq`, `iter`, `file_system`, `math`). Reach for a lib function before writing any helper.
2. Boundary modules that confine host code: one module per OS surface (process, fs, net), each primitive a named one-expression macro.
3. Granular typed domain modules with test blocks: one concept per union or GADT, one small module per concern.
4. eoie packages whose `standards.spi` declares `HostFree` and `LibReferenced`.

Rules:
- Model the domain first: unions, GADTs (`| X :: t`), existentials, records, result unions for outcomes. No joined-string rows or number tables: the census counts both per package and the ratchet refuses growth.
- Host code lives only in the OS boundary package as named one-expression primitives. No `$'{ ... }` blocks, no `Rc::<str>::from` outside it, no new `rust_std_string` use.
- A missing generic helper goes into lib/spiral (backend_switch shape, with a lib test block), never into eoie.
- Every package references the lib (`spiral-` in `package.spiproj`).
- Touching a legacy function means migrating it in the same change.
- Tests are Spiral test blocks; generated crates stay under 1000 lines.

`eoie standards --ratchet standards/census.spi <package>` measures a package against its baseline, and the edit-time hook reports drift in the text you add.
