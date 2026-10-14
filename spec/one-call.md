# One-call eoie: cases mined from agent history

Each case is a tutorial and a test. A runner extracts the `fixture`, `command` and `expected` blocks, builds the
fixture in a temporary root, runs the command once and compares its whole output with `expected`, byte for byte.
Cases are ordered by how many recorded agent calls they replace (transcript sweep 2026-10-13: ~45k tool calls from
Claude, Codex, OpenCode and Grok; 48% of shell calls chained three or more operations).

Every run is one call: `eoie do <root> '<program>'`. A program is statements separated by newlines or `;`. Paths
are relative to the root. Every run ends with one receipt line: `ok` or `failed`, the counts it touched, elapsed time
and the local clock with its offset, so no case ever needs a separate clock or status call. Elapsed times and clocks
are written `<elapsed>` and `<clock>` in `expected`; the runner substitutes them.

## 1. Edit anchors, check counts, show the result (read + write + replace: 744 calls, + search: 466)

Habit (Claude, abbreviated): read the file, `.Replace` each anchor inside a `foreach` that throws when an anchor is
missing, `WriteAllText`, then `Get-Content` the file again to look at it.

```fixture
lib/harness.spi
inl show_i32 (x : i32) : string =
    x |> convert.i64 |> show_i64
inl bench dt iters = dt / convert.f64 iters * 1000000000.0
lib/main.spi
inl main () =
    harness.run_all []
```

```command
eoie do . '
edit lib/harness.spi
  replace once "x |> convert.i64 |> show_i64" with "show_i64 (convert.i64 x)"
  replace once "convert.f64 iters" with "f64_of_i32 iters"
edit lib/main.spi
  replace once "inl main () =" with "inl main () : i32 ="
show lib/main.spi'
```

```expected
edit lib/harness.spi  2 replacements  lines 2-3  sha256 <hash> -> <hash>
edit lib/main.spi     1 replacement   line 1     sha256 <hash> -> <hash>
lib/main.spi
1  inl main () : i32 =
2      harness.run_all []
ok  files 2  replacements 3  <elapsed>  <clock>
```

## 2. A missing anchor changes nothing anywhere (the `throw "nf"` guard, built in)

```fixture
a.txt
alpha
b.txt
beta
```

```command
eoie do . '
edit a.txt
  replace once "alpha" with "ALPHA"
edit b.txt
  replace once "gamma" with "GAMMA"'
```

```expected
edit b.txt  anchor not found: "gamma" (0 matches, expected 1)
nothing written: 1 of 2 edits failed validation
failed  files 0  replacements 0  <elapsed>  <clock>
```

## 3. Search many files with context, bounded (read + search + walk: 236 calls)

Habit (Codex, abbreviated): `Get-ChildItem out/...; Get-Content main.rs | Select-String -Pattern 'if v|RecursiveCreate' -Context 1,4 | Select-Object -First 12`, once per sample, in parallel tool calls.

```fixture
out/a/main.rs
fn f() {
    if v0 { RecursiveCreate(1) }
}
out/b/main.rs
fn g() {}
out/c/main.c
if v0 {}
```

```command
eoie do . 'search "if v|RecursiveCreate" in out/**/*.rs context 1 first 12 per file'
```

```expected
out/a/main.rs
1  fn f() {
2> if v0 { RecursiveCreate(1) }
3  }
ok  files 2 searched  1 matched  matches 1  <elapsed>  <clock>
```

## 4. Classify a file set and count the classes (read + walk + aggregate: 257 calls)

Habit (Claude, abbreviated): `Get-ChildItem samples -Directory | Where-Object { Test-Path main.c -and Test-Path main.spi }`, then per directory read `main.spi`, regex-match, build `[pscustomobject]` rows, `Group-Object`, format.

```fixture
samples/s1/main.spi
!!!!BackendSwitch { C = 1 }
samples/s1/main.c
int main(){}
samples/s2/main.spi
!!!!BackendSwitch { C = 1; Wasm = 2 }
samples/s2/main.c
int main(){}
samples/s3/main.spi
inl main () = 0
```

```command
eoie do . '
dirs samples/* having main.spi and main.c
  tag switch_no_wasm = main.spi ~ "BackendSwitch" and not main.spi ~ "Wasm ="
count by switch_no_wasm'
```

```expected
switch_no_wasm  count  dirs
true            1      s1
false           1      s2
ok  dirs 2 of 3 matched  <elapsed>  <clock>
```

## 5. Compare two result tables by key (the suite reviews; json/csv + aggregate)

Habit (Claude, 2026-10-13, abbreviated): `Import-Csv` both `results.tsv`, build a hashtable by `id|backend`, `Where-Object` over changed columns, `Group-Object`, format, every review.

```fixture
a.tsv
id	backend	compile	exit	residual
s1	C	ok	0	aa
s2	C	ok	7	bb
s3	Rust	error		
b.tsv
id	backend	compile	exit	residual
s1	C	ok	0	aa
s2	C	ok	7	cc
s3	Rust	ok	3	dd
s4	C	ok	0	ee
```

```command
eoie do . 'diff a.tsv b.tsv by id,backend on compile,exit,residual'
```

```expected
key       column    a      b
s2|C      residual  bb     cc
s3|Rust   compile   error  ok
s3|Rust   exit             3
s3|Rust   residual         dd
added     s4|C
ok  rows a 3  b 4  changed 2  added 1  removed 0  <elapsed>  <clock>
```

## 6. Run until a condition, sample it, always stop (read + process + wait: 150 calls)

Habit (Claude, abbreviated): `Start-Process -PassThru` with redirected logs, a `while` loop with `Start-Sleep 2`, peak working set, regex on the log, `Stop-Process`, `Get-Content -Tail`.

```fixture
server.cmd
@echo off
echo booting
echo server online
ping -n 30 127.0.0.1 >nul
```

```command
eoie do . 'run server.cmd until output ~ "server online" timeout 20s then stop; tail output 2'
```

```expected
run server.cmd  matched "server online" after <elapsed>  stopped (tree)  exit killed
output (last 2 lines)
booting
server online
ok  runs 1  <elapsed>  <clock>
```
