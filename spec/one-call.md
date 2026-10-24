# One-call eoie: cases mined from agent history

Each case is a tutorial and a test. A runner extracts the `fixture`, `command` and `expected` blocks, builds the
fixture in a temporary root, runs the command once and compares its whole output with `expected`, byte for byte.
Cases are ordered by how many recorded agent calls they replace (transcript sweep 2026-10-13: ~45k tool calls from
Claude, Codex, OpenCode and Grok; 48% of shell calls chained three or more operations).

Every run is one call: `eoie do <root> '<program>'`. A program is statements separated by newlines or `;`. Paths
are relative to the root. Every run ends with one receipt line: `ok` or `failed`, the counts it touched, elapsed time
and the local clock with its offset, so no case ever needs a separate clock or status call. Elapsed times and clocks
are written `<elapsed>` and `<clock>` in `expected`; the runner substitutes them.

The other placeholders are `<hash>` (a sha256), `<commit>` (a commit id), `<pid>` (a process id) and `<HH:mm>` (the
clock's hour and minute). A `fixture` block lists a path, then that file's lines; every line ends in LF. A path line
has no whitespace and either contains `/` or ends in `.` and letters; every other line is content (this rule splits
all 33 cases correctly). A receipt shows `bytes` when the program read or wrote whole files by size, and `exit` when it
ran a process or ended in a failure that has a code. A `setup`
block, when a case has one, is an eoie program the runner runs before `command`; its output is not compared. Cases run
with every `EOIE_*` and `SPIRAL_*` variable unset.

Cases 7 to 33 come from a second pass over the same history (2026-10-14, `tmp/one-call/mine_clusters.py` in the job
folder: 21,239 shell calls that combine two or more intents, grouped by their exact intent set). Each heading names one
cluster and how many recorded calls it holds; a case marked "companion" is the failure path of the case before it.

## 1. Edit anchors, check counts, show the result (read + write + replace: 744 calls, + search: 466)

Habit (Claude, abbreviated): read the file, `.Replace` each anchor inside a `foreach` that throws when an anchor is
missing, `WriteAllText`, then `Get-Content` the file again to look at it.

```fixture
lib/bench_kit.spi
inl show_i32 (x : i32) : string =
    x |> convert.i64 |> show_i64
inl bench dt iters = dt / convert.f64 iters * 1000000000.0
lib/main.spi
inl main () =
    bench_kit.run_all []
```

```command
eoie do . '
edit lib/bench_kit.spi
  replace once "x |> convert.i64 |> show_i64" with "show_i64 (convert.i64 x)"
  replace once "convert.f64 iters" with "f64_of_i32 iters"
edit lib/main.spi
  replace once "inl main () =" with "inl main () : i32 ="
show lib/main.spi'
```

```expected
edit lib/bench_kit.spi  2 replacements  lines 2-3  sha256 <hash> -> <hash>
edit lib/main.spi       1 replacement   line 1     sha256 <hash> -> <hash>
lib/main.spi
1  inl main () : i32 =
2      bench_kit.run_all []
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
2>     if v0 { RecursiveCreate(1) }
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
key      column    a      b
s2|C     residual  bb     cc
s3|Rust  compile   error  ok
s3|Rust  exit             3
s3|Rust  residual         dd
added    s4|C
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

## 7. Read the lines around a match (read + search: 1,832 calls)

Habit (Claude, abbreviated): `Select-String -Path f -Pattern 'x' | % LineNumber`, then
`Get-Content f | Select-Object -Skip ($n-2) -First 6`, or `Select-String -Context 1,2` and re-read to see the indentation.

```fixture
src/lib.fs
let a = 1
let b = 2
// section: parse
let parse x =
    x + 1
let c = 3
```

```command
eoie do . 'read src/lib.fs around "section: parse" context 1,2'
```

```expected
src/lib.fs  match at line 3
2  let b = 2
3> // section: parse
4  let parse x =
5      x + 1
ok  files 1  matches 1  lines 4  <elapsed>  <clock>
```

## 8. List a folder, then the first lines of each file (read + walk: 841 calls)

Habit (Claude, abbreviated): `Get-ChildItem logs | Select Name, Length`, then `Get-Content $f -TotalCount 2` per file.

```fixture
logs/a.log
start a
step 1
done
logs/b.log
start b
fail
```

```command
eoie do . 'files logs/*.log; head logs/*.log 2'
```

```expected
path        bytes  lines
logs/a.log  20     3
logs/b.log  13     2
logs/a.log (first 2 lines)
1  start a
2  step 1
logs/b.log (first 2 lines)
1  start b
2  fail
ok  files 2  bytes 33  <elapsed>  <clock>
```

## 9. The last line of every task output, with the time (read + clock: 716 calls)

Habit (Claude, abbreviated): `Get-Content $o1 | Select-Object -Last 2; Get-Content $o2; Get-Date -Format HH:mm`. The
receipt's clock replaces the `Get-Date`.

```fixture
tasks/b1.output
compiling
done exit 0
tasks/b2.output
waiting
```

```command
eoie do . 'tail tasks/*.output 1'
```

```expected
tasks/b1.output  done exit 0
tasks/b2.output  waiting
ok  files 2  <elapsed>  <clock>
```

## 10. Search a tree with includes and excludes, paths only (search + walk: 420 calls)

Habit (Claude, abbreviated): `Get-ChildItem src -Recurse -Include *.rs,*.spi -File | Where-Object { $_.FullName -notmatch
'\\target\\' } | Select-String -Pattern 'show <root> <id> \[--json\]' | ForEach-Object { '{0}:{1}' -f $_.Path,
$_.LineNumber }`.

```fixture
src/a/model.spi
show <root> <id> [--json]
src/b/help.rs
"show <root> <id> [--json]"
src/target/gen.rs
show <root> <id> [--json]
src/c/other.spi
nothing here
```

```command
eoie do . 'search literal "show <root> <id> [--json]" in src/** include *.rs,*.spi exclude src/target/** paths'
```

```expected
src/a/model.spi:1
src/b/help.rs:1
ok  files 3 searched  2 matched  matches 2  <elapsed>  <clock>
```

## 11. Preview an edit without writing it (read + replace: 279 calls)

Habit (Claude, abbreviated): read the file, `.Replace` in memory, print the changed region, decide, then repeat the same
replace with `WriteAllText`.

```fixture
cfg.toml
[server]
port = 8080
host = "localhost"
```

```command
eoie do . '
edit cfg.toml dry-run
  replace once "port = 8080" with "port = 9090"'
```

```expected
edit cfg.toml  1 replacement  line 2  dry run, nothing written
-2  port = 8080
+2  port = 9090
ok  files 0 written  replacements 1 previewed  <elapsed>  <clock>
```

## 12. When did a line appear in history (search + git: 240 calls)

Habit (Codex, abbreviated): `git log -S 'inl b' --oneline -- lib.spi; git grep -n 'inl b'`.

```fixture
repo/lib.spi
inl a = 1
```

```setup
eoie do . '
git init repo
git commit repo all "first" at 2026-10-01T10:00:00Z
append repo/lib.spi "inl b = 2"
git commit repo all "add b" at 2026-10-02T10:00:00Z'
```

```command
eoie do . 'git history repo "inl b" in lib.spi'
```

```expected
commit    date        change  subject
<commit>  2026-10-02  added   add b
repo/lib.spi:2  inl b = 2
ok  commits 1  matches 1  <elapsed>  <clock>
```

## 13. Wait until a log says it is done, then show its end (read + wait: 226 calls)

Habit (Claude, abbreviated): `while (-not ((Get-Content $log -Raw) -match 'SUITE-EXIT')) { Start-Sleep 5 }; Get-Content
$log -Tail 2`.

```fixture
out.log
rows 11
rows 12
SUITE-EXIT 0
```

```command
eoie do . 'wait out.log ~ "SUITE-EXIT" timeout 5s; tail out.log 2'
```

```expected
wait out.log  matched "SUITE-EXIT 0" at line 3 after <elapsed>
out.log (last 2 lines)
rows 12
SUITE-EXIT 0
ok  waits 1  <elapsed>  <clock>
```

## 14. A wait that times out fails and shows what it saw (companion of 13)

```fixture
out.log
running
```

```command
eoie do . 'wait out.log ~ "SUITE-EXIT" timeout 1s'
```

```expected
wait out.log  no match for "SUITE-EXIT" in 1s  lines 1  bytes 8
out.log (last 1 line)
running
failed  waits 1  timed out 1  exit 3  <elapsed>  <clock>
```

## 15. Find a process by its command line, then stop it (read + process: 222 calls)

Habit (Claude, abbreviated): `Get-CimInstance Win32_Process -Filter "Name='dotnet.exe'" | Where-Object { $_.CommandLine
-match 'spiral-bin' } | ForEach-Object { '{0} {1:HH:mm} {2}' -f $_.ProcessId, $_.CreationDate, $_.CommandLine }`, then
`Stop-Process -Id ... -Force`.

```fixture
sleeper.cmd
@echo off
ping -n 30 127.0.0.1 >nul
```

```command
eoie do . 'run sleeper.cmd background as s1; processes where command ~ "sleeper.cmd"; stop s1'
```

```expected
run sleeper.cmd  background as s1  pid <pid>
pid      started  command
<pid>    <HH:mm>  cmd.exe /c sleeper.cmd
stop s1  stopped (tree)
ok  runs 1  processes 1  stopped 1  <elapsed>  <clock>
```

## 16. Build, then the errors grouped by code (read + build: 209 calls, search + build: 163)

Habit (Codex, abbreviated): `cargo build > build.log 2>&1; Get-Content build.log -Tail 45`, then `Select-String 'error'
| Group-Object` in the next call.

```fixture
build.cmd
@echo off
echo src/a.fs(3,5): error FS0001: type mismatch
echo src/b.fs(9,1): error FS0039: not defined
echo src/b.fs(12,7): error FS0039: not defined
echo Build FAILED.
exit /b 1
```

```command
eoie do . 'run build.cmd; errors in output by "error (FS[0-9]+)"'
```

```expected
run build.cmd  exit 1  lines 4  <elapsed>
code    count  first
FS0039  2      src/b.fs(9,1): error FS0039: not defined
FS0001  1      src/a.fs(3,5): error FS0001: type mismatch
failed  runs 1  exit 1  errors 3  <elapsed>  <clock>
```

## 17. Files and bytes by extension (walk + aggregate: 207 calls)

Habit (Claude, abbreviated): `Get-ChildItem proj -Recurse -File | Group-Object Extension | ForEach-Object { '{0} {1} {2}'
-f $_.Name, $_.Count, ($_.Group | Measure-Object Length -Sum).Sum }`.

```fixture
proj/a.spi
inl a = 1
proj/b.spi
inl b = 22
proj/c.rs
fn c() {}
proj/d.md
# d
```

```command
eoie do . 'files proj/** count by extension'
```

```expected
ext   files  bytes
.spi  2      21
.rs   1      10
.md   1      4
ok  files 4  bytes 35  <elapsed>  <clock>
```

## 18. The nearest marker above given lines (read + search + clock: 178 calls)

Habit (Claude, abbreviated): `$lines = [IO.File]::ReadAllLines($f)`, collect the indexes of the marker lines in a
`for` loop, then for each target line pick the last marker before it, and print `Get-Date` at the end.

```fixture
big.fs
/// ## Alpha
let a = 1
let b = 2
/// ## Beta
let c = 3
let d = 4
```

```command
eoie do . 'nearest "^/// ## " before lines 3,6 in big.fs'
```

```expected
line  nearest  text
3     1        /// ## Alpha
6     4        /// ## Beta
ok  files 1  lookups 2  <elapsed>  <clock>
```

## 19. Environment variables and the folders that should exist (read + walk + clock: 170 calls)

Habit (Claude, abbreviated): `Write-Output ('EOIE_SPIRAL_BUNDLE = [' + $env:EOIE_SPIRAL_BUNDLE + ']')`, `foreach ($d in
'bundles','dist','bin') { if (Test-Path ...) ... }`, `Get-Date`.

```fixture
bundles/x.zip
z
dist/readme.txt
r
```

```command
eoie do . 'env EOIE_SPIRAL_BUNDLE, EOIE_SPIRAL_COMPILE; exists bundles, dist, bin'
```

```expected
EOIE_SPIRAL_BUNDLE   unset
EOIE_SPIRAL_COMPILE  unset
bundles  directory  1 entry
dist     directory  1 entry
bin      missing
ok  env 2 (0 set)  paths 3 (2 present)  <elapsed>  <clock>
```

## 20. Fields out of a JSON file (read + json/csv: 156 calls)

Habit (Claude, abbreviated): `$j = Get-Content package.json -Raw | ConvertFrom-Json; $j.name; $j.version;
$j.scripts.build; $j.deps.Count`.

```fixture
package.json
{"name": "eoie", "version": "0.3.1", "scripts": {"build": "pwsh build.ps1"}, "deps": ["a", "b"]}
```

```command
eoie do . 'json package.json get name, version, scripts.build, deps.length'
```

```expected
name           "eoie"
version        "0.3.1"
scripts.build  "pwsh build.ps1"
deps.length    2
ok  files 1  fields 4  <elapsed>  <clock>
```

## 21. Append a timestamped line to a board (read + write: 140 calls)

Habit (Claude, abbreviated): `[IO.File]::AppendAllText("$dir\BOARD.md", "- 02:15 lane: done`n")` with a time typed by
hand, then `Get-Content BOARD.md -Tail 2`. `{clock:HH:mm}` takes the time from the clock instead.

```fixture
BOARD.md
# board
- 00:10 lane 1: start
```

```command
eoie do . 'append BOARD.md "- {clock:HH:mm} lane 3: R2 done"; tail BOARD.md 2'
```

```expected
append BOARD.md  1 line  24 bytes  now 3 lines, 54 bytes
BOARD.md (last 2 lines)
- 00:10 lane 1: start
- <HH:mm> lane 3: R2 done
ok  files 1  appended 1  bytes 24  <elapsed>  <clock>
```

## 22. Count log lines by a captured group (read + aggregate: 124 calls)

Habit (Claude, abbreviated): `Get-Content app.log | ForEach-Object { ($_ -split ' ')[0] } | Group-Object | Sort-Object
Count -Descending | Format-Table Name, Count`.

```fixture
app.log
INFO start
WARN slow disk
ERROR no route
WARN slow disk
INFO stop
```

```command
eoie do . 'lines app.log count by "^(\w+)"'
```

```expected
group  count
INFO   2
WARN   2
ERROR  1
ok  files 1  lines 5  groups 3  <elapsed>  <clock>
```

Ties keep the order in which a group first appears.

## 23. The git state of several repos, grouped (walk + git: 119 calls; read + walk + git: 66)

Habit (Claude, abbreviated): `foreach ($r in 'app','lib','tool') { $st = @(git -C $g --no-optional-locks status
--porcelain --untracked-files=all); ... Where-Object { $_.StartsWith('??') } ... }`.

```fixture
a/lib.spi
x
b/readme.md
r
```

```setup
eoie do . '
git init a; git commit a all "init" at 2026-10-01T10:00:00Z
git init b; git commit b all "init" at 2026-10-01T10:00:00Z
edit a/lib.spi
  replace once "x" with "y"
append a/new.txt "n"'
```

```command
eoie do . 'git status a, b group by state'
```

```expected
repo  state      count  paths
a     modified   1      lib.spi
a     untracked  1      new.txt
b     clean      0
ok  repos 2  changed 1  untracked 1  <elapsed>  <clock>
```

## 24. Edit, then build, in one call (read + write + replace + build: 116 calls)

Habit (Claude, abbreviated): the case 1 edit block, then `pwsh build.ps1 ... 2>&1 | Select-Object -Last 20` in the same
call, with the edit already written whether or not the build passes.

```fixture
main.c
int main(void) {
    return 1;
}
build.cmd
@echo off
findstr /c:"return 0;" main.c >nul && (echo built) || (echo missing return & exit /b 1)
```

```command
eoie do . '
edit main.c
  replace once "return 1;" with "return 0;"
run build.cmd'
```

```expected
edit main.c  1 replacement  line 2  sha256 <hash> -> <hash>
run build.cmd  exit 0  lines 1  <elapsed>
built
ok  files 1  replacements 1  runs 1  exit 0  <elapsed>  <clock>
```

## 25. Replace across files with the counts it must find (read + search + replace: 106 calls; search + walk + replace: 67)

Habit (Claude, abbreviated): a list of `@(old, new)` pairs applied with `eoie proxy text-replace apply` per pair, then a
`Get-ChildItem -Recurse | Select-String` to find the sites it missed.

```fixture
src/a.spi
edit <root> <id> <deps|tags> <value>
show <root>
edit <root> <id> <deps|tags> <value>
src/b.spi
list <root>
edit <root> <id> <deps|tags> <value>
src/c.spi
nothing here
```

```command
eoie do . '
edit src/**/*.spi expect files 2 replacements 3
  replace all "<deps|tags>" with "<deps|tags|estimate>"'
```

```expected
edit src/a.spi  2 replacements  lines 1,3  sha256 <hash> -> <hash>
edit src/b.spi  1 replacement   line 2     sha256 <hash> -> <hash>
ok  files 2 of 3 matched  replacements 3  <elapsed>  <clock>
```

## 26. Counts that differ from the expectation write nothing (companion of 25)

```fixture
src/a.spi
edit <root> <id> <deps|tags> <value>
src/b.spi
list <root>
```

```command
eoie do . '
edit src/**/*.spi expect files 2 replacements 2
  replace all "<deps|tags>" with "<deps|tags|estimate>"'
```

```expected
edit src/**/*.spi  expected files 2 replacements 2, found files 1 replacements 1
src/a.spi:1  edit <root> <id> <deps|tags> <value>
nothing written: the counts differ
failed  files 0  replacements 0  <elapsed>  <clock>
```

## 27. Count rows of a table by two columns, filtered (json/csv + aggregate: 105 calls)

Habit (Claude, abbreviated): `Import-Csv results.tsv -Delimiter "`t" | Where-Object status -ne 'ok' | Group-Object
backend, status | Sort-Object Count -Descending | Format-Table Count, Name`.

```fixture
results.tsv
id	backend	status
s1	C	ok
s2	C	error
s3	Rust	timeout
s4	Rust	error
s5	Rust	error
```

```command
eoie do . 'table results.tsv where status <> "ok" count by backend, status'
```

```expected
backend  status   count
Rust     error    2
C        error    1
Rust     timeout  1
ok  rows 5  matched 4  groups 3  <elapsed>  <clock>
```

## 28. Hash files and say which are equal (hash: 669 calls with a hashing step in `intent-report.txt`; not a cluster)

Habit (Claude, abbreviated): `Get-FileHash a, b, c | Select-Object Hash, Path`, then compare the strings by eye.

```fixture
a.bin
alpha
b.bin
beta
c.bin
alpha
```

```command
eoie do . 'hash a.bin, b.bin, c.bin'
```

```expected
file   bytes  sha256
a.bin  6      b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060
b.bin  5      f2c82decdd7181cf98945929a62598db7e6b477e11f6e0eb0ae97020eff151ad
c.bin  6      b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060
equal  a.bin = c.bin
ok  files 3  bytes 17  distinct 2  <elapsed>  <clock>
```

## 29. Copy a tree without its temporary files (walk + copy/move/delete: 94 calls)

Habit (Claude, abbreviated): `Get-ChildItem src -Recurse -File | Where-Object Extension -ne '.tmp' | ForEach-Object {
New-Item -ItemType Directory -Force (Split-Path $dest) | Out-Null; Copy-Item $_.FullName $dest }`.

```fixture
src/a.spi
inl a = 1
src/b.spi
inl b = 22
src/x.tmp
tmp
src/sub/c.spi
inl c = 3
```

```command
eoie do . 'copy src/** to backup except *.tmp; files backup/**'
```

```expected
copy src -> backup  3 files  31 bytes  1 skipped (*.tmp)
path              bytes  lines
backup/a.spi      10     1
backup/b.spi      11     1
backup/sub/c.spi  10     1
ok  files 3 copied  bytes 31  skipped 1  <elapsed>  <clock>
```

## 30. A job with a wall-clock timeout and a notification (read + process + wait + clock: 57 calls)

Habit (Claude, abbreviated): `Start-Job { ... }; if (Wait-Job $job -Timeout 190) { Receive-Job $job } else { "still
running"; Get-CimInstance ... | Stop-Process -Force; Stop-Job $job }; Remove-Job $job -Force`, with the notification
coming from a separate watcher.

```fixture
slow.cmd
@echo off
echo step 1
ping -n 30 127.0.0.1 >nul
echo step 2
```

```command
eoie do . 'job start slow.cmd as j1 timeout 2s notify; job wait j1'
```

```expected
job j1  started slow.cmd  pid <pid>  timeout 2s
job j1  timed out after 2s  stopped (tree)  exit killed
notify  j1 timed out
j1 output (last 1 line)
step 1
failed  jobs 1  timed out 1  notifications 1  exit 3  <elapsed>  <clock>
```

## 31. Commits counted by day (aggregate + git: 86 calls)

Habit (Claude, abbreviated): `git log --since=2026-10-01 --format=%ad --date=short | Group-Object | Sort-Object Name
-Descending`.

```fixture
repo/a.txt
1
```

```setup
eoie do . '
git init repo
git commit repo all "one" at 2026-10-01T10:00:00Z
append repo/a.txt "2"
git commit repo all "two" at 2026-10-02T09:00:00Z
append repo/a.txt "3"
git commit repo all "three" at 2026-10-02T11:00:00Z'
```

```command
eoie do . 'git log repo since 2026-10-01 count by day'
```

```expected
day         commits
2026-10-02  2
2026-10-01  1
ok  commits 3  days 2  <elapsed>  <clock>
```

## 32. Search hits with each file's git state (read + search + git: 87 calls)

Habit (Codex, abbreviated): `rg -n 'TODO' repo; git -C repo status --short -- a.spi b.spi`.

```fixture
repo/a.spi
inl a = 1
// TODO split
repo/b.spi
// TODO rename
```

```setup
eoie do . '
git init repo
git commit repo all "init" at 2026-10-01T10:00:00Z
edit repo/a.spi
  replace once "inl a = 1" with "inl a = 2"'
```

```command
eoie do . 'search "TODO" in repo/** with git status'
```

```expected
repo/a.spi:2  // TODO split   modified
repo/b.spi:1  // TODO rename  clean
ok  files 2 searched  2 matched  matches 2  <elapsed>  <clock>
```

## 33. Run a suite to its end, then pull its marker lines (read + search + wait: 89 calls)

Habit (Claude, abbreviated): `pwsh monitor.ps1 -Script ... -TimeoutMin 150 | Out-Null; Get-Content workflow.log |
Select-String 'SUITE-EXIT|DISAGREE|REGRESSED' | Select-Object -Last 12`.

```fixture
suite.cmd
@echo off
echo row s1 ok
echo row s2 DISAGREE
echo row s3 ok
echo SUITE-EXIT 1
exit /b 1
```

```command
eoie do . 'run suite.cmd timeout 10s; search "SUITE-EXIT|DISAGREE|REGRESSED" in output'
```

```expected
run suite.cmd  exit 1  lines 4  <elapsed>
output:2  row s2 DISAGREE
output:4  SUITE-EXIT 1
failed  runs 1  exit 1  matches 2  <elapsed>  <clock>
```

## 34. Audit lab directories against the backlog (read + walk + agile: new verb, LAB-IN-REPO)

Habit (Claude, 2026-10-17, abbreviated): `Get-ChildItem lab -Directory`, then `eoie agile show <root> <ID>` per
directory, then `Select-String -Pattern 'C:\\Users'` over the sources, then a hand-written verdict per directory.

```fixture
lab/convo-in-spiral/schema.spi
inl main () : i32 = 0i32
lab/pack-night/apply.ps1
"applied"
lab/old-probe/probe.spi
inl main () : i32 = 1i32
lab/convo-in-spiral/bench.fsx
#r @"C:\Users\someone\.nuget\packages\hopac\0.5.1\lib\netstandard2.0\Hopac.dll"
```

```setup
agile init backlog
agile add backlog CONVO-IN-SPIRAL Epic P2 D6 "Convo in Spiral" spiral; agile set backlog CONVO-IN-SPIRAL 20 Paused
agile add backlog PACK-NIGHT Story P0 D3 "Night pack" compiler; agile set backlog PACK-NIGHT 100 Done
```

```command
eoie do . 'lab audit lab against backlog'
```

```expected
lab                  item             status  progress  verdict
lab/convo-in-spiral  CONVO-IN-SPIRAL  Paused  20        keep
lab/old-probe        -                -       -         orphan: no backlog item OLD-PROBE
lab/pack-night       PACK-NIGHT       Done    100       done: promote or delete
lab/convo-in-spiral/bench.fsx:1  non-portable path C:\Users\someone\.nuget\packages\hopac\0.5.1\lib\netstandard2.0\Hopac.dll
failed  labs 3  keep 1  orphan 1  done 1  stale 0  non-portable 1  <elapsed>  <clock>
```

## 35. A row is done only when its own anchor is gone (the row-local idempotency trap, BUNDLE-WISDOM)

Habit (old apply-spec scripts, abbreviated): skip a row when its replacement text already appears anywhere in the file.
Here the replacement text already sits on line 2, and the anchor on line 4 still needs the edit.

```fixture
lib/kit.spi
inl show_i32 (x : i32) : string =
    show_i64 (convert.i64 x)
inl show_u8 (x : u8) : string =
    x |> convert.i64 |> show_i64
```

```command
eoie do . '
edit lib/kit.spi
  replace once "x |> convert.i64 |> show_i64" with "show_i64 (convert.i64 x)"'
```

```expected
edit lib/kit.spi  1 replacement  line 4  sha256 <hash> -> <hash>
ok  files 1  replacements 1  <elapsed>  <clock>
```

## 36. Running the same edit again fails loudly (companion of 35)

```fixture
lib/kit.spi
inl show_i32 (x : i32) : string =
    show_i64 (convert.i64 x)
inl show_u8 (x : u8) : string =
    x |> convert.i64 |> show_i64
```

```setup
do . 'edit lib/kit.spi
  replace once "x |> convert.i64 |> show_i64" with "show_i64 (convert.i64 x)"'
```

```command
eoie do . '
edit lib/kit.spi
  replace once "x |> convert.i64 |> show_i64" with "show_i64 (convert.i64 x)"'
```

```expected
edit lib/kit.spi  anchor not found: "x |> convert.i64 |> show_i64" (0 matches, expected 1)
nothing written: 1 of 1 edits failed validation
failed  files 0  replacements 0  <elapsed>  <clock>
```

## 37. Edits that would empty a file write nothing (the empty-payload trap, BUNDLE-WISDOM: a source wiped to 0 bytes)

```fixture
a.txt
alpha
b.txt
beta
```

```command
eoie do . '
edit b.txt
  replace once "beta" with "BETA"
edit a.txt
  replace once "alpha\n" with ""'
```

```expected
edit a.txt  the edits would leave the file empty (was 6 bytes); empty it with eoie proxy fs-write --expect-hash
nothing written: 1 of 2 edits failed validation
failed  files 0  replacements 0  <elapsed>  <clock>
```

## 38. Latency between paired log events across files (read + walk + aggregate: log timing habit, new verb table sequences)

Habit (abbreviated): a loop over log files with a regex per event kind, a hash of open starts, and a subtraction per end, printed per file.

```fixture
runs/a.log
t=1000 request
t=1250 reply
t=2000 request
t=3900 reply
runs/b.log
t=500 request
t=900 reply
```

```command
eoie table sequences runs --time "t=(?<ms>\d+)" --latency lat=request --to reply
```

```expected
file	line	lat
runs/a.log	1	250
runs/a.log	3	1900
runs/b.log	1	400
eoie table sequences ok files=2 rows=3
```

The table goes to stdout and the receipt to stderr; each start pairs with the next end in the same file.

## 39. Files two trees keep in step, and which side moved (read + walk + hash: the mirrored-repos habit, new verb mirror check)

Habit (abbreviated): hash the same relative paths under two roots and list the ones that differ or exist on one side only.

```fixture
left/ci.yml
on: push
right/ci.yml
on: push
left/extra.txt
one side only
```

```command
eoie mirror check left right ci.yml extra.txt
```

```expected
left-only	extra.txt
eoie mirror check left-ahead files=2 differing=1
```

Anything but in-step exits 1, so a pre-commit step can refuse a change made on one side only.

## 40. May I edit this path, or is a release reading the tree (read + walk + process: the hand-rolled lease check, new verb window claim)

Habit (abbreviated): list the lease folder, parse each file's pid= line, ask the process table whether that pid still runs, and compare the leased root with the path about to be edited.

```setup
eoie window request tree-read 5 --label repo
```

```command
eoie window claim repo/src/a.spi
```

```expected
eoie window claim refused path=repo/src/a.spi edit refused: repo is read by win-<ms>-<pid>-tree-read for 5 more min
```

A path outside every leased root prints 'edit allowed' and exits 0; a refusal exits 1, so a hook or a script can gate an edit on the exit code alone.

---

## 41. The same filtered list, asked for by name (agile: the retyped long-flag list, existing verbs view-add and view)

Habit (abbreviated): retype 'agile list <backlog> --team eoie --open --load light --sort priority' every time the heavy slots are busy and light work is wanted, sometimes with a flag in the wrong slot.

```setup
eoie agile init backlog
eoie agile add backlog A-LIGHT Story P1 D1 "light first item" "eoie" --load light
eoie agile add backlog B-HEAVY Story P0 D2 "heavy item" "eoie" --load heavy
eoie agile add backlog C-LIGHT Story P0 D1 "light urgent item" "eoie" --load light
eoie agile view-add backlog light list --open --load light --sort priority
```

```command
eoie agile view backlog light
```

```expected
ID                         P  D     %  STATUS  KIND     EST    LOAD   WHO   TITLE
C-LIGHT                    P0 D1    0  Planned Story    -      light  -     light urgent item
A-LIGHT                    P1 D1    0  Planned Story    -      light  -     light first item
eoie agile list ok shown=2 filter=all load=light assignee=all open planned=3 active=0 blocked=0 paused=0 done=0 elapsed_ms=<ms>
```

A saved view runs its verb with its stored flags plus any extra flags given after the name, so the long list is typed once.

---

## 42. A serial chain of jobs in one call (jobs: N job starts, each pasting the previous id, new verb job chain)

Habit (abbreviated): start a job, read its id from the receipt, start the next with --after <that id>, and repeat for every job in the chain; when the order changes, stop each job and start it again with new ids (seven or eight re-chains in one day).

```command
eoie job chain -- one 60000 cmd /c exit 0 -- two 60000 cmd /c exit 0
```

```expected
eoie job start ok id=job-<ms>-one supervisor_pid=<pid> timeout_ms=60000 gate=plain label=one
eoie job start ok id=job-<ms>-two supervisor_pid=<pid> timeout_ms=60000 gate=plain label=two after=job-<ms>-one after_mode=any
eoie job chain ok jobs=2 ids=job-<ms>-one,job-<ms>-two
```

Each link starts with --after the previous id (--after-ok when the chain begins with --ok); a link that does not start stops the chain and names the ids started before it. 'job after <queued-id> <predecessor-id>' re-points a waiting link in place, keeping its id.

---

## 43. Which generated crates are over the line budget (walk + count: the per-repo gate.ps1 habit, new verb crate gate)

Habit (abbreviated): find every Cargo.toml with a [package.metadata.spiral] section, read its output file name, count that file's lines, and compare each with 1000; a repo-specific script per repo, and some scan only apps/.

```fixture
tool/Cargo.toml
[package.metadata.spiral]
entry = "tool.spi"
output = "tool.rs"
tool/tool.rs
fn a() {}
fn b() {}
fn c() {}
big/Cargo.toml
[package.metadata.spiral]
entry = "big.spi"
output = "big.rs"
big/big.rs
fn a() {}
fn b() {}
fn c() {}
fn d() {}
fn e() {}
```

```command
eoie crate gate . --enforce --limit 4
```

```expected
gate / tool tool.rs 3 lines ok
gate / big big.rs 5 lines OVER 4
eoie crate gate refused: 1 crate(s) over the limit; crates=2 over=1 limit=4 root=.
```

Without --enforce the same rows print and the call exits 0; with it, a crate over the limit or a crate whose output was never generated exits 1, and a root with no Spiral crate at all is refused rather than read as clean.

---

## 44. An alternation that silently matched nothing (search: a regex-looking needle searched literally, grep hint)

Habit (abbreviated): search for "a|b" expecting either word, get zero matches, conclude neither is there, and only later notice the search was literal.

```fixture
notes.txt
alpha one
beta two
```

```command
eoie grep notes.txt "one|two"
```

```expected
ok  files 1 searched  0 matched  matches 0  hint: the needle is literal; pass --regex to use it as a pattern  <elapsed>  <clock>
```

The hint appears only when nothing matched and the needle holds pattern syntax; with --regex the same needle finds both lines.

---

## 45. Per-run first events in a log that holds many runs (logs: a scratch reader per question, table sequences with sections)

Habit (abbreviated): one log file holds many runs, each opened by a header line; a scratch script splits the file at the headers, moves the lines stamped before a header (@knight+-1) into the run they belong to, and finds each monster's first move per run.

```fixture
trace.log
lead-trace monster 81 Hunter starts at 1931,1352 @knight+-1
lead-trace monster 82 Hunter starts at 1931,1353 @knight+-1
lead-trace run 20261023-030000 knight seen at 1932,1349 @knight+0
lead-trace think 81 at 1931,1352 @knight+600
lead-trace think 82 at 1931,1353 @knight+300
lead-trace monster 81 moved to 1930,1352 @knight+850
lead-trace think 82 at 1931,1353 @knight+1300
lead-trace think 82 at 1931,1353 @knight+2300
lead-trace knight targets 82 @knight+2500
lead-trace knight gone @knight+90000
lead-trace monster 71 Hunter starts at 1931,1352 @knight+-1
lead-trace monster 72 Hunter starts at 1931,1353 @knight+-1
lead-trace run 20261023-030300 knight seen at 1932,1349 @knight+0
lead-trace monster 71 moved to 1932,1352 @knight+700
lead-trace knight targets 72 @knight+2500
lead-trace knight gone @knight+90000
```

```command
eoie table sequences trace.log --section-on "run (?<section>\S+)" --carry "@knight\+-1" --time "@knight\+(?<ms>-?\d+)" --first move="monster (?<id>\d+) moved to (?<value>\S+)"
```

```expected
section	id	move_time	move_value
20261023-030000	81	850	1930,1352
20261023-030300	71	700	1932,1352
eoie table sequences ok files=1 sections=2 rows=2
```

A section starts at each line matching --section-on; lines matching --carry before a header belong to the section that header opens. --first keeps the earliest match per section and id, named by the word before '='. (Expected rows from tibia's synthetic trace, TIBIA-EVENT-TIME note 100, verified by hand and by tibia's scratch reader.)

---

## 46. First and last event per run and id, side by side (logs: life = last - first, table sequences --first and --last)

Habit (abbreviated): for each run in a multi-run log, find when each creature appeared and when it last acted (or died), by hand-splitting the file per run, then subtract.

```fixture
trace.log
lead-trace monster 81 Hunter starts at 1931,1352 @knight+-1
lead-trace monster 82 Hunter starts at 1931,1353 @knight+-1
lead-trace run 20261023-030000 knight seen at 1932,1349 @knight+0
lead-trace think 81 at 1931,1352 @knight+600
lead-trace think 82 at 1931,1353 @knight+300
lead-trace monster 81 moved to 1930,1352 @knight+850
lead-trace think 82 at 1931,1353 @knight+1300
lead-trace think 82 at 1931,1353 @knight+2300
lead-trace knight targets 82 @knight+2500
lead-trace knight gone @knight+90000
lead-trace monster 71 Hunter starts at 1931,1352 @knight+-1
lead-trace monster 72 Hunter starts at 1931,1353 @knight+-1
lead-trace run 20261023-030300 knight seen at 1932,1349 @knight+0
lead-trace monster 71 moved to 1932,1352 @knight+700
lead-trace knight targets 72 @knight+2500
lead-trace knight gone @knight+90000
```

```command
eoie table sequences trace.log --section-on "run (?<section>\S+)" --carry "@knight\+-1" --time "@knight\+(?<ms>-?\d+)" --first appear="monster (?<id>\d+) Hunter starts at (?<value>\S+)" --last think="think (?<id>\d+) at (?<value>\S+)"
```

```expected
section	id	appear_time	appear_value	think_time	think_value
20261023-030000	81	-1	1931,1352	600	1931,1352
20261023-030000	82	-1	1931,1353	2300	1931,1353
20261023-030300	71	-1	1931,1352	-	-
20261023-030300	72	-1	1931,1353	-	-
eoie table sequences ok files=1 sections=2 rows=4
```

Rows are one per (section, id) seen by any --first or --last, in section order and then by first sighting; a column with no match in that section prints '-'. Ids repeat across sections (each run restarts the server), so the key is the pair. Negative times (@knight+-1, lines carried into the next run) are kept as written. (tibia, EOIE-LOG-SEQUENCES note 15.)

---