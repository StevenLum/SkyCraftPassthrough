## 2026-10-12 published to GitHub: dev and master at 5b5f4dd

**Changed:** Records only (this entry, the handover note).

**Why:** User: "Commit and push to github as it is"; the branch was left to
the agent.

**Tested how:** `git fetch`, then `git branch -vv`.

**Result:** `git push origin dev` created the branch `dev` on GitHub;
`git push origin dev:master` moved `master` from a9fc0fd to 5b5f4dd as a plain
fast-forward. Local and remote `dev` and `master` all show 5b5f4dd.

**Still broken / not tested:** See the entry below: the installed mod has to
be built and installed again from the new folder.

**Commit:** 5b5f4dd Passthrough 0.10.57 to 0.10.59, a launcher, and the
project's new folder

**Next:** Build and install from the new place before the next play session.

## 2026-10-12 project folder moved and flattened; committed for GitHub

**Changed:** No source change. The user moved the project from
`D:\Documents\ChatGPT\Game modding` to `D:\Documents\Vibe coding\SkyCraft
attempt`. After the move Git's own folder (and `.claude`) sat in that folder
while every project file sat one level deeper, in a second `SkyCraft attempt`
folder, so Git saw every tracked file as removed. Asked how to publish, the
user said: "You can restructure the local files better first then push in the
best layout". The inner folder's contents were moved up beside `.git` (a
rename on the same drive) and the empty inner folder removed. The project
root is now `D:\Documents\Vibe coding\SkyCraft attempt`, laid out as it always
was on GitHub. Then everything uncommitted was committed on `dev`: 0.10.57
chair return and 0.10.58 effect healing (Codex), 0.10.59 fortify healing, the
launcher (`Play.cmd`, `tools\play.ps1`), all records, and
`docs\NATIVE_SKYRIM_MOD_PLAN.md` (a proposal written in another session; read
before committing, prose only). Done by Claude Code.

**Why:** User: "Commit and push to github as it is".

**Tested how:** After the move `git status --short` lists exactly the files
changed since 5ac01c9 and three new ones, nothing removed.
`.\tools\cargo.ps1 test --offline` at the new place.

**Result:** 89 Rust tests pass at the new location (the first run rebuilt the
libraries, 17 s). No build, no install.

**Still broken / not tested:** The mod installed in the games (0.10.59) was
built at the old place, and the place of its log folder and of `switches.txt`
is fixed when it is built (`CARGO_MANIFEST_DIR` in `native.rs` and
`switches.rs`). Until it is built and installed again from here, the games
write their logs under the old path, the collector finds no new logs, and
`switches.txt` in this folder is not read. The Minecraft-side checks and the
full build have not been run at the new place. `.claude\worktrees` still
holds a leftover work tree that points at the old path; untouched. Chair exit
(.57): still no result from play. The plan document's remark about the nested
folder is now out of date.

**Next:** Push. Then build and install from the new place before the next
play session.

## 2026-10-12 one launcher for both games (tool; mod stays 0.10.59)

**Changed:** New `tools\play.ps1` and `Play.cmd` in the project folder (a
double-click wrapper that only calls the script); one whitelist line for
`Play.cmd` in `.gitignore`. The script checks, then starts Skyrim with
`skse64_loader.exe` and Minecraft with Prism Launcher's `--launch 26.3`. It
writes nothing in either game. No change to the mod, its version or the
packets. Done by Claude Code.

**Why:** User: "Can you make a single launcher for this game?" Until now the
two games were started by hand, each with its own launcher.

**Tested how:** `.\tools\play.ps1 -Check` and `.\Play.cmd -Check` (check mode
prints what would be started and starts nothing), and once with a wrong Prism
path. Prism's `--launch` option read from its own `--help` (version 8.4).

**Result:** Check mode: "Passthrough mod 0.10.59 is installed in both games",
the two start commands, exit code 0. Wrong path: "NOT STARTED: Prism Launcher
was not found", exit code 1.

**Still broken / not tested:** The real start is **not tested**: the agent did
not start the games. Not seen: the stops for "Steam is not running", for
different mod files in the two games and for a game already running. The save
in Skyrim and the world in Minecraft are still chosen by hand. The paths of
the two games are written in the script, as in `install-build.ps1`.

**Next:** The user double-clicks `Play.cmd`.

## 2026-10-12 Test 0.10.59, second session: two hearts confirmed, food heals too

**Changed:** Records only. No code, no build. Done by Claude Code.

**Why:** The user: "You're right that the potion of healing also heals 2
hearts. Eating multiple random food items also eventually heal half a heart."

**Tested how:** `.\tools\collect-scene-logs.ps1`, then the healing lines of
`logs\scene-report-20261012-144608`.

**Result:** 13 heals handed over by Skyrim and 13 applied in Minecraft, in the
same order; no error lines.
- Potion of Health: 20, Minecraft 0.83 to 4.83 (4 points, two hearts).
- Potion of Minor Healing, three times: 25 each, 4.17 points each.
- Food, nine items of five kinds (items 0x00064B40, 0x00064B42, 0x05000EFE,
  0x03003540, 0x03003538; all through base effect 0x000F33CB): 1 or 2 points
  of Skyrim health each, 0.17 or 0.33 of a Minecraft point each, 1.67 points
  together. That is the "half a heart, eventually" the user saw, and it is
  the right size: Skyrim's food restores very little.

**Still broken / not tested:** Healing spells not tested. Chair exit (.57) not
tried. Not committed, by the user's word: "Dont bother with committing until I
say".

**Next:** Nothing owed on healing.

## 2026-10-12 Test 0.10.59: both kinds of health potion heal in Minecraft

**Changed:** Records only (this entry, README, STATUS, ABI, Backlog). No code,
no build. Done by Claude Code.

**Why:** The user played 0.10.59: "the minor healing potion worked. The potion
of health also healed 1 heart and I realise I was using the wrong potion once
you pointed it out."

**Tested how:** `.\tools\collect-scene-logs.ps1`,
`.\tools\check-test-report.ps1`, then the healing lines of
`logs\scene-report-20261012-143915` read in order.

**Result:** Protocol 33 on both sides, no switches file, no error lines.
- Potion of Health (favourites click at 323836015): `SCENE_HEAL_FORTIFY
  state=raised bonus=20 was=0`, `SCENE_HEAL_CAPTURE skyrim_heal=20
  fraction=0.2 source=fortify`, and in Minecraft `SCENE_HEAL_APPLY
  requested=4.0 before=11.665022 after=15.665022`. That is two hearts by the
  numbers; the user saw one. Three more clicks on it while it was running
  changed nothing, as expected.
- Potion of Minor Healing (inventory click at 323912468):
  `SCENE_HEAL_EFFECT_REQUEST requested=25 base_id=0x0003EB15
  item_id=0x0003EADD`, captured, applied `requested=4.1666665
  before=15.665022 after=19.831688`. This is Codex's .58 hook, seen working
  for the first time: it was right all along and had only never met a
  restore-health potion.
- The first value of the session (a potion still running from the save) was
  only remembered, not healed: `state=first_seen bonus=20`.
- Minecraft's receiver applied a heal in play for the first time.
- Two facts for the record: the bonus sits in Skyrim's "permanent" value
  (`lasting=120 passing=0`), not in the temporary one, so reading the plain
  value (slot 3, `plain=100`) was what made this work; slot 3 is confirmed.

**Still broken / not tested:** The raw effect line used up its 200 lines in
3.5 seconds on one standing effect that asks for 0 health every frame
(base 0x000E5F4B), so it recorded neither potion; it should skip requests of
0. The two routes divide by different numbers (plain 100 for fortify, maximum
120 for restore); harmless. Food and healing spells not tested. Chair exit
(.57): not exercised in this session, no result reported. Not committed: the
user has not said that all the testing passed.

**Next:** Ask the user whether to commit. Chair result still owed by play.

## 2026-10-12 a potion that raises maximum health now heals in Minecraft (candidate 0.10.59)

**Changed:** `passthrough/src/damage.rs`: every frame the bridge reads how much
health Skyrim's character has on top of its plain amount (maximum minus
ActorValueOwner slot 3, the plain value) and remembers it per damage
generation. A rise of half a point or more is queued as an ordinary heal for
Minecraft, as a share of the plain health; the first value of a generation is
only remembered. `SCENE_HEAL_FORTIFY` logs every change. New line
`SCENE_HEAL_EFFECT_RAW`: every call of the potion-effect function on the
steered character before any filter, 200 a session at most. New switch
`fortify_healing`, number 24, in `switches.rs` and `Switches.java`.
`tools\check-test-report.ps1` shows the new lines. Versions 0.10.59 and
Cargo.lock. No packet change, protocol 33. Done by Claude Code.

**Why:** "Health potions do not work. Codex tried and failed 3 times, give it
a try." The logs of the failed test (report 20261012-052857) show which potion
it was: Skyrim's maximum health went from 100 to 120 after each drink and back
by exactly 20 sixty seconds later (SCENE_DAMAGE_PROBE supplied=-20 at
290842593 and 290936937; SCENE_DAMAGE_READY skyrim_max 100 and 120). That is
"Potion of Health", a fortify effect: it raises the maximum and the current
health together and never makes a "restore" call. .30, .53 and .58 all waited
for a restore, and .58 leaves fortify effects out on purpose.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New Rust
tests with the report's numbers: first look only remembers; 0 to 20 gives 20;
running out, no change, noise and non-numbers give nothing; 20 on a plain 100
is a fifth; the raw line reads no further than its pointers allow.

**Result:** 89 Rust tests (83 unit, 5 connection, 1 position), 113 rule checks
and 136 hook checks pass. Packages 0.10.59 built.

**Still broken / not tested:** Not played. Slot 3 of Skyrim's value table as
"the plain value" is from CommonLib and has not been seen in this version; if
it returns nonsense the code falls back to the temporary bonus alone, and the
log line carries all the numbers. Minecraft's side of healing has never been
seen applying a heal in play. Nothing is taken away when the potion's minute
ends. Drinking the same potion again while it is still running gives nothing,
in Skyrim too. Putting on enchanted armour that adds health heals once as
well. Restore-health potions ("Minor Healing" and the like): still unproven;
the raw line will show at the next test whether Codex's .58 hook is reached.
Chair exit (.57): no result reported.

**Next:** User installs 0.10.59 and runs the test at the top of README.md.

## 2026-10-12 healing still failed on 0.10.58; extra attempt stopped

**Changed:** Failure records only: STATUS first handover, README, DESIGN result/
Backlog, ABI observation. Healing source and packaged .58 preserved. No new repair.

**Why:** User reports "Healing still doesnt work", then confirms the potion was
consumed while Minecraft hearts were missing. This rejects the one extra healing
attempt authorized after .30/.53 failures. AGENTS requires stopping and handover.

**Tested how:** Authorized collect-scene-logs.ps1 then check-test-report.ps1 on
report20261012-052857; read exact menu/hook/health events and filter/lease order.
Read-only cross-check agrees. Baseline87 Rust tests pass; no new build or game action.

**Result:** Effect hook installed1/owner hook1; protocol33 connected, all switches
on, zero ERROR/disabled. Five item-use dispatches (favourites1/inventory4), all9
menu commands game_owned. Effect request0, owner request0, heal capture/drop/apply0.
Minecraft health11.665022 in all555 samples. Potion consumption comes from user;
item identity/count not logged. No qualifying restoration entered bridge queue.

**Still broken / not tested:** Healing FAILED, not accepted. Exact cause unknown:
request is logged only after effect metadata filter/native return. Zero requests
cannot distinguish callback bypass from filter rejection. Lease rejection alone
would still leave a qualifying request log. Native health/max100-120 variation
cannot identify the potion/cause. No proof that Minecraft receiver failed.
Chair gameplay outcome not reported; five held/clear/accepted returns measured,
not a user pass. Other accepted/deferred outcomes retained.

**Next:** Healing STOPPED per explicit extra-attempt limit. No further fix,
rebuild/install/retest, staging/commit/push this turn. Failure handover first in
STATUS.md includes tried approaches and missing evidence; future investigation
requires user authorization. Report inspection command at README top.

## 2026-10-12 ordinary potion effect capture (candidate 0.10.58)

**Changed:** damage.rs optionally hooks ValueModifierEffect slot32, copies effect
metadata before calling the original once, then forwards positive health requests
through the existing player/menu lease and queue. Thread-local ownership guard
prevents nested owner6 double capture. skse.rs resolves AE206143 optionally;
effect_healing switch23 appended on both sides. Versions/Cargo.lock, checker,
README/ABI/Backlog and handover updated. Separate .57 chair candidate included.

**Why:** .30/.53 healing failed. Installed runtime ordinary potion effect calls
Actor::ModActorValue AE38468 directly, bypassing owner virtual6. OnStart/Update
use effect virtual32. Vanilla restore-health base3EB15 has health24/kRecover clear;
no full-health gate found in this path. Capture requested amount even when native
health stays full; don't modify native health. User authorized one extra attempt.

**Tested how:** Pinned native class headers and installed runtime read-only
instructions/vanilla records, retained in ignored .tools/logs. Six new regressions:
direct bypass at full native health plus metadata lifetime, unchanged native ABI
and one call in off/invalid cases, nested owner suppression/rollback, filters,
RAII guard unwind/nesting and stale player/menu/revision lease. Existing menu pair,
normal-regeneration filter and Minecraft health-event checks remain. Final chair
review found real prior acknowledgements15-141ms, below its500ms validity window.

**Result:** Offline package-only build passed87 Rust tests (81 unit,6 integration),
113 Java rules and136 hook checks. Both ZIP DLL hashes equal release and package
folders; Fabric manifest/four versions/Cargo.lock .58, protocol33,24 switch indices
agree. No test/build failures in .58. Optional hook protection failure logs once
and keeps ordinary movement available. No installation/staging/commit/push.
Accepted .56 work already committed5ac01c9 on dev under standing instruction.

**Still broken / not tested:** Actual chair exits and potion-to-Minecraft healing
NOT TESTED. Base ValueModifierEffect only; Peak/Accumulating/Dual/custom effects,
food/healing spells and native-hand outcome not established. Minecraft's existing
receiver still must be verified by play; synthetic native callback isn't acceptance.
One extra attempt each, not unlimited. Earlier failures are preserved.

**Next:** User installs .58 and runs focused README checks. Effect request/capture/
apply and chair held/clear/accepted logs distinguish stages. Rollbacks:
chair_return=off / effect_healing=off, restart both games. Collect/check logs after
user report. If an extra attempt fails, stop with the failure handover.

## 2026-10-11 chair return to pre-seat feet (candidate 0.10.57)

**Changed:** seat.rs remembers authoritative entry feet; recovery.rs chooses only
supported terrain/NPC-clear returns; skse.rs holds the standing actor/controller
while fresh scan and recovery acknowledgement complete. world_export.rs exposes
its existing NPC collector. chair_return switch22, versions/Cargo.lock, checker,
README/ABI/Backlog and handover updated. No packet change or healing edit.

**Why:** Report225755 still waits2.4s at a blocked native exit after standing.
Prior .50/.51 failed; user authorizes one additional attempt. Native exit location
can drift under physics while Minecraft waits. Preserve original mapping and
return toward the previously usable Minecraft point instead.

**Tested how:** New entry/scope/furniture, low exit, support/NPC/wall/ceiling and
standing hold fixtures. Read back actor/controller after placement; ignored setter
must not count as held. Pending targets recheck live NPCs and expire geometry at
500ms; suspension clears scans. Package-only offline build before packaging.

**Result:**81 Rust tests (75 unit,6 integration),113 Java rules,136 hook checks
pass. Both .57 ZIP DLL hashes match release; Fabric manifest .57/protocol33.
No install/staging/commit/push. Intermediate compile failure E0384 from removing
mut on the existing controller variable corrected. Two fixture expectations fixed:
unchanged entry needs no floor epsilon; test Sample.frame must be nonzero.
Review found unsupported entry and stale pending clearance/read-back gaps; fixed
and tested before final build. These are automated checks, not game acceptance.

**Still broken / not tested:** Actual chair/table and stool/NPC exits NOT TESTED.
This is the single extra attempt; if it fails in game, stop. Camera/hand behavior
not established. Healing still failed until next independent step. Bed/workbench
alignment not changed; accepted/deferred tests retained.

**Next:** Separate healing .58 plan/build, then user installs final package and
runs focused tests in README. chair_return=off plus restart restores .56 chair path.

## 2026-10-11 0.10.56 gameplay accepted; extra chair/healing attempts authorized

**Changed:** Acceptance and permission records only. User reports "All working"
for .56 take/equip/use, held click, category targeting and favourites checks.
Chair and healing are explicitly reopened for one additional attempt each;
AGENTS.md records exception for both agents. Earlier failures remain historical.

**Tested how:** Authorized collector/checker report20261011-225755; baseline77
Rust tests pass. Logged17 mouse dispatches:15 item_use (14 inventory,1 favourites),
2 pointer; skipped passive menus1=3/2=14. No ERROR/disabled. Actual gameplay
acceptance comes from user. Container passed by user, not separately in this log.

**Result:** .56 left clicks accepted. About to commit current accumulated source
and records on dev under standing instruction. No install or push/master changes.

**Still broken / not tested:** Chair exit obstruction and Minecraft potion
healing remain failed. Report has1 terrain_blocked exit,1 native clamp request2,
zero restore callbacks/captures/applied. No new fix tested. Other deferred checks
remain deferred. Additional repair attempts start only after evidence/plan.

**Next:** Chair diagnosis and separate candidate step, then healing. Each gets
rollback/testing/logs. If additional attempt fails, stop with failure handover.

## 2026-10-11 0.10.56: skip passive renderer when routing inventory left click

**Changed:** Corrected input-menu selection in menu_input.rs; optional explicit
ContainerMenu identity in skse.rs; all four version locations/Cargo.lock to .56.
Click diagnostics add passive_skipped, stack_index, selected_vtable; report checker
counts skipped layers. Existing item_click rollback retained, no wire changes.

**Why:** User accepts conversation/buying but cannot take/equip/use in inventories.
Collected report20261011-224351:52 mouse dispatches, ALL menu=other/route=pointer,
flags0x8840/context19, no item_use. The .55 fix never selected an item menu.
This records failure of attempt1; .56 is attempt2 of2. Earlier native-use hypothesis
has not been tested in game because those events were never selected.

**Tested how:** Baseline76 Rust tests pass. Added exact reported rendering-layer
regression and ownership barriers, including barter/dialogue/unknown popup, plus
container/off/missing/depth/single press-release tests through synthetic native
sink. Installed ContainerMenu message7 compares RightEquip+0x230 and invokes
Flash AttemptEquip just as InventoryMenu; read-only diagnostics in ignored logs.
Offline package-only build ran Rust/Java rules/hooks before packaging.

**Result:**77 Rust tests (71 unit,6 integration),113 Java rule checks,136 hooks
passed. Source/packages .56, protocol33. No install/staging/commit/push.
Rendering-only layers no longer mask an underlying recognized item menu in the
fixture; real input popups/barter/dialogue retain Click. Both ZIP DLL hashes match
release; Fabric manifest/all source versions agree. UTF-8:113 files, git diff --check
passes. Ignored counter fixture reports3 item_use/1 pointer, skips0=1/1=2/2=1.

**Still broken / not tested:** Gameplay take/equip/use and category/hover behavior
NOT TESTED on .56. Conversation/buying accepted on .55, not replayed by the agent.
Skyrim item use does not establish Minecraft healing. Chair/healing remain stopped;
accepted movement/door/bed tests kept. Custom menus/mouse remapping not tested.

**Next:** User install .56; focused take/equip/use, held-click and category target
checks in README. Collect logs after report. If attempt2 fails, stop with handover,
no third guessed fix. Package/hash/UTF-8 review completed.

## 2026-10-11 contextual left-click item use (0.10.55)

**Changed:** Native action10 routing in menu_input.rs: InventoryMenu -> RightEquip,
FavoritesMenu -> Right Attack/Block, other menus -> Click. One press/release only;
pointer totals dispatched before click. Optional vtable support resolved in skse.rs;
item_click rollback switch21 appended on both sides. Version .55 in all four
locations/Cargo.lock; no protocol/layout change. Java menu comments corrected.
SCENE_MENU_CLICK logs route, menu, native event, flags/context and cursor position;
checker tallies route/menu. Records and Backlog updated.

**Why:** User says left click cannot use but E does. Report221546 has20 mouse
Click deliveries and3 Accept deliveries, with no item-use confirmation. Read the
installed PC controlmap directly from BSA: mouse0 = Right Attack/Block, linked as
RightEquip in Item Menus. Native ClickHandler queues only GFx mouse down/up and
returns handled, preventing generic user-event dispatch; FavoritesMenu native
message7 branch handles Right Attack/Block explicitly. This is a contextual event
mismatch. Native references came from pinned CommonLib and read-only installed
binary; disassembly/libraries stay in ignored logs/.tools, no game assets tracked.

**Tested how:** New routing rule and synthetic native input-sink fixture verify
exact events, one down/up, pointer-before-use/no second motion, popup precedence,
native depth15 cutoff, off/missing-support fallback, busy controls and unchanged
E/non-click commands. Package-only offline build runs all native/Java checks.

**Result:** 76 Rust tests (70 unit, 6 integration), 113 Java rule checks and136
hook checks passed. Both ZIP DLL hashes match release; Fabric manifest .55; all22
switch indices match. Checker fixture reports item_use=2, pointer=1, all3 menus.
No install/staging/commit/push. No build/test failure in this step.

**Still broken / not tested:** Actual item use/equip, Flash hover timing and click
behavior with modified menus/remapped mouse controls are NOT TESTED. Optional
click binding shares action10 and the contextual native left mouse action.
Chair and potion-to-Minecraft healing remain stopped after earlier failed attempts;
this step does not establish healing. Accepted door/bed/held-E/movement checks kept.

**Next:** User install .55; focused inventory/favourites use and held click, plus
one dialogue pointer choice if available. Check category clicks still navigate.
UTF-8 verified for114 files, whitespace checks pass, diagnostic folders ignored.
Inspect actual item count/equip mark,
not Minecraft health; collect logs/check report after feedback.

## 2026-10-11 0.10.54 gameplay rejection and two-attempt failure handover (no build)

User accepted no later door teleport, owned-bed grounding and held entering E.
Lower chair with table and high stool with NPC still fail on exit; interference
not proven. Potion healing still fails; left-click item use fails while E works.
Native hand visibility outcome not separately confirmed; workbench not tested.
Future face-away-from-door request recorded in DESIGN Backlog.

Collected logs/scene-report-20261011-221546 and ran check-test-report.ps1:
zero ERROR/disabled, 17 recovery acknowledgements, six terrain-blocked chair
exits, six furniture waits/dispatches, one installed restoration hook but zero
requests/captures/applications. 20 action10 mouse clicks and 3 action5 confirms
reached native dispatch. Native120 health / Minecraft13.665022. First blocked
chair exit holds MC at y128.125000238 for about6.5s while native gravity remains
restored; actual obstruction and native camera/controller heights unrecorded.

Chair .50 exit and .51 terrain correction both failed. Healing .30 clamp capture
and .53 restore-entry/menu lease both failed. Stopped those repairs under the
AGENTS.md two-attempt rule; STATUS first section records approaches, evidence,
unknowns and diagnostic leads. Do not infer passing native behavior from fixtures.
True click routing predates these repairs; regression cause not established.
No code edits/build/install/staging/commit. Baseline 74 Rust tests pass. Existing
113 Java and136 hook build checks not rerun for documentation-only changes.
README/ABI/DESIGN/Backlog updated. No new gameplay retest request.

## 2026-10-11 native first-person arms and final repair checks (0.10.54)

Only Skyrim's first-person model root is hidden while the bridge owns look/rendering.
Original hidden bit is restored on release or model replacement; unrelated flags
are preserved. NiRefObject reference is retained until restoration so loads cannot
invalidate the saved pointer. Native model getter/visibility/refcount ABI verified
in pinned CommonLib (Get3D1 slot 0x6F, flat flags 0xF4 bit 0, refcount 8,
DeleteThis slot 1). Native_hands switch 20 restores prior visibility behavior.
No camera height, third-person skeleton or furniture animation changes.

Final build includes separate 0.10.51 recovery, 0.10.52 grounding and 0.10.53
healing repairs. Callback regression verifies original native restore is called
once at already-full health and the requested potion amount still reaches capture.
Recovery scan identity includes both paired IDs and restarts on place/pair change.
Checker now reports terrain checks, landing outcomes, native model state, health
hook installation and raw restore requests.

Final offline package-only build: 74 Rust tests (68 unit, 5 connection, 1 position),
113 Java rule checks, 136 hook checks passed. Both package DLL hashes match release;
Fabric manifest 0.10.54; all 21 native/Java switches agree. No install/commit.
Hands test covers repeated hides, refcount balance, original hidden state, unrelated
flag changes, node replacement/null and release. Real rendering and all four repairs
are NOT TESTED. Low-chair camera cause is still uncertain; model visibility is the
specific unwanted-rendering change. Earlier accepted A/E/F/H/I remain accepted.

Next: install 0.10.54 and perform the focused four checks at the top of README.

## 2026-10-11 potion request capture through menus (0.10.53)

ActorValueOwner slot 6 ModActorValue is now hooked, preserving the original native
call and capturing positive health kDamage requests even if native health is full.
The older damage clamp no longer captures positive changes, avoiding double healing
through nested calls. Negative damage routing is unchanged. A separate 250 ms
healing lease is refreshed by normal play and by the already-authorized live menu
heartbeat; combat readiness stays off in menus. Menu heartbeat retains queued
healing until the menu closes; stale/disconnected/wrong-pair menus cannot renew it.
Load/pair replacement resets lease and regeneration bucket. Existing healing switch.

Package-only build: 72 Rust tests, 113 Java rules, 136 hook checks passed. Restore
modifier/value filtering, raw full-health potion amount, menu pair/revision isolation
and regeneration threshold tested. Native entry verified in pinned local CommonLib
ActorValueOwner.h/.cpp. Actual potion callbacks and healing remain NOT TESTED.
No installation, staging or commit. Next: 0.10.54 native first-person arm visibility.

## 2026-10-11 native landing before beds/crafting (0.10.52)

A fresh bed/crafting request restores native gravity and holds Minecraft while
Actor::IsInMidair (AE 37243, bool Actor*) is true. Dispatch occurs once after
landing, only for the original target/pair/revision, within two seconds. Focus
or menu suspension cancels waiting activation; no delayed replay. Native furniture
animations own position until standing; terrain-safe recovery then aligns exit.
New furniture_ground switch 19; protocol remains 33.

Package-only build: 70 Rust tests, 113 Java rules, 136 hook checks passed. Pure
cases cover timeout, target/pair/load changes and native sleep/crafting states;
adapter fixture verifies restored gravity, no movement writes while airborne,
once-only bed dispatch after landing and recovery acknowledgement. Fixture needed
cooldown, fresh timestamp, idle resume and increasing frame corrections; its traces
identified the existing guards, which remain intact. No installation/commit.
Real owned/unowned beds and workbenches remain NOT TESTED on this repair.

Next: 0.10.53 potion restoration entry and paired menu authorization.

## 2026-10-11 terrain-safe recovery (0.10.51)

Recovery waits for a fresh scan around native arrival feet before requesting the
existing token teleport. A terrain overlap can lift only the height, at most half
a block, and requires standing clearance; walls/low ceilings hold rather than
teleport through. Applied to door/load arrivals and intentional chair exits.
Separate recovery exporter leaves ordinary flight pacing and saved origins alone.
New safe_arrival switch 18 restores old recovery when off. Protocol stays 33.

Offline package-only build passed: 68 Rust tests, 113 Java rules, 136 hook checks.
Reported low-chair/exterior coordinates covered by floor examples; clear air,
nearby nonoverlap, walls, low ceiling and excessive lift covered. Real terrain
and gameplay remain NOT TESTED. No installation or commit.

Next: 0.10.52 native grounding before bed/crafting activation; then potion capture
and native first-person model visibility. Original play report remains below.

## 2026-10-11 play report: 0.10.50 partly accepted, repairs pending

User: A creative movement passed; B original sequence allowed leaving, but an
unrequested move followed outside; C E stand passed (held-E wording unclear);
D lower chair caused falling through the world and a Skyrim hand in the view;
E focus/menu sitting passed; F seated save/reload passed; G owned bed failed
with an airborne message until pausing Minecraft let Skyrim land; workbench
not tested; H close combat movement passed; I obstructed/empty bow shots passed;
L potion healing failed from both items and favourites. J/K/M/N/O not tested.
No previously deferred tests are being requested again.

Collected logs/scene-report-20261011-213834, protocol 33, no ERROR/disabled lines.
Seven native intentional exits/alignment acknowledgements and one automatic
loaded-seat stand-up. Chair token 19 put feet at y=127.997305849, below the indoor
floor; the following frames fell. Exterior token 6 put feet at y=40.013725063,
then Minecraft fell hundreds of blocks and Skyrim recovered to another exterior
position (token 7). The extra outside teleport and chair fall have the same
candidate cause: recovery trusts native actor feet without checking Minecraft's
sampled terrain. Potion request did not reach the clamp callback: no healing
capture/application; only positive clamp was 9999 on load. Inventory suspension
also sets damage readiness to zero, so a healing callback must retain paired
menu authorization. Hand visibility cause not yet established.

Baseline: 66 Rust tests passed. No installation, commit, or staged changes.
Next: separate repairs for recovery placement, furniture activation grounding,
health restore capture, and first-person hand visibility. Gameplay repairs not
started. Keep one feature per version; plan before code.

## 2026-10-11 E sits; WASD or E stands before walking (0.10.50)

**Changed:** Intentional seat ownership in seat.rs, activated for ordinary
sit-capable furniture only when the stand-up address is available. Interaction
records chair activation; attack/crosshair reports it as usable. While entering,
seated or standing, skse.rs restores saved gravity and skips movement/controller
writes, preserves view/rendering/health, and publishes a held-seat heartbeat
immediately then every 125 ms. Fresh WASD or E calls StopInteractingQuick with
bounded retries; actual standing starts the existing coordinate-recovery
handshake before steering resumes. E cannot be replayed as a chair/door action.
Intent survives ordinary pause/menu/focus suspension, clears on load/death/pair
change. Automatic recovery remains for unowned seated saves. sitting switch 17.

Protocol 33 (same frame15, sizes and PTWORL14 as 32), new validated 0/1 held-seat
word at TARGET_AT+4. Java gives seated E precedence even without a crosshair and
holds movement ahead of collision coverage. Report checker counts seat outcomes.
Creative idle scans retain slow pacing despite tall coverage (flight follow-up).
All records and the focused test procedure updated; no install or commit.

**Why:** Blocking seats removed an interaction the user still wants. Moving an
actor while Skyrim still owns its chair was the suspected door problem. Requested
controls: E sits, WASD OR E stands. Also record partial flat-ground success and
creative failure honestly rather than treating the full door sequence as passed.

**Tested how / result:** Final offline package-only build passed: 66 Rust tests
(60 unit, 5 connection, 1 position), 113 Java rule checks, 136 hook checks. Six
intent-state tests cover indefinite sitting, both exit keys, fresh/repeated/stale
E, activation timeout, ownership/reset, native retry cap and clock rollback.
Packet flag rejects unknown values. Adapter fixture dispatches E on a classified
chair, verifies retained native chair position/restored gravity, no auto recovery,
WASD native call, exit target/ack and no replay. Quiet creative scan budget tested.
FFM frame15's 30 argument types match the native export; no old runtime offsets
or export names remain. Both ZIP DLL hashes match release, Fabric says 0.10.50;
all 18 Rust/Java switch numbers match. UTF-8 checked for 114
source/non-ignored files and git diff --check passed. Synthetic report fixture
counts each sitting outcome once and skyrim_seated=1; not gameplay evidence.

**Still broken / not tested:** New seating, animations/camera, real E/WASD exit,
creative ascent/falling, dense scan timing and full original menu/shout/chair/door
sequence are **not tested**. Beds/workbenches remain **not tested**. Native stand-up
itself worked once in the preceding 0.10.48 session; that does not verify this mode.
Riding, wood chopping/mining, invisible arrow and indoor calm-scan issue deferred.

**Next:** User installs 0.10.50 and tests README A/B/C. No repeat of accepted flat
ground jumps; no deferred checks reopened. Collect logs directly after report.

## 2026-10-11 Creative flight and long-fall coverage (0.10.49)

**Changed:** Half-block collision sampling now supports a tall prism: radius 4
sideways, 12 vertically during creative flight permission or fast falls, 24 at
vertical speeds above 40. Fast scans allow 1536 casts within the unchanged 2 ms
frame budget. Velocity averages at least 50 ms rather than one rendered frame.
New fast_scan switch 16. Protocol 32 / frame15, controls word 396, world header
144 bytes, PTWORL14; Rust and Java parsers and the NPC reader updated together.

**Why:** Creative ascent/falling crossed the old cube and lost prediction during
steady flight. Flight permission remains true after toggling flight off, keeping
tall coverage during initial fall acceleration. Ordinary on-foot jump aim stays.

**Tested how / result:** 59 Rust tests, 104 Minecraft arithmetic/rule checks, 136
hook checks passed; offline package-only build completed. Tests cover all face
rays at half-block spacing, supported tall packets and invalid radii/controls,
terminal fall/ascent through two 100 ms scan intervals plus next movement tick,
velocity jitter/reset and existing repeated jumps. Cached 26.3 javap confirms
Abilities.mayfly is separate from flying. Both 0.10.49 packages built, not installed.

**Still broken / not tested:** Creative gameplay, dense terrain scan latency and
actual costs are **not tested**. No collision hold was removed or bypassed; slow
or stale geometry still holds safely. Intentional chair sitting is the next step.

**Next:** 0.10.50 intentional seating, then user installs/tests the combined build.

## 2026-10-11 Partial play report for 0.10.48; new creative-flight failure

**User result:** Sprint-jump movement seamless on relatively flat ground tested.
Leaving the room worked, but the original enter/shopkeeper menus/shout/chair/leave
sequence is **not tested**. Workbenches and beds **not tested**. User still wants
E to sit down and WASD or E to stand up. Blocking all seats was not the desired
final behavior. Creative ascent and flight-off falling interrupted movement.

**Evidence:** Collected report logs/scene-report-20261011-194102. 21 outside_bounds
entries: 19 downward, 2 mostly sideways while rising; observed holds about 50 ms.
The native stand-up recovered sit=3 to sit=0, and the front door then caused a
cell transition. This verifies the real recovery call in this session, not the
cause of the original failure. No shout command was logged; the full sequence
remains unverified. No gameplay claim for beds/workbenches.

**Next:** Separate versions: 0.10.49 fixes fast-movement coverage; 0.10.50 restores
intentional seating. No commit: the user has not said all testing passed.

## 2026-10-11 Timed recovery of a seated Skyrim character (0.10.48)

**Changed:** Codex completed Claude Code's step 3. seat.rs adds the pure Watch
rule: four seconds non-standing while steered, two seconds between attempts,
five attempts then one gave_up after a final two-second wait. Standing, a gap
over 500 ms, a backwards clock or releasing the character resets it. skse.rs
resolves the optional StopInteractingQuick address (38697), logs before and
after calling it with true, then rechecks game state and the movement
controller. Missing address disables only this recovery. stand_up is switch
15 in Rust and Java. Version 0.10.48 in the four source locations. The report
checker counts the new outcomes. README, ABI notes and Backlog now cover all
three repairs. No packet change; protocol 31.

**Why:** A Skyrim character left seated by a stool can remain seated while
Minecraft moves it; this is the suspected cause of the meadery doors failing.
0.10.47 stops new seat activations; this step handles an already-seated save.
The cause still fits the logs rather than being proven in the game.

**Tested how:** Baseline .\tools\cargo.ps1 test --offline (50 passed).
Five new timer tests cover standing, retry timing and the call cap, resets,
nonzero state transitions and workbench sit/menu/stand timing. The existing
simulated Puppet test now observes the stand-up call with true and its
standing state, followed by normal movement. CommonLib Actor.cpp was checked
for the void(bool) signature and RELOCATION_ID(37752, 38697).

**Result:** .\tools\build-step5b.ps1 -Offline -PackageOnly passed: 55 Rust
 tests (49 unit, 5 connection, 1 position), 99 Minecraft rule checks and 136
 hook checks (23 hook files). Both 0.10.48 ZIPs are built; their DLL hashes match
 the release library and the packaged Fabric manifest says 0.10.48. All 16
 Rust/Java switch numbers match. The report checker counted the before, after,
 disabled and gave_up fixture outcomes once each. UTF-8 reads and git diff
 --check passed (Git's existing line-ending warnings were left alone).

**Still broken / not tested:** Gameplay of 0.10.46 through 0.10.48 is not tested.
The native stand-up routine has not been called by this mod in real Skyrim;
use stand_up=off if it misbehaves. Furniture offsets remain unverified for
this runtime; seat_guard=off restores old activation behavior. Sitting,
riding, mining and wood chopping from Minecraft remain unsupported.

**Next:** The user runs Test 0.10.48 at
the top of README.md. Do not install or commit for them.

## 2026-10-11 A Skyrim seat is not something E uses; the log says what a press met (0.10.47)

**Changed:** New file `passthrough/src/seat.rs`: what a seat is (furniture,
record type 0x28, with no workbench and not a bed) and the sit state of a
Skyrim character (bits 14 to 17 of the state word at +0xC8). `attack.rs`: a
seat under the crosshair is reported to Minecraft as kind 0, "nothing to use",
so E opens the Minecraft inventory there; the log line says `seat=true`.
`interaction.rs`: a press aimed at a seat is refused (`state=refused
reason=seat`; this is what stops the R key, which does not look at the kind),
and every press handed to Skyrim first writes `SCENE_INTERACT_TARGET` with the
target's form ID, base form ID and record type, its lock (none, open, locked,
level, key) and the sit state of Skyrim's character. `skse.rs`: `SCENE_SEAT`
whenever that sit state changes while the character is being moved; two more
addresses for the lines above (reference lookup, already used elsewhere, and
TESObjectREFR::GetLock, ID 20223, optional). New switch `seat_guard`, number
14, in `switches.rs` and `Switches.java`. `tools\check-test-report.ps1` has a
section for these lines. Versions 0.10.47. No packet change: kind 0 already
existed. Done by Claude Code.

**Why:** The user could not leave Honningbrew Meadery in Test 0.10.45. The
logs point to Skyrim's character having sat down on a bar stool when E was
pressed on it, and staying seated while the bridge moved it; a seated
character in Skyrim opens no door (entry "Test 0.10.45" below). The sit state
was not logged, so that reading could not be proven; now it would be.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New Rust
tests: the seat rule with the numbers read from Skyrim.esm for that meadery
(stool and chairs: bench type 0, "can sit"; beds: "can sleep"; every workbench
type is not a seat; no other record type is one); the sit-state bits do not
pick up their neighbours in the state word, the life state among them; a
reference whose base record is not furniture has only its type byte read; the
lock names.

**Result:** 50 Rust tests (44 unit, 5 connection, 1 position), 99 rule checks
and 136 hook checks pass. Packages 0.10.47 built.

**Still broken / not tested:** Not played. Where the bench type and the marker
flags sit in a furniture record (+0xE0, +0xE4) comes from CommonLib and has
not been seen in this game version; if it is wrong, either chairs are still
used (as before) or workbenches and beds are refused, and `seat_guard=off`
puts the old behaviour back. The log line carries the raw numbers. This step
does nothing for a character that is already seated: a save made in that state
needs 0.10.48. Sitting, wood chopping and the like cannot be done from
Minecraft at all now (they left the character stuck before). A horse is a
character, not furniture: E still mounts it.

**Next:** 0.10.48, standing a seated character up.

## 2026-10-11 The collision cube stays put for a jump (0.10.46)

**Changed:** `passthrough/src/world_export.rs`: the look-ahead of the
collision cube on the height axis is limited to 0.75 block again for any speed
up to 13 blocks a second, growing evenly to the 1.75 of 0.10.41 at 19 blocks a
second (`lead_up`, used by `aim`). Sideways unchanged (2.75). Versions
0.10.46. No packet change, no switch. Done by Claude Code.

**Why:** Letter B of Test 0.10.45 failed: seven freezes of 50 to 100 ms while
sprint-jumping (entry below). 0.10.41 had raised the limit to 1.75 for every
speed; a cube aimed while the player comes down from a jump was then placed
1.75 blocks lower, the fall ended on the ground, and the next jump put the
player's head above the cube.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New Rust
test `a_cube_aimed_during_a_jump_still_holds_the_player_through_the_next_one`:
it runs Minecraft's own jump arithmetic (take-off 0.42 a tick, gravity 0.08,
drag 0.98, jumping again on landing), aims a cube at every moment of it, for
four scan durations and six ground heights, and requires that cube to hold the
player for the following 0.4 s. With the limit of 0.10.41 put back for a
moment the test failed at "aimed at tick 10, used at tick 16", which is the
freeze seen in play; with the new limit it passes. The older look-ahead test
now states the limit per speed.

**Result:** 46 Rust tests (40 unit, 5 connection, 1 position), 99 rule checks
and 136 hook checks pass. Packages 0.10.46 built.

**Still broken / not tested:** Not played. A drop of more than about two and
a half blocks followed at once by a jump can still leave the cube for a
moment, and so can creative flight straight up or down at full speed; both
were so before 0.10.41 too.

**Next:** 0.10.47, Skyrim's seats.

## 2026-10-11 Test 0.10.45: two failures, and what the logs say about them

**Changed:** Nothing in the mod. Records only: the handover note, the plan at
the top of docs/DESIGN.md, this entry. Done by Claude Code.

**Why:** The user played 0.10.45 and reported: "A passed. B failed. There are
freezes, freezes dont seem to happen on a set interval. C did not attempt. D
passed. E passed. F conditional pass, enemy took different damage on short and
full draw - I can't see the arrow fly. I did not shoot at an enemy behind a
wall or shoot at the sky. G did not test - I don't have shouts yet, but I
think I hear the skyrim sound effect when the shout is on cooldown/no shout
equipped. H-L - WASD in menus work. Did not test continuous damage, health
potion or sensitivity. M - Runs better. I could not leave the building, I
tried this after all the tests above."

**Tested how:** `.\tools\collect-scene-logs.ps1`,
`.\tools\check-test-report.ps1`, then the three logs of
`logs\scene-report-20261011-184225` read line by line around each event. For
the building: the cell, its doors, locks and seats were read from Skyrim.esm
in the install folder with a throw-away script kept outside the project
(read-only; nothing from the game was copied anywhere).

**Result:**
- Confirmed by the logs: A; D (129 calm scans, 573 re-sent box sets; Skyrim's
  share 757 microseconds a frame, 1093 in the session before); E (items 5
  times, magic once, favourites 5 times, map once); F1 and F2 (7 shots: 6 hits
  at 177 to 321 units, 1 miss with nothing on the line; draw 0.32 to 1.0 and
  the damage followed it); H (41 direction keys in Skyrim menus, 1 repeat); M
  (Minecraft 120 frames a second throughout, Skyrim 59).
- 0.10.37 works: six changes of Skyrim cell without a hold, and the collision
  scan kept its number across them, so outdoor cells do share one physics
  world (that was an assumption until now). Skyrim let go 15 times, all for
  menus, loads and the start (33 before).
- G: the Z key reached Skyrim's player controls 8 times (4 more fell in the
  one-second spacing), and the user heard Skyrim's "nothing equipped" sound.
  The route works; an actual shout is not tested.
- **B failed.** Minecraft held the player 7 times for 50 to 100 ms, reason
  `outside_bounds`, all at the start of a jump with the player 1.5 to 1.75
  blocks above the middle of the cube. The four cubes named in those lines
  (445, 457, 486, 499) were aimed with a lead of exactly -1.75 on the height
  axis. The session before had no such hold. Cause: 0.10.41. Fixed in
  0.10.46 (entry above).
- **The building could not be left.** It is Honningbrew Meadery. At 18:37:46
  E was pressed on an object 123 units from the bar stool; Skyrim's camera
  left the player for 1.5 s (its character sitting down) and afterwards stayed
  6 units higher than before. From then on E on the inner door (9 times) and
  on the front door (once) reached Skyrim's own routine and did nothing; E on
  a character still opened dialogue. Going in through the same door had
  worked while standing. Read as: Skyrim's character was left seated, the
  bridge carried it around seated, and Skyrim does not let a seated character
  use a door. Fits every line; not proven, because the sit state was not
  logged. The first door outside that did nothing is the boilery's and needs a
  key.
- Side finding: in the meadery four of five collision scans ran at full rate
  although nothing moved and the number of boxes stayed 2,708; outdoors the
  calm rate was reached normally.

**Still broken / not tested:** Not tested by the user: C, F3 (wall), F4 (sky),
I, J, K; L not mentioned. They test these when they choose. The quicksave of
18:38:46 was made in the seated state.

**Next:** 0.10.46 to 0.10.48 (plan at the top of docs/DESIGN.md).

## 2026-10-11 Third batch: review, and the report checker brought up to date

**Changed:** No change to the mod after the re-read. tools\check-test-report.ps1
gained a movement section (cell borders crossed without letting go, times
Skyrim let go, holds by reason on the client and on the local server,
Minecraft's own movement complaints, overlaps and the pose during them, scans
thrown away), bow shots (sent, draw, hit or miss, reasons, distances), the
shout key's outcome, scans by pace and re-sent boxes, attack drops by reason,
and the four parts of Minecraft's cost. Version stays 0.10.45. Done by Claude
Code.

**Why:** Last step of the third batch plan: re-read everything from 0.10.37
to 0.10.45, and make the next report readable for the new steps.

**Tested how:** Read the whole batch's changes again against the plan: every
early exit of the per-frame function lets go by itself, so an unjudged cell
change cannot be left over; the "leave Skyrim's shapes out" flags are cleared
on every normal return and set again at the next call if one was missed; a
calm scan only sends boxes that a finished scan produced; a shot only reaches
the call that deals damage after the same checks as a sword hit. Ran the
checker on the last session's report (made by 0.10.36): it shows that
session's 33 let-go events, 24 "no_snapshot" holds, 15 server-side holds and
zero for everything new, which is the baseline the next report is compared
with. Checked the 0.10.45 packages (below).

**Result:** Nothing to correct. 45 Rust tests (39 unit, 5 connection, 1
position), 99 Minecraft-side rule checks and 136 hook checks (23 hook files)
pass. Packages Passthrough-Minecraft-0.10.45.zip and
Passthrough-Skyrim-0.10.45.zip are the build to install.

**Still broken / not tested:** Nothing from 0.10.37 to 0.10.45 has been
played. From 0.10.36 still untested: W, A, S, D in menus, continuous damage,
Skyrim healing; not exercised: sensitivity, height and depth, the wait key.

**Next:** The user runs "Test 0.10.45" at the top of README.md.

## 2026-10-11 Skyrim's shout or power on a key, experimental (0.10.45)

**Changed:** New key Z "Skyrim: shout or power" (SkyrimShortcuts.java,
en_us.json). Skyrim side, menu_input.rs: command 18 sends Skyrim's "Shout" key
press and release to Skyrim's player controls (CommonLib PlayerControls,
singleton Address Library ID 400864, resolved in skse.rs; its busy flags at
0x1D8 and 0x1D9 are respected) instead of its menu controls. Same guards as
the other play commands. Switch `shout=off`; it has its own switch, apart from
`menu_keys`. Versions 0.10.45. No packet change. Done by Claude Code.

**Why:** Shouts and powers are a large part of Skyrim and could not be used
from Minecraft at all.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Tests: the
command carries Skyrim's event name, is the only one routed to the player
controls, and is a play command (never sent into an open menu).

**Result:** 45 Rust tests, 99 rule checks, 136 hook checks pass. Packages
0.10.45 built.

**Still broken / not tested:** Not played, and nothing about this route has
been seen working: whether Skyrim's player controls act on a forwarded key at
all, and whether a quick press and release makes the shortest shout. A tap
only; holding for a longer shout is not possible. If the missing address is
reported at load (SCENE_MENU state=shout_unavailable) the key does nothing.

**Next:** Review of the batch.

## 2026-10-11 Bows hit Skyrim enemies (0.10.44)

**Changed:** Minecraft side: new mixin SkyrimBowMixin (BowItem.releaseUsing,
client side only, observes; Minecraft's own shot is unchanged).
SkyrimAttack.shot sends an attack with the reserved handle 0xFFFFFFFE and the
arrow's damage (BridgeRules.arrowDamage with checks: 6 at full draw, in
proportion below, a quarter more at full draw, Power counted, times 1.75;
nothing below a tenth of a draw or without an arrow). Skyrim side: attack.rs
`look` (Minecraft view direction to Skyrim axes), `ray_hits_body` (closest
approach of the view line and an upright body 128 units tall and 32 wide), and
`shoot`; world_export.rs `shot` picks the nearest living character on the line
among those Skyrim is simulating around the player, within 60 blocks, and
`blocked` casts one ray (the scan's own ray, layers and lock) for walls and
ground in between. Damage goes through the same call as a sword hit. Logs
SCENE_SHOT_SEND, SCENE_SHOT_RECV (dispatch, or miss with a reason),
SCENE_SHOT_RESULT. Switch `ranged=off`. Versions 0.10.44. No layout change.
Done by Claude Code.

**Why:** A Minecraft arrow flies through an empty Minecraft world and hits
nothing in Skyrim; bows were useless.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Rust tests:
the view direction for the four compass points, straight up and down; a shot
against bodies on, beside, above, below, behind and beyond the line, down a
slope, straight down, and two bodies in a row. Nine arrow-damage checks. The
bow's release logic, draw formula and the Power enchantment were read from the
26.3 jar and its data. The hook check covers the new hook (136 checks, 23
files).

**Result:** 45 Rust tests (39 unit, 5 connection, 1 position), 99 rule checks,
136 hook checks pass. Packages 0.10.44 built.

**Still broken / not tested:** Not played. No arrow is seen in Skyrim and the
shot does not drop: it lands where the crosshair points at release. Every
creature counts as a human-sized body, so a wolf is hit a little above itself
and a mammoth only near its feet. Friendly characters on the line are hit like
anyone else. Crossbows, tridents and thrown things do nothing yet. The wall
check uses the same ray as the collision scan, which is proven; using it for a
single long ray is new.

**Next:** 0.10.45: Skyrim's shout or power on a key.

## 2026-10-11 More Skyrim menus on the proven path (0.10.43)

**Changed:** New keys in SkyrimShortcuts.java: I opens Skyrim's items, P its
magic, G its favourites (names in en_us.json; registered through the existing
list). Skyrim side, menu_input.rs: command numbers 15 "Quick Inventory", 16
"Quick Magic", 17 "Favorites". Minecraft's "social interactions" action shares
P; in a bridge world its press is dropped while it shares a key with one of
these (SkyrimShortcuts.beforeKeybinds, called from the existing hook).
Versions 0.10.43. No packet change. A step of the third batch plan. Done by
Claude Code.

**Why:** Tab, M and J opening Skyrim's menus worked in play on 2026-10-11 (5 of
5 carried out). Items and magic go through the same Skyrim handler; favourites
through one of its own.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. The existing
tests cover the new numbers (one rule and one event each; the play-only rules)
and the name test now includes them.

**Result:** 43 Rust tests, 90 rule checks, 131 hook checks pass. Packages
0.10.43 built.

**Still broken / not tested:** Not played. Favourites uses a Skyrim handler
that has not been seen to accept a forwarded key; items and magic use the one
that has. `menu_keys=off` turns all of these off together with Tab, M and J.

**Next:** 0.10.44: bows hit Skyrim enemies.

## 2026-10-11 Where Minecraft's half millisecond goes (0.10.42)

**Changed:** PassthroughClient.java: the SCENE_COST side=minecraft line gains
send_us, world_us (receiving and collision), overlay_us (hand and HUD capture)
and blocks_us (block capture), each the average per frame over the five
seconds. Versions 0.10.42. No behaviour change. Done by Claude Code.

**Why:** The first numbers gave one figure for Minecraft (about 0.5 ms per
frame at 120 frames a second) with no way to tell which part it is.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`.

**Result:** Builds; all checks pass. Packages 0.10.42 built.

**Still broken / not tested:** Not played; the numbers come with the next
report.

**Next:** 0.10.43: more Skyrim menu keys.

## 2026-10-11 The collision cube looks further ahead at high speed (0.10.41)

**Changed:** Skyrim side, world_export.rs `aim`: a scan may now be centred up
to 2.75 blocks ahead of the player sideways (was 1.5) and 1.75 up or down (was
0.75). Versions 0.10.41. No packet change. An addition to the third batch plan,
made because 0.10.40 had just turned this calculation into a testable function.
Done by Claude Code.

**Why:** The old limits were already reached at sprinting speed (5.6 blocks a
second times 0.27 s is 1.51 blocks), so anything faster outran the 8-block
cube: sprint-jumping, a speed effect, flying, a long fall. This is the real
"8 block limit" the user named. The logs show it twice: flying at 20 blocks a
second, and falling at 14 with the player 0.05 block outside the cube.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. The test
checks the limits and, for seven speeds up to 30 blocks a second and four
positions on the grid, that the player with the room Minecraft asks for is
inside the cube and within the 3 blocks beyond which a scan is thrown away.

**Result:** 43 Rust tests, 90 rule checks, 131 hook checks pass. Packages
0.10.41 built.

**Still broken / not tested:** Not played. Walking and sprinting are unchanged
(their look-ahead was below the old limit or a hundredth of a block above it).
At 20 blocks a second the cube is still left for part of the time; a scan
takes about 0.12 s and the cube is 8 blocks. Turning round at such a speed
puts the player at the back edge of the cube for one scan.

**Next:** 0.10.42: where Minecraft's half millisecond goes.

## 2026-10-11 Standing still costs a quarter (0.10.40)

**Changed:** Skyrim side, world_export.rs. When a finished collision scan has
the same centre and the same boxes (checksum) as the scan before it, the next
scan is "calm": it casts a quarter of the rays per frame (half when frames are
longer than 20 ms, full when longer than 40 ms, so it still finishes in about
half a second). While a calm scan runs, the last finished boxes are sent again
every 125 ms with fresh characters, health, hits and crosshair. A calm scan is
dropped and a full-rate one started the moment the player's half-block cell or
speed changes, and scanning returns to full rate when a calm scan finds
different boxes. The scan's centre is now worked out by a pure function `aim`.
world.rs: the box checksum became a shared function. New log line
SCENE_WORLD_RESEND; SCENE_WORLD_SCAN gains `pace=calm|full`. Switch
`slow_scan=off`. Versions 0.10.40. No packet change. Step 4 of the third batch
plan. Done by Claude Code.

**Why:** First in-game cost numbers (report 20261011-155446): the bridge takes
about 1.1 ms of every Skyrim frame, nearly all of it this ray casting, moving
or not.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New tests:
the pacing (full rate when not calm at any frame time; a quarter at 60 frames a
second; a calm scan of 2,700 casts finishes within 500 ms at 16 and 33 ms
frames; re-send every 125 ms only when calm) and the centre calculation against
a logged case and against sprinting, falling and the look-ahead limits, so the
extraction did not change it.

**Result:** 43 Rust tests (37 unit, 5 connection, 1 position), 90 rule checks,
131 hook checks pass. Packages 0.10.40 built.

**Still broken / not tested:** Not played; the saving is expected (about 0.3 ms
instead of 1.1 ms per frame while standing), not measured. SCENE_COST and
`pace=` in the next report will show it. A door or anything else that moves
while the player stands still is now seen after about half a second instead of
an eighth. While moving, nothing changes.

**Next:** An added step, 0.10.41: the collision cube looks further ahead at
high speed (the real "8 block limit" case, seen once in the logs while flying).

## 2026-10-11 Skyrim's shapes never force a crouch or crawl; bodies are not solid (0.10.39)

**Changed:** Minecraft side: new mixin BridgePoseMixin (Player
.canPlayerFitWithinBlocksAndEntitiesWhen): in a bridge world Skyrim's shapes
are left out while Minecraft asks whether the player fits a pose, so they can
no longer force a crouch or a crawl; Minecraft's blocks still count and the
sneak key is unchanged. SkyCollision.reportOverlap logs `event=SCENE_OVERLAP
terrain= characters= pose= sneak_key= on_ground=` at most once a second while
Skyrim shapes overlap the standing player. Skyrim side: dead or dying
characters get no stand-in box (attack.rs `dead`, shared with the crosshair
classification; world_export.rs counts them as bodies_left_out in
SCENE_WORLD_SCAN). Switch `pose_free=off`. Versions 0.10.39. Step 3 of the
third batch plan. Done by Claude Code.

**Why:** User: "Sometimes something also seems to block me, I can move out of
whatever it is blocking me but the movement is very slow and I cannot jump."
Very slow and unable to jump is what Minecraft's forced crawl feels like, and a
stand-in box overlapping the upper body forces it. A body on the ground was an
invisible pillar 1.8 blocks high.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. The hooked
function and the pose logic were read from the 26.3 jar (Player.updatePlayerPose
picks crouching or crawling when canPlayerFitWithinBlocksAndEntitiesWhen fails
for standing). The hook check now passes 131 checks over 22 hook files.

**Result:** 41 Rust tests, 90 rule checks, 131 hook checks pass. Packages
0.10.39 built.

**Still broken / not tested:** Not played, and the cause is argued, not
measured: the problem was not in the one logged session. The new SCENE_OVERLAP
line is there to show it next time. A consequence to know: under a really low
Skyrim ceiling the player now stands up into it instead of staying crouched.
Stand-ins are still one human-sized box for every living creature, large or
small.

**Next:** Step 4, 0.10.40: standing still costs a quarter.

## 2026-10-11 The local server trusts the client about Skyrim's shapes (0.10.38)

**Changed:** Minecraft side. SkyCollision gained a per-thread "leave Skyrim's
shapes out" flag. BridgeServerMoveMixin sets it while the local server handles
one of the player's reported steps (handleMovePlayer) and no longer drops
steps; BridgeMotionMixin holds only the client's player. Switches added for
this batch: `server_trust`, `pose_free`, `slow_scan`, `ranged`, `shout` (Rust
and Java lists, same order). `server_trust=off` restores the old behaviour.
Versions 0.10.38. Step 2 of the third batch plan. Done by Claude Code.

**Why:** Same user report as 0.10.39. Read from the 26.3 jar: the server sends
the player back when the reported position touches a collision shape the old
position did not (isEntityCollidingWithAnythingNew; silent, also in creative
mode), and in survival when its own replay of the step ends more than a
quarter block away ("moved wrongly"). Skyrim's shapes are part of both checks
and change under the player seven times a second. The bridge's own server-side
hold dropped reported steps while the client kept moving, which sets up the
same correction.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`; hook checks
pass (the second hook on handleMovePlayer is counted).

**Result:** 41 Rust tests, 90 rule checks, 125 hook checks pass at this step.
Packages 0.10.38 built.

**Still broken / not tested:** Not played. The session's log has no "moved
wrongly" line (the collector keeps them from now on), so this cause is argued
from the code. What changes on purpose: the server no longer double-checks the
player against Skyrim's ground; only the client does. Dropped items and other
server-side things still collide with Skyrim's shapes as before.

**Next:** Step 3, 0.10.39.

## 2026-10-11 No pause at Skyrim cell borders (0.10.37)

**Changed:** Skyrim side. skse.rs: a change of cell no longer lets go of the
character at once. It is judged once that frame's Minecraft position is known
(`stays_steered`): the next outdoor cell of the same world, with the character
within 140 units of where Minecraft says it is, keeps everything running;
anything else lets go as before. world_export.rs: a collision scan restarts
only when Skyrim's physics world changed, not on every cell; new `same_place`
(same cell, or two outdoor cells of one world), used for which characters get a
stand-in and (attack.rs) which enemy a hit may land on. tools: the log
collector now also keeps Minecraft's own "moved wrongly" and "moved too
quickly" lines. Versions 0.10.37. No packet change; protocol 31. Step 1 of the
third batch plan. Done by Claude Code.

**Why:** User: "when I'm moving fast the 8 block limit seems to block me and
there is a small pause in movement before it resyncs again." The logs of
report 20261011-155446 show it is not the collision cube (left once in the
session, by 0.05 block, while falling). Every short hold while moving sits on a
change of Skyrim cell (58.5 blocks apart): the bridge let go, sent Minecraft an
empty collision packet, and Minecraft held the player about 0.1 s until a new
scan arrived. Along a border it repeated (cells 9638 and 9639 five times in ten
seconds). One sword hit in that session was refused only because the enemy
stood across a border.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New tests:
the decision for every kind of cell change (next outdoor cell, fast travel in
the same world, doors between worlds and buildings, unknown distance); the
"same place" rule; and the simulated-player test now walks from one outdoor
cell into the next (not let go, same origin, still 70 units per block) and
then through a door into another world (let go, recovery asked).

**Result:** 41 Rust tests (35 unit, 5 connection, 1 position), 90
Minecraft-side rule checks, 124 hook checks pass. Packages 0.10.37 built.

**Still broken / not tested:** Not played. Assumed and not verified in game:
that the outdoor cells of one world share one physics world (if they do not,
only the scan restarts at a border, as before, but Minecraft keeps its last
collision and is not held). At creative flying speed (20 blocks a second, seen
once in the logs) the 8-block collision cube really can be outrun; not changed
here. The log line to look for is SCENE_ALIGNMENT state=cell_crossed with no
"no_snapshot" hold beside it.

**Next:** Step 2, 0.10.38: the local server trusts the client about Skyrim's
shapes.

## 2026-10-11 0.10.36: the user's answers; no commit

**Changed:** Records only (this entry, README.md, STATUS.md, two Backlog lines
in docs/DESIGN.md). Done by Claude Code.

**Why:** Asked after the test, the user answered: "Question 1: Don't commit. I
will not test anything again the 'Not confirmed' until later. Question 2: Not
tested. Question 3: Not tested."

**Tested how:** Nothing new was run.

**Result:** E (W, A, S, D and held directions in Skyrim menus) and J (Skyrim
healing) are now recorded as not tested, not as unexplained: the absence of
their log lines is simply because they were not tried. H stays not tested.
Nothing is committed, by the user's decision; the work from 0.10.19 to 0.10.36
stays uncommitted on `dev`.

**Still broken / not tested:** E1, E2, H, J not tested; F3, K2, K3 and the
wait key not exercised. Confirmed by play and logs: A, B, C, D, F1, F2, G, I,
K1, L, M (entry below).

**Next:** Nothing is in progress. The user will test the rest later, or name
the next step.

## 2026-10-11 Test of 0.10.36: user result and what the logs show

**Changed:** Records only (this entry, README.md, STATUS.md, the Backlog in
docs/DESIGN.md). No code or build change. Done by Claude Code.

**Why:** The user played 0.10.36 and reported: "Pass: A,B,C,D,E,F,G seems to
work, look at logs,I,K,L,M fps is definitely higher(was 50 now 60). Did not
test: H". J was not mentioned.

**Tested how:** `.\tools\collect-scene-logs.ps1`, then
`.\tools\check-test-report.ps1` on report scene-report-20261011-155446 (one
session, 15:42 to 15:53), then the individual lines behind each count.

**Result:** Protocol 31 on both sides, no error or "disabled" line, no
switches file used. Per letter:
- A, B: confirmed. 4 quicksaves (2 more refused by the one-second spacing), 2
  quickloads, 6 activations with E all carried out, 30 real clicks, take-all
  once. R was not used to open anything in this session.
- C: confirmed. Crosshair kinds seen: living 27, **enemy 31**, body 3, door 1,
  object 2. So Skyrim's hostility test answers and does not crash.
- D: confirmed. Character menu twice, map twice, journal once: all 5 carried
  out by Skyrim. The wait key was not tried.
- E: **not shown by the logs.** No W, A, S or D command and no repeat was sent
  in the whole session; no arrow key either. Every Up and Down (9) came from
  the mouse wheel. Reading Minecraft's key code again found no reason the
  presses would be missed, so most likely they were not pressed inside a
  Skyrim menu. Open until the user says which.
- F: the monitor part is confirmed (window 395 wide, measured against a
  1920-wide monitor in all 108 pointer lines). The sensitivity part was not
  exercised: the factor was 1.0 throughout.
- G: confirmed to the digit. Wooden sword 7.0; with Sharpness V 12.25
  ((4+3) x 1.75), critical 15.75; with Sharpness X 16.625; with Sharpness V
  and Strength II 22.75 ((4+6+3) x 1.75). 99 of 107 attacks carried out.
- H: not tested. Ordinary hits: 23 pooled, 21 deliveries, 9 taken; all 12
  refusals had a raised shield. No hit arrived inside Minecraft's hit pause
  (the counter read 0 every time), so the rule 0.10.27 corrected is still
  unobserved in play.
- I: confirmed. Two pauses cancelled 1 and 4 hits (4.4 to 41.3 Skyrim damage);
  no hit reached Skyrim's health ("not_ready" count 0, was 113 before).
- J: no evidence either way. Skyrim reported no positive health change at all
  in the session.
- K: fast travel from the map is confirmed for the first time (map opened
  15:43:52, two clicks, arrival 182 blocks away at 15:43:58). The height and
  depth parts were not exercised: all 6 arrivals were inside the building
  range and the player stayed between y 36.5 and 42.1.
- L: confirmed. One Minecraft death: Skyrim's character killed, reload,
  cleared.
- M: Skyrim ran at 59 frames a second (296 frames per 5 s); the user reports
  50 before. Raw logs for 11 minutes: 25 MB and 37 MB (the previous 5-minute
  session wrote 430 MB each).
First real cost numbers (0.10.34): the bridge takes about 1.1 ms of each
Skyrim frame on average (worst frames about 2 ms), nearly all of it ray
casting for collision (about 7.4 ms per scan spread over 7 frames). In
Minecraft about 0.5 ms per frame on average at 120 frames a second, with one
83 ms spike.

**Still broken / not tested:** H (continuous damage), J (Skyrim healing), E
(W, A, S, D and held directions), F3 (sensitivity), K2 and K3 (height and
depth), the wait key. Not committed: the user has not said that all the
testing passed (AGENTS.md rule 3); asked.

**Next:** The user's answers on E and J, the H test when convenient, and
whether to commit now with those marked untested.

## 2026-10-11 Test list 0.10.36: one dropped check put back

**Changed:** README.md only: "Test 0.10.36" gained step B4 (E on a chest or
body, then left click one item). Done by Claude Code.

**Why:** The user asked whether the new list includes the tests of "Test
0.10.27". Compared step by step: everything is there under new letters except
this one check from the old letter H, which had been lost when the R and E
container checks were merged.

**Tested how:** Listed the steps of both sections side by side and matched
each old step to a new one.

**Result:** Old A1-A5 are C1-C5; A6 is B3; B1, B3, B4 are D1, D2, D4; B2 is
D3 plus K1; C is H; D is I; E is G; F is F1 and F2; G is K2; H is A, B1, B2
and the new B4; I is M.

**Still broken / not tested:** Nothing from 0.10.19 to 0.10.36 has been
played. No code or build change.

**Next:** The user runs the test list at the top of README.md.

## 2026-10-11 Review correction: Skyrim logs the switches at load (0.10.36)

**Changed:** skse.rs reads switches.txt as the plugin loads. Versions 0.10.36.
Last step of the second batch. Done by Claude Code.

**Why:** Re-reading the second batch: on the Skyrim side the file was only read
the first time a switched feature was needed, so the SCENE_SWITCHES line could
appear late or never, and a report could not tell which steps were off.
Nothing else was found to correct in this pass.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Checked the
packages: the jar holds 66 classes including BridgeRules, Switches,
SkyrimShortcuts and the new hook, its hook list names BridgeVoidMixin, and the
library in both packages is the same file and contains the new entry point and
log names.

**Result:** 39 Rust tests (33 unit, 5 connection, 1 position), 90
Minecraft-side rule checks and 124 hook checks pass. Packages
Passthrough-Minecraft-0.10.36.zip and Passthrough-Skyrim-0.10.36.zip built.
This is the build to install.

**Still broken / not tested:** Nothing from 0.10.19 to 0.10.36 has been played.

**Next:** The user runs the test list at the top of README.md.

## 2026-10-11 A checker for the test report (0.10.35)

**Changed:** New tools\check-test-report.ps1. It reads a collected report (the
newest unless one is named) and prints, feature by feature, what the logs
show: protocol and connection, switches, error and "disabled" lines, crosshair
kinds, activations by key, menu commands by number, source and outcome, pooled
hits and deliveries, pause protection, restored health, attacks with Sharpness
or effects, pointer span and factor, arrivals, death handling, cost per frame,
and the size of the newest raw logs. Versions 0.10.35. No code change in the
mod. Step 8 of the second batch plan. Done by Claude Code.

**Why:** Sixteen steps will be judged from one report. Whichever agent reads
it, and the user, should get the evidence per feature without composing
searches by hand.

**Tested how:** Ran it on the existing report 20261011-094530 (made by
0.10.16). It reproduced the numbers already recorded for that session (115
menu commands dispatched, 132 hits handed over, 16 of 110 taken, 113 hits left
to Skyrim as not_ready, 102 attacks of which 37 critical, 9 recoveries) and
showed "none" for everything newer.

**Result:** Works on an old report. Packages 0.10.35 built.

**Still broken / not tested:** It has not seen a report from this version, so
the lines for the new features are checked against the code that writes them,
not against real output. It shows evidence, not whether something felt right.

**Next:** Review of the whole second batch.

## 2026-10-11 Measure what the bridge costs per frame (0.10.34)

**Changed:** New passthrough/src/cost.rs (a small adder with a test). skse.rs
times the bridge's own work inside Skyrim's player update and logs
`event=SCENE_COST side=skyrim frames= avg_us= max_us= period_ms=` every five
seconds. world_export.rs adds `cast_us=` (time spent casting rays) and
`scan_frames=` to each SCENE_WORLD_SCAN line. PassthroughClient.java times its
after-render work and logs the same SCENE_COST line with side=minecraft.
Versions 0.10.34. No behaviour or packet change. Step 7 of the second batch
plan. Done by Claude Code.

**Why:** "Does the bridge slow the games, and where" could only be guessed.
The one thing measurable offline was measured for the plan (handing Minecraft
the collision packet: about 80 microseconds a frame, left alone). The rest
needs numbers from inside the games.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Unit test for
the adder: one summary per period with the right average and worst frame, then
a fresh start.

**Result:** 39 Rust tests (33 unit, 5 connection, 1 position), 90
Minecraft-side rule checks and 124 hook checks pass. Packages 0.10.34 built.

**Still broken / not tested:** Not played, so no in-game number exists yet.
Not covered by these timers: Minecraft's collision queries during movement
(inside its physics, not after rendering) and the drawing of Minecraft blocks
and hands inside Skyrim's renderer.

**Next:** Step 8, 0.10.35: a checker for the test report.

## 2026-10-11 No falling out of the world in the bridge world; any depth; hook checks (0.10.33)

**Changed:** New mixin BridgeVoidMixin (listed in passthrough.mixins.json): in
a bridge world the player is exempt from Minecraft's "fell out of the world"
damage (LivingEntity.onBelowWorld); every other world and creature keeps it.
BridgeRules.arrival now allows arrival down to 2,000 blocks below the floor
when that exemption is on, and 56 when it is switched off (`void_guard=off`);
BridgeRecovery passes the switch. Tooling added in the same step because the
new hook needed it: minecraft\diagnostics\MixinTargetsCheck.java checks every
hook the mod places into Minecraft against the class files of the Minecraft
jar, and tools\test-java-rules.ps1 (so every build) runs it. Versions 0.10.33.
Step 6 of the second batch plan. Done by Claude Code.

**Why:** 0.10.25 had to stop arrivals 56 blocks below Minecraft's floor because
of that damage. In a bridge world the ground is Skyrim's at any depth and the
player is held whenever it is not known, so the rule only punished low places
(deep water, deep caves, interiors built far below Skyrim's zero height).

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. The hooked
function was read from the 26.3 jar: LivingEntity declares `protected void
onBelowWorld()` and it only calls hurt with the fell-out-of-world damage.
Experiment: with the hook's target deliberately misspelt the build still said
BUILD SUCCESSFUL, so the build does not protect against this. The new hook
check does: on the real sources it passes 124 checks over 21 hook files; on a
scratch copy with five planted mistakes (misspelt method, field, accessor,
call descriptor, and a listed file missing) it reported every one and exited
with an error. Five new rule checks for the two depth limits.

**Result:** 38 Rust tests, 90 Minecraft-side rule checks and 124 hook checks
pass. Packages 0.10.33 built.

**Still broken / not tested:** Not played. The hook check proves the targets
exist with the names (and, where written, the types) the hooks use. It does not
prove a hook's own parameters are right, or that it behaves as intended; the
new hook takes none beyond the standard one. Arrival deeper than 2,000 blocks
below the floor is still refused.

**Next:** Step 7, 0.10.34: measure what the bridge costs per frame.

## 2026-10-11 Pointer speed follows Minecraft's mouse sensitivity (0.10.32)

**Changed:** SkyrimMenus.java multiplies pointer travel by
BridgeRules.pointerFactor: twice Minecraft's Mouse Sensitivity setting, so
100% leaves 0.10.24's speed, 200% doubles it, 50% halves it, never below a
quarter. SCENE_CURSOR_SEND logs factor=. With `pointer_monitor=off` the factor
is 1 and the mapping is exactly the one accepted in 0.10.18. Versions 0.10.32.
No native or packet change. Step 5 of the second batch plan. Done by Claude
Code.

**Why:** 0.10.24 fixed the pointer at one Skyrim pixel per mouse unit and left
no way to change it.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`; six new rule
checks. Options.sensitivity() and its 0-to-1 range were read from the 26.3 jar.

**Result:** 38 Rust tests and 85 Minecraft-side checks pass. Packages 0.10.32
built.

**Still broken / not tested:** Not played. The setting also turns the camera
faster or slower, as it always did; the two cannot be set apart yet.

**Next:** Step 6, 0.10.33: no falling out of the world in the bridge world.

## 2026-10-11 W, A, S, D and held directions in Skyrim menus (0.10.31)

**Changed:** SkyrimMenus.java: while a Skyrim menu is open, Minecraft's four
movement keys send the same Up, Down, Left and Right commands as the arrow
keys, and a direction that stays held repeats (after 0.4 s, then every 0.12 s;
BridgeRules.repeatDue with checks). A direction repeats only if it was pressed
while that menu was open, and stops when the menu changes. Switch
`menu_wasd=off` turns both off. Versions 0.10.31. No native or packet change.
Step 4 of the second batch plan. Done by Claude Code.

**Why:** In Skyrim's own window W, A, S and D move through menus; here only
the arrow keys and the wheel did, and long lists needed one press per line.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Six new
Minecraft-side checks, one of them counting that a second of holding gives the
press plus six repeats.

**Result:** 38 Rust tests and 79 Minecraft-side checks pass. Packages 0.10.31
built.

**Still broken / not tested:** Not played. The key handling around the rule
(which presses count, emptying the movement keys' press queues every frame) is
checked by reading only. Logged as SCENE_MENU_SEND source=move_key or
source=repeat. Enter and Backspace do not repeat.

**Next:** Step 5, 0.10.32: pointer speed follows Minecraft's mouse sensitivity.

## 2026-10-11 Skyrim healing heals your Minecraft hearts (0.10.30)

**Changed:** Skyrim side, damage.rs: a positive health change reported by the
function that already reports hits is queued for Minecraft as "restore this
share of full health", unless it is only Skyrim's natural recovery (pure
function `restore`: at least 0.5% of full health in one go is forwarded at
once; smaller changes are added up for a quarter second and forwarded only
above 2.5% per second). Skyrim's own health is left as the game computes it.
world.rs: an event's Skyrim amount may now be negative (restored); zero is
still refused. Minecraft side, BridgeDamage.java: such an event heals the
player by that share of maximum health at once, outside the damage pool;
BridgeRules gained validHealthEvent and isRestore with checks. Switch
`healing=off`. Protocol 31, name link31 (lib.rs, windows.rs). Versions 0.10.30.
Step 3 of the second batch plan. Done by Claude Code.

**Why:** Skyrim potions, food and healing spells did nothing useful: Minecraft
owns the health and never heard of them.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New Rust
tests: a potion and a bite of food are forwarded at once; natural recovery as
logged in report 20261011-094530 (0.0083 per frame) is not, nor three times
that, nor at 20 frames a second; a healing spell of 10 a second for two
seconds forwards between 17 and 20.9 of its 20, also on a character with 300
health; nonsense values forward nothing. Packet test: restored health survives
encoding and decoding with its sign; six malformed events are refused. Nine new
Minecraft-side checks.

**Result:** 38 Rust tests (32 unit, 5 connection, 1 position) and 73
Minecraft-side checks pass. Packages 0.10.30 built.

**Still broken / not tested:** Not played. The one thing nobody has seen:
whether Skyrim calls that function at all for a potion drunk at full Skyrim
health, which is the normal case here. If it does not, nothing is forwarded
and there is no SCENE_HEAL line; the existing SCENE_DAMAGE_CLAMP line will show
whether the call happened. Natural recovery stronger than about 2.5% of full
health per second (heavy recovery bonuses) would be forwarded as healing.
Waiting, sleeping and levelling up are expected to heal as they do in Skyrim.

**Next:** Step 4, 0.10.31: W, A, S, D and held directions in Skyrim menus.

## 2026-10-11 Checks for the Minecraft side before every build (0.10.29)

**Changed:** New BridgeRules.java: the Minecraft side's own arithmetic and
yes/no rules, with nothing from Minecraft in it (attack damage with recharge,
critical, Sharpness, Strength and Weakness; what E does per crosshair kind;
when a pooled hit may be delivered and when a pool is too old; arrival
heights; what the pointer is measured against). SkyrimAttack, BridgeDamage,
BridgeRecovery and SkyrimMenus now call it instead of repeating the numbers.
New minecraft\diagnostics\RulesCheck.java with 64 worked examples, new
tools\test-java-rules.ps1, and tools\build-step5b.ps1 runs it after the Rust
tests and stops on a failure. Versions 0.10.29. No behaviour change intended.
Step 2 of the second batch plan. Done by Claude Code.

**Why:** The Minecraft side had no automated tests; 0.10.21 shipped a wrong
rule that only reading found.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly` printed
"PASS: 64 Minecraft-side rule checks." between the Rust tests and the build.
The failure path was tried on a copy in the scratch folder with one expectation
made wrong: "FAIL fist: expected 2.0, got 1.75 / FAILED: 1 of 64", exit code 1,
which the script turns into a stopped build. The examples include the accepted
0.10.5 numbers (wooden sword 7, fist 1.75), so the refactor cannot have moved
them unnoticed.

**Result:** 36 Rust tests and 64 Minecraft-side checks pass. Packages 0.10.29
built.

**Still broken / not tested:** Not played. These checks cover the bridge's own
numbers and decisions only. They cannot catch a wrong assumption about how
Minecraft itself behaves, which is what was wrong in 0.10.21; that still needs
the game's code to be read (AGENTS.md).

**Next:** Step 3, 0.10.30: Skyrim healing heals your Minecraft hearts.

## 2026-10-11 Switches: turn one step off without a rebuild (0.10.28)

**Changed:** New passthrough/src/switches.rs: reads `switches.txt` from the
project folder once per game (lines `name=off`, `#` comments), logs
SCENE_SWITCHES with what is off and what it did not understand. New entry point
`pt_mc_switches` (native.rs) hands the set to Minecraft; new Switches.java
holds it. Used so far: `hostility` (attack.rs), `pause_shield` (damage.rs),
`e_precedence` (SkyrimAttack), `menu_keys` (SkyrimShortcuts), `damage_pool`
(BridgeDamage, which got a small `deliver` helper so "off" is one Minecraft hit
per Skyrim hit as before 0.10.21), `pointer_monitor` (SkyrimMenus). Names
reserved for later steps of this batch: `healing`, `menu_wasd`, `void_guard`.
Also: a timing check in lib.rs that does nothing in a normal test run.
Versions 0.10.28. No packet change; protocol 30. Step 1 of the second batch
plan. Done by Claude Code.

**Why:** Sixteen unplayed steps will be tested in one session. If one of the
unverified ones misbehaves or crashes a game, the user can switch that one off
and carry on, instead of waiting for a rebuild.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Unit tests
for the parser (comments, spaces, capitals, later line wins, misspelt names
reported) and for the bit order shared with Java. End to end on the built
library: loaded dist\step5b\minecraft\mods\passthrough_link.dll from a small
Python script and called `pt_mc_switches`: no file gave 0; a temporary
switches.txt with `hostility=off`, `void_guard = OFF`, `menu_keys=on`,
`nonsense=off` gave bits 1 and 8, and the log line `off=["hostility",
"void_guard"] not_understood=["nonsense=off"]`. The temporary file was removed.
Timing run for the plan: collision packet receive plus encode, 1,500 boxes,
release build, 79.7 microseconds per frame.

**Result:** 36 Rust tests pass (30 unit, 5 connection, 1 position). Packages
0.10.28 built.

**Still broken / not tested:** Not played: that each "off" really restores the
earlier behaviour in the games is checked only by reading. The file is read
once at start, so a change needs both games restarted. A build warning
(`HEALING` not used yet) goes away in 0.10.30.

**Next:** Step 2, 0.10.29: checks for the Minecraft side before every build.

## 2026-10-11 Review correction: the damage pool watched the wrong counter (0.10.27)

**Changed:** BridgeDamage.java only: the pool from 0.10.21 now waits on
`LivingEntity.damageCooldownTime`, the counter Minecraft 26.3 really uses for
its half-second hit pause. Versions 0.10.27. Also corrected in place, before
any commit: the 0.10.21 entry below and the 0.10.21 part of docs/ABI.md, which
named the wrong counter. Last step of the batch. Done by Claude Code.

**Why:** Re-reading the whole batch after the last build, with the bytecode of
LivingEntity.hurtServer printed in full this time, showed that 0.10.21 as built
compared `Entity.getInvulnerableTime()` with 10. In 26.3 that is a separate
protection that is normally zero, so the pool would have been delivered at once
every time and small hits would still have been refused: 0.10.21 would have
changed nothing. My first check of the bytecode filtered the lines by words I
expected, and the line naming the real field did not contain them, so I assumed
the name instead of reading it. Recorded as a mistake of method: print the
code, do not search it for what you expect to find.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Read with
javap: hurtServer loads `damageCooldownTime`, compares it with 10.0, refuses an
amount not above `lastHurt`, otherwise applies the difference; outside the
pause it sets the counter to 20. The field is public. Also checked this time
from the game's own data rather than from memory: Sharpness is 1.0 plus 0.5 per
level above the first (data/minecraft/enchantment/sharpness.json), Strength is
+3.0 and Weakness -4.0 attack damage (MobEffects).

**Result:** 33 Rust tests pass (27 unit, 5 connection, 1 position). Packages
Passthrough-Minecraft-0.10.27.zip and Passthrough-Skyrim-0.10.27.zip built.
This is the build to install for the combined test.

**Still broken / not tested:** Nothing in the batch (0.10.19 to 0.10.27) has
been played. The Minecraft side still has no automated tests, which is why this
mistake could only be found by reading.

**Next:** The user runs the combined test list at the top of README.md.

## 2026-10-11 The bridge's own log writing (0.10.26)

**Changed:** native.rs: each log call is now written to the file as one piece
(one system call) instead of about seven per line. world.rs: a collision
snapshot logs one WORLD line with `boxes_sum`, a checksum of every box, the
same on the sending and receiving side; the line per box is written only when
the environment variable PASSTHROUGH_LOG_BOXES=1 is set before the games
start. Character (NPC) lines are unchanged. tools\compare-world-logs.ps1
accepts both forms. New tools\prune-logs.ps1 lists old raw logs and removes
them only with -Delete. Versions 0.10.26. No layout change; protocol 30. Step 8
of the batch plan. Done by Claude Code.

**Why:** Measured on 2026-10-11: logs\step2 holds 34 GB; one session wrote
2.6 GB per game. In the last session 1,678,181 of 1,724,764 Skyrim lines (97%)
were one line per collision box, about 7,000 a second, written on Skyrim's
player-update thread and mirrored on Minecraft's render thread.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New unit
tests: a snapshot of 1,000 boxes logs three lines by default and 1,000 box
lines when asked; the checksum survives encoding and decoding and changes when
one coordinate changes; a log call becomes one piece of text. The log written
by the test run itself shows the new line (`event=WORLD_SEND ... boxes=0 npcs=0
boxes_sum=CBF29CE484222325`) and no box lines. compare-world-logs.ps1 was run
on four small hand-made log pairs: new form matching (pass, "1000 by
checksum"), new form with a different checksum (refused), old form with every
box (pass), old form with a box missing (refused). prune-logs.ps1 was run
without -Delete: "Would remove 129 raw log files, 27.8 GB ... Would keep the
newest 2 per game: 4 files, 5.6 GB. Nothing was deleted."

**Result:** 33 Rust tests pass. Packages 0.10.26 built.

**Still broken / not tested:** Not played, so the effect on frame rate is not
measured; from here it cannot be. Expected: raw logs about 97% smaller, and
both games' main threads stop making tens of thousands of file writes a second.
The Minecraft client log still gets six lines per block capture (about 55 a
second); left as it is. The 34 GB already on disk stays until the user runs the
prune script.

**Next:** Review of the whole batch, then the combined test list.

## 2026-10-11 Arriving at any height (0.10.25)

**Changed:** BridgeRecovery.java only. The teleport that follows a door, a
load or fast travel used to be refused when its target lay outside Minecraft's
building range (y -64 to 319). It is now accepted from 56 blocks below the
floor to 2,000 blocks above the ceiling. Arriving outside the building range
shows one chat line saying that blocks cannot be placed there, and
SCENE_RECOVERY state=received logs buildable=, min_y= and max_y=. Versions
0.10.25. No native or layout change. Step 7 of the batch plan. Done by Claude
Code.

**Why:** With the user's saved origin the old check kept Skyrim heights below
about -11,600 (sea level is -14,000) and above about 15,000 (the mountains)
out of reach. Reading the code showed this check was the only place the limit
was enforced: walking was never limited, because the ground comes from
Skyrim's collision and not from Minecraft blocks, and a Minecraft player can
stand outside the building range.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Minecraft's
rule for falling out of the world was read from the 26.3 jar with javap
(Entity.checkBelowWorld: only below the floor minus 64), which is why arrival
stops at 56 below.

**Result:** 31 Rust tests pass. Packages 0.10.25 built.

**Still broken / not tested:** Not played. At home the reachable Skyrim heights
become about -15,500 to 155,000. Blocks still cannot be placed or kept outside
Minecraft's own range; Minecraft shows its own height-limit message there.
Walking or diving below about Skyrim -16,000 (deep sea floor) would take
Minecraft's void damage; not guarded. Unknown until played: whether anything
else in the bridge misbehaves outside the building range (block drawing has
nothing to draw there; nothing else was found by reading).

**Next:** Step 8 of the batch, 0.10.26: the bridge's own log writing.

## 2026-10-11 Pointer speed independent of the Minecraft window size (0.10.24)

**Changed:** SkyrimMenus.java only. Mouse travel for Skyrim's menu pointer is
now measured against the size of the monitor the Minecraft window is on
(Window.findBestMonitor, Monitor.currentMode; re-read at most once a second)
instead of the window itself. If the monitor cannot be read, or is smaller than
the window, the window size is used as before. SCENE_CURSOR_SEND logs span_w=
and span_h=. Versions 0.10.24. No native or layout change. Step 6 of the batch
plan. Done by Claude Code.

**Why:** Backlog item deferred by the user. The logs show three window widths
across sessions (854, 682, 372) against a 2560-wide Skyrim screen, so the same
hand movement moved the pointer about 3, 3.8 and 6.9 Skyrim pixels per mouse
unit depending on how the window happened to be sized.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`; function
names checked against the 26.3 jar with javap.

**Result:** 31 Rust tests pass. Packages 0.10.24 built.

**Still broken / not tested:** Not played. Expected: with Skyrim at the
monitor's resolution, one Skyrim pixel per mouse unit at every window size, so
the pointer will feel clearly slower than in the last sessions (about a third
of the 854-wide speed). A full-screen Minecraft keeps its old speed. No speed
setting exists yet; if this is too slow, that is the next thing to add.

**Next:** Step 7 of the batch, 0.10.25: arriving at any height.

## 2026-10-11 Sharpness, Strength and Weakness against Skyrim enemies (0.10.23)

**Changed:** SkyrimAttack.java only. The damage asked of Skyrim now includes,
with Minecraft's own numbers: Sharpness (0.5 per level plus 0.5), the Strength
effect (plus 3 per level) and the Weakness effect (minus 4 per level, never
below zero). As in Minecraft's own attack, recharge and critical hits act on
the weapon part, and the Sharpness part follows the recharge in a straight line
and is never critical. The existing x1.75 scale applies to the sum.
SCENE_ATTACK_SEND logs sharpness= and effects=. Versions 0.10.23. No native or
layout change. Step 5 of the batch plan. Done by Claude Code.

**Why:** Backlog: "Still missing: enchantment damage (Sharpness and the like),
temporary effects such as Strength". An enchanted sword hit Skyrim enemies no
harder than a plain one.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Function
names checked against the 26.3 jar with javap before writing
(ItemStack.getEnchantments, ItemEnchantments.entrySet, Enchantments.SHARPNESS,
MobEffects.STRENGTH and WEAKNESS, LivingEntity.getEffect,
MobEffectInstance.getAmplifier).

**Result:** 31 Rust tests pass. Packages 0.10.23 built.

**Still broken / not tested:** Not played; no automated test covers the Java
arithmetic. Worked example to check in the log: iron sword (6) with Sharpness
III (2) at full charge asks for (6+2) x 1.75 = 14. Smite, Bane of Arthropods
and Impaling are deliberately left out. A hit weakened to zero with no
Sharpness sends nothing. Fire Aspect, Knockback, Looting and Sweeping Edge do
nothing to Skyrim enemies.

**Next:** Step 6 of the batch, 0.10.24: pointer speed independent of the
Minecraft window size.

## 2026-10-11 Protection while Minecraft is paused (0.10.22)

**Changed:** Skyrim side only. lib.rs: new Endpoint::idle_position, which
recognises the connected Minecraft sending "not active" frames no older than
half a second (position.rs MAX_IDLE_AGE_MS). skse.rs: when there is no active
frame, a paused Minecraft calls damage::pause instead of damage::reset.
damage.rs: pause() keeps the waiting hits and turns protection on for half a
second per frame seen, but only for the same Minecraft session, world and
loaded Skyrim save that was steering; a hit is then routed by the new pure
function route(): forwarded to Minecraft, cancelled without queueing
(protected), or left to Skyrim. Protection is logged on, off and per cancelled
hit (SCENE_DAMAGE_SHIELD). Versions 0.10.22. No layout change; protocol 30.
Step 4 of the batch plan. Done by Claude Code.

**Why:** Measured in report 20261011-094530: while Minecraft was paused or not
the active window, 113 hits took 52.6 health from Skyrim's character directly
and 22 hits waiting for delivery were thrown away. The user asked for the
recorded candidates to be worked on; "protect while paused" is the choice made
here and can be reverted alone.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New unit
tests: every case of route(), and that only the Minecraft that was steering
keeps the protection. The connection test now also checks that a playing
Minecraft is not "paused", a paused one is recognised but never steers, and a
silent one stops counting after half a second.

**Result:** 31 Rust tests pass (25 unit, 5 connection, 1 position). Packages
0.10.22 built.

**Still broken / not tested:** Not played. Consequences to know: while
Minecraft sits paused, Skyrim's character cannot be hurt at all; enemies keep
attacking and the fight resumes when Minecraft does. Hits that were waiting
are delivered if Minecraft resumes within five seconds and dropped (logged as
expired) after that. If Minecraft stops drawing frames (closed, crashed,
possibly minimised) protection ends within half a second and Skyrim's own
damage applies, as before. Whether a minimised Minecraft keeps drawing frames
is not known.

**Next:** Step 5 of the batch, 0.10.23: Sharpness, Strength and Weakness.

## 2026-10-11 Continuous Skyrim damage counts (0.10.21)

**Changed:** BridgeDamage.java only. Hits forwarded from Skyrim are added to a
pool (still consumed and acknowledged once each). The pool is delivered as one
Minecraft hit whenever Minecraft's half-second hit pause is over (10 or fewer
pause ticks left; as built in 0.10.21 this read the wrong counter, corrected in
0.10.27 above), then emptied whatever Minecraft does with it. A pool that
could not be delivered for two seconds (Skyrim menu, pause) is dropped and
logged, never delivered late. Versions 0.10.21. No native or layout change.
Step 3 of the batch plan. Done by Claude Code.

**Why:** Measured in report 20261011-094530: of 99 Skyrim hits under 2 damage
Minecraft took 7 (12.76 half-hearts requested, 0.69 applied), because Minecraft
refuses a hit within half a second of the last unless it is larger. Fire,
frost, poison and burning arrive as many tiny hits.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. The pause
rule was read from the 26.3 jar with javap (LivingEntity.hurtServer compares a
pause counter with 10 and the amount with the last hit; I took that counter to
be the "invulnerable time" without reading its name, which was wrong, see
0.10.27 above). The first build failed to compile (a variable name was already
in use); renamed and rebuilt.

**Result:** 29 Rust tests pass. Packages 0.10.21 built.

**Still broken / not tested:** Not played. The Minecraft side has no automated
tests, so this rule is checked only by reading and by the logs once played:
SCENE_DAMAGE_POOL per hit added, SCENE_DAMAGE_APPLY per delivery with hits= and
the summed amounts. Behaviour change to know: two enemies hitting in the same
half second now both count in full (Minecraft's own rule would count only the
larger); a blow can arrive up to half a second late; at most two knockbacks a
second.

**Next:** Step 4 of the batch, 0.10.22: protection while Minecraft is paused.

## 2026-10-11 Keys that open Skyrim's own menus (0.10.20)

**Changed:** New Minecraft keys (Controls > Key Binds > Miscellaneous): Tab
opens Skyrim's character menu, M its map, J its journal, and "Skyrim: wait" has
no key until one is chosen. Inside any Skyrim menu, Tab goes back. New file
SkyrimShortcuts.java; SkyrimMenus.frame sends the command;
InteractionOptionsMixin registers the keys; en_us.json names them. Skyrim side:
menu_input.rs gains command numbers 11 to 14 ("Tween Menu", "Quick Map",
"Journal", "Wait") in the gameplay list and the event table. Versions 0.10.20.
No layout change; protocol stays 30. Step 2 of the batch plan. Done by Claude
Code.

**Why:** Until now Skyrim's character menu, map and journal could only be
opened by clicking into Skyrim's window. With them, fast travel and Skyrim's
own inventory, magic and skills are reachable from Minecraft.

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly`. New Rust
test: the four commands carry exactly Skyrim's event names and are refused when
aimed at an open menu. Existing tests already require every command number to
have one rule and one event, and apply the quicksave rules to every gameplay
command.

**Result:** 29 Rust tests pass (23 unit, 5 connection, 1 position). Packages
0.10.20 built.

**Still broken / not tested:** Not played. Unknown until played: whether
Skyrim's menu-opening handler acts on a forwarded key event the way its
quicksave handler does (same route, different handler). The log can only show
that the event was sent; the user sees the menu or not. Tab is also Minecraft's
"list players" key, which shows nothing in a single-player world.

**Next:** Step 3 of the batch, 0.10.21: continuous Skyrim damage counts.

## 2026-10-11 Skyrim's E takes precedence (0.10.19)

**Changed:** Skyrim side: attack.rs reports a new crosshair kind 6 "living and
hostile to the player", asked of the game itself (Actor::IsHostileToActor,
Address Library ID 37537, resolved in skse.rs; missing address means kind 3 as
before); world.rs TARGET_HOSTILE and the raised maximum. Minecraft side:
SkyrimAttack.activates() (was lootTarget) is true for kinds 1 to 5, and
SkyrimInteraction.beforeKeybinds uses it, so the inventory key activates
anything Skyrim's crosshair is on except an enemy. Protocol 30, name link30
(lib.rs, windows.rs). Versions 0.10.19. Step 1 of the batch plan in
docs/DESIGN.md. Done by Claude Code.

**Why:** User: "I eventually want pressing E in skyrim to take precedent. Only
open the minecraft inventory when E won't do anything in skyrim."

**Tested how:** `.\tools\build-step5b.ps1 -Offline -PackageOnly` (runs the Rust
tests first). The classification test gained the enemy cases; the existing
packet test covers every kind up to the new maximum.

**Result:** 28 Rust tests pass (22 unit, 5 connection, 1 position). Packages
Passthrough-Minecraft-0.10.19.zip and Passthrough-Skyrim-0.10.19.zip built.

**Still broken / not tested:** Not played. Unknown until played: whether the
hostility test answers as expected (animals, summoned creatures, guards after a
crime), and whether E on plants, items, levers and doors feels right. E with
the crosshair on a friendly character now talks instead of opening the
Minecraft inventory; look away to open the inventory.

**Next:** Step 2 of the batch, 0.10.20: keys that open Skyrim's own menus.

## 2026-10-11 Handover rules for Codex and Claude Code; first checkpoint commit

**Changed:** AGENTS.md: new section "Handover between agents" (what must
always be true, how to pick up, where things are, things that have cost time);
rule 3 now carries the user's standing instruction to commit when they say all
testing passed, on branch `dev`, without pushing; the log permission is marked
as applying to both agents; the Environment line names both agents; the "Keep
these files updated" list gains STATUS.md and the Backlog. New
templates/HANDOVER-note.md. STATUS.md: its first section is now a handover note
in that shape. README.md: the one remaining Windows-1252 dash byte (in "bytes
240-255", 0.8.8 section) replaced by a UTF-8 dash, so the file is valid UTF-8
like every other tracked file. No code or build change. Done by Claude Code.

**Why:** The user asked how best to hand over between the two agents, then
asked for documented rules so that either can continue the other's work at any
state, and for a commit each time they report that all testing passed.

**Tested how:** `git add -A --dry-run` before the first commit: 84 files, only
.rs, .java, .json, .md, .ps1, .toml, .lock, .properties and .gitignore; a
credential-pattern scan of those files found nothing; largest file 201 KB.
Checked every tracked file for encoding by decoding its bytes. No build or game
run was needed for these documents.

**Result:** Branch `dev` created from `master`. Commit 28bf33d holds everything
from 0.5.0 through the accepted 0.10.18 (84 files). A second commit holds these
handover documents. `master` and GitHub are unchanged; nothing was pushed. All
tracked files are UTF-8.

Correction made the same day, in a third commit: the first version of this
entry and of AGENTS.md also said that every file uses Windows line endings.
That was wrong. Counting bytes shows 56 tracked files with Unix endings, 32
with Windows endings and 12 with both, a mix that predates this session and is
harmless because Git stores them one way. My check had used `grep` in Git
Bash, which reported Windows endings on every file, including a test file
that had none. Every line-ending count I made that way this session was
meaningless; no file content was changed because of it.

**Still broken / not tested:** The handover rules have not yet been exercised
by a real switch to Codex; the first one will show what they miss. Codex has
not been told about branch `dev` except through AGENTS.md. Notes that Claude
Code keeps in its own memory folder outside the project are not visible to
Codex; what mattered from them is now in AGENTS.md.

**Commit:** 28bf33d (dev) Passthrough 0.5.0 to 0.10.18; the handover documents
are in the commit that follows it.

**Next:** To check: `git log --oneline -3` should show two new commits on
`dev` above a9fc0fd, and `git status --short` should print nothing. At the
next switch, the incoming agent follows "Picking up from the other agent" in
AGENTS.md.

## 2026-10-11 0.10.18 test session verified

**Changed:** README and STATUS record the result; DESIGN backlog gains the
user's wish for Skyrim's E to take precedence everywhere. No code or build
change. Done by Claude Code.

**Why:** User ran the 0.10.18 test list and reports all tests passed.

**Tested how:** User gameplay. Ran `.\tools\collect-scene-logs.ps1` (report
20261011-101205) and counted crosshair kinds, interaction and menu sends
against receipts, recoveries and error lines.

**Result:** Protocol 29 on both sides; no ERROR, DISABLED or fallback line.
Crosshair kinds logged: container 13, living 11, body 4, door 9, other 2.
Activations: 10 from the inventory key, 8 from the Interact key; 17 dispatched,
1 dropped with reason=cooldown. Menu commands: 75 sent, 75 received, 75
returned, including 25 true clicks from the left button and 5 confirms from
the inventory key. Recoveries 3 requested, 3 accepted.

**Still broken / not tested:** The optional map click was not reported. The
logs show that E activated a container or body, not what Skyrim then showed;
that part rests on the user's report. Findings from the 0.10.16 session
(rapid small hits mostly lost; no protection while Minecraft is inactive) are
unchanged.

**Next:** Nothing to install. Commit as the user asked, then write the
handover rules.

## 2026-10-11 E opens Skyrim containers (0.10.18)

**Changed:** Planned in DESIGN first. Rust: world.rs adds target_kind (u32 at
TARGET_AT=397,440, 8 bytes inserted before the damage section; size 398,024;
magic PTWORL13) with kind constants, a maximum and a test; attack.rs classifies
the crosshair reference (classify() pure, kind() reads the game) and logs
SCENE_CROSSHAIR on change; lib.rs protocol 29; windows.rs link29. Java:
NativeLink WORLD_SIZE and TARGET_KIND_AT; SkyrimAttack keeps the kind and
offers lootTarget(); SkyrimInteraction.beforeKeybinds makes the inventory key
confirm in Skyrim menus and activate a container or body under the crosshair,
and logs source=inventory_key; SkyrimSavesMixin calls it. Versions 0.10.18.
ABI, README, STATUS updated. Done by Claude Code.

**Why:** User asked for E to open the Skyrim container. Activating whatever
the crosshair is on would have stopped E opening the Minecraft inventory in a
fight, so Minecraft has to know what the target is.

**Tested how:** Read CommonLib FormTypes.h and TESObjectREFR.h. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first), twice: once
more after a review change. Checked the packaged DLL carries PTWORL13. No game
run, install, log read or commit.

**Result:** 28 tests pass (22 unit, 5 connection, 1 position). Packages
0.10.18: Minecraft 344563 bytes, Skyrim 219302 bytes. Review found one hazard
before release: a press from E could, in a rare timing, have been sent as the
Interact key's "take all"; a press from E can now only activate or confirm.

**Still broken / not tested:** All gameplay is **not tested**. The reads on
arbitrary Skyrim references (base object at +0x40, its type, life state on
NPCs) come from CommonLib and have not run. E does nothing special for doors or
living characters, by design. If Skyrim reports no crosshair target at the
distance the user stands, E opens the Minecraft inventory. Java has no
automated tests.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.18). Follow README "Test 0.10.18": E on a chest, E on open ground,
E on a living NPC, E on a body, left click on a dialogue choice. Report what
happened; logs are collected directly.

## 2026-10-11 Left click is a real Skyrim click (0.10.17)

**Changed:** Planned in DESIGN first. SkyrimMenus.java: in a Skyrim menu the
left mouse button sends command 10 (true click) instead of command 5
(confirm). Versions 0.10.17. Also recorded: the user confirms R inside a
container takes everything (the unconfirmed item from the 0.10.16 session).
Done by Claude Code.

**Why:** User reported the experimental middle-button click "acts like left
mouse in Skyrim" and chose this as the next step. A real click reacts to where
the pointer is, which the confirm key cannot do.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`
(tests first). No game run.

**Result:** Build passed; 26 Rust tests passed at this step. Packages 0.10.17:
Minecraft 343326 bytes, Skyrim 218772 bytes. No wire change.

**Still broken / not tested:** **Not tested** with the left button itself. A
left click with the pointer over nothing no longer confirms the highlighted
entry (Enter and, from 0.10.18, E do). Map markers are untested.

**Next:** Covered by README "Test 0.10.18".

## 2026-10-11 0.10.16 test session verified

**Changed:** README and STATUS record the user's test results and the log
check; DESIGN backlog updated with two findings and the true-click result. No
code or build change. Done by Claude Code.

**Why:** User ran the 0.10.16 test list: A, C, D and E passed; for B, "middle
mouse acts like left mouse in skyrim".

**Tested how:** User gameplay. Ran `.\tools\collect-scene-logs.ps1` (report
20261011-094530; about 167,000 lines) and counted events on both sides with
grep and awk: errors, waits, draws, recoveries, menu sends against receipts,
attack requests against Skyrim health changes, wear, damage applications by
armour and blocking, deaths.

**Result:** No ERROR, DISABLED or fallback_generic line. Fix: 6
mesh_from_another_place waits, then 6410 draws; depth usable in 2221 of 2238
probes. Travel: 9 recoveries requested, 9 accepted, 0 rejected, one fast
travel. Menus: 115 SEND = 115 RECV dispatch = 115 RESULT returned, identical
sequence sets; wheel 25, right_click 14, interact_key (action 9) 5,
pointer_click (action 10) 27. Pointer baseline remembered=true with gain 2,2
from the fourth menu. Attacks: 102 sent, 90 dispatched, 11 dropped
range_or_cell, 1 not seen by Skyrim; max |loss - requested| 0.00001; scale
1.75; 37 critical requests. Wear: 59 events of 1, broken=true at damage 58. Incoming:
110 applications as mob attack; larger hits 11 (9 taken, 2 shield-blocked);
armour 14 turned 13.578 into 8.43. Death: cause=mob, kill, reload, clear,
recovery.

**Still broken / not tested:** Small rapid hits: 99 under 2 Skyrim damage, 7
taken (0.69 of 12.76 requested). This is Minecraft's hurt cooldown combined
with the bridge using up refused hits; it existed before tonight. For about
19 seconds Minecraft sent no active frames (paused or unfocused; not
established which); 113 hits then reached Skyrim's health natively (52.6) and
22 queued hits were dropped. Whether R takes all from a container was not
reported. One attack request never appeared on the Skyrim side; reason not
established. Axes, enchantments, other armour sets, walled cities and places
outside the height range remain untested.

**Next:** Nothing to install. To review: `rg --no-ignore -c "SCENE_DAMAGE_APPLY"
logs/scene-report-20261011-094530` (expect 110) and
`rg --no-ignore -c "event=ERROR" logs/scene-report-20261011-094530` (expect no
output). Choose the next step.

## 2026-10-11 Review correction: button latches cannot stick (0.10.16)

**Changed:** SkyrimMenus.java: the two held-button latches added in 0.10.11
(right click swallowed until released, confirming left click not allowed to
start mining) now count only while the per-frame poll that releases them has
run in the last half second. Versions 0.10.16. Done by Claude Code while the
user was away.

**Why:** Re-reading tonight's Java changes, which have no automated tests. The
latches are released by code that runs once per rendered frame as part of the
bridge sender. If that sender ever stopped after an error while a button was
held, right click would have been swallowed until Minecraft restarted.

**Tested how:** Code review only, then
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No game run.

**Result:** Build passed; 26 Rust tests pass. Final packages 0.10.16:
Minecraft 343307 bytes, Skyrim 218760 bytes. This is the build to install; it
contains every step from 0.10.10 on.

**Still broken / not tested:** **Not tested** in game. The review found no
other defect, which is not proof that there is none.

**Next:** Install 0.10.16 and follow the combined test list at the top of
README.md.

## 2026-10-11 Armour and shields now work against Skyrim hits (0.10.15)

**Changed:** Planned in DESIGN first. BridgeDamage.java: forwarded Skyrim hits
are delivered as Minecraft's "mob attack" damage type instead of "generic",
coming from the position of the nearest Skyrim character in the snapshot (none
listed: no direction). Lookup failure falls back to generic and logs
SCENE_DAMAGE_SOURCE state=fallback_generic once. SCENE_DAMAGE_APPLY also logs
source, attacker, armor and blocking. Versions 0.10.15. Done by Claude Code
while the user was away.

**Why:** Found while checking Minecraft's own tag data in the cached 26.3 jar:
"generic" is listed in bypasses_armor, and bypasses_shield includes that list.
So Minecraft armour and shields did nothing against Skyrim hits. The 0.9.8
entry and DESIGN said Minecraft armour "remains native"; that was never tested
and was wrong.

**Tested how:** Read data/minecraft/tags/damage_type/bypasses_armor.json,
bypasses_shield.json and no_knockback.json from the cached jar (read only,
nothing copied into the project). javap for DamageSource constructors,
DamageTypes, Registries.DAMAGE_TYPE, LivingEntity.getArmorValue/isBlocking.
Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No
game run, install, log read or commit.

**Result:** Build passed. Packages 0.10.15: Minecraft 343229 bytes, Skyrim
218769 bytes. Minecraft side only; no wire change.

**Still broken / not tested:** **Not tested** in game. Side effect: a mob
attack knocks the Minecraft player back a little, which generic damage did not,
so Skyrim hits will now push you. Only Skyrim characters within the collision
snapshot (about 8 blocks, same cell) give a direction; hits from further away
are reduced by armour but cannot be shield-blocked. The nearest character is
assumed to be the attacker, which can be wrong in a crowd. If you die to such a
hit, Minecraft's death line in chat may read oddly because there is no
Minecraft attacker to name. This changes a behaviour accepted in 0.9.9 (damage
arriving in full), on purpose.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Weapons wear out when hitting Skyrim enemies (0.10.14)

**Changed:** Planned in DESIGN first. SkyrimAttack.java: when a bridge attack
is sent, the held item's own "weapon" data decides the wear (swords 1, axes 2)
and it is applied on the integrated server with Minecraft's hurtAndBreak, so
Unbreaking, creative mode and breaking behave as usual. Logs SCENE_ATTACK_WEAR
with item, amount, damage before/after and whether it broke. Versions 0.10.14.
Done by Claude Code while the user was away.

**Why:** Continue gameplay features; "no durability" was in the backlog.

**Tested how:** javap for ItemStack.hurtAndBreak, DataComponents.WEAPON and the
Weapon record. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`
(tests first). No game run, install, log read or commit.

**Result:** Build passed. Packages 0.10.14: Minecraft 341918 bytes, Skyrim
218767 bytes. First draft used ItemStack.is(Item), which does not exist in
26.3; javap showed that before the build and it compares getItem() instead.

**Still broken / not tested:** **Not tested** in game. Wear is charged when the
attack is sent, so a swing that Skyrim then drops (target moved, out of range)
still costs durability. Bare hands and items without weapon data are unaffected.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Critical hits on Skyrim enemies (0.10.13)

**Changed:** Planned in DESIGN first. SkyrimAttack.java: a bridge attack is
critical under Minecraft's own conditions (charge above 0.9, falling, not on
the ground, not climbing, in water, sprinting, riding or blinded); the requested
damage is then multiplied by 1.5 (new constant CRITICAL_SCALE), Minecraft's
critical-hit sound is played, and SCENE_ATTACK_SEND logs critical=true/false.
Versions 0.10.13. Done by Claude Code while the user was away.

**Why:** Continue gameplay features: jump attacks should matter against Skyrim
enemies as they do in Minecraft. Listed in the backlog as "no critical hits".

**Tested how:** javap on the cached 26.3 jar for Entity.fallDistance, onGround,
isInWater, isSprinting, isPassenger, LivingEntity.onClimbable/hasEffect,
MobEffects.BLINDNESS, SoundEvents.PLAYER_ATTACK_CRIT and LocalPlayer.playSound.
Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No
game run, install, log read or commit.

**Result:** Build passed; 26 Rust tests still pass. Packages 0.10.13: Minecraft
340755 bytes, Skyrim 218717 bytes. Minecraft side only; no wire change. A full
wooden-sword critical asks for 7 x 1.5 = 10.5 damage.

**Still broken / not tested:** **Not tested** in game. No critical-hit
particles on the Skyrim enemy (it is not a Minecraft entity). The rule is
checked on the client at click time. Java has no automated tests.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Pointer speed remembered between menus (0.10.12)

**Changed:** Planned in DESIGN first. menu_input.rs: the measured pointer
pixels-per-count is kept for the session (State.learned) and used as the
starting value for the next menu; the two rules are now pure functions
starting_gain() and measured_gain() with two tests using logged numbers. The
baseline log line reports gain_x, gain_y and remembered=true/false. Versions
0.10.12. Done by Claude Code while the user was away.

**Why:** Backlog defect found on 2026-10-11: every menu started from Skyrim's
stored sensitivity (1) while the real value is 2, so the first small pointer
movements in each menu travelled double.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`
(tests first). No game run, install, log read or commit.

**Result:** 26 tests pass (20 unit, 5 connection, 1 position). Packages
0.10.12: Minecraft 340333 bytes, Skyrim 218725 bytes. No wire change.

**Still broken / not tested:** **Not tested** in game. The first menu of each
Skyrim session still starts from the stored sensitivity until one larger
movement has been measured. Pointer speed scaling for a small Minecraft window
stays in the backlog.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 More Skyrim menu controls from Minecraft (0.10.11)

**Changed:** Planned in DESIGN first. Minecraft: SkyrimMenus gains request(),
scroll() (wheel = Up/Down), back() (right click = Back, once per press, press
swallowed until released), an optional "Skyrim menu: click at pointer" key
(default middle mouse button) and a latch so a held confirming click cannot
start mining after the menu closes; SkyrimInteraction sends Skyrim's R-key menu
action when a Skyrim menu is open; SkyrimCursorMixin hooks MouseHandler.onScroll;
SkyrimAttackMixin hooks Minecraft.startUseItem; SkyrimSavesMixin also drops
"pick block" while a menu is open and it shares the pointer-click key; the new
key is registered and named. Rust menu_input.rs: command table event(), pure
verdict() rule, new commands 9 ("XButton", keyboard R) and 10 ("Click", mouse
left), three tests. Versions 0.10.11. Done by Claude Code while the user was
away.

**Why:** Containers, shops and other Skyrim list menus could only be driven
with arrows, Enter and Backspace. Right click also still used the held
Minecraft item during a Skyrim menu.

**Tested how:** javap on the cached 26.3 jar for MouseHandler.onScroll,
Minecraft.startUseItem, KeyMapping constructors and the mouse constants; read
CommonLib UserEvents.h for the event names. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No game run,
install, log read or commit.

**Result:** 24 tests pass (18 unit, 5 connection, 1 position). Packages
0.10.11: Minecraft 339916 bytes, Skyrim 218307 bytes. No wire layout change,
protocol 28. Left click, arrows, Enter and Backspace are unchanged in code.

**Still broken / not tested:** All of it is **not tested** in game. Wheel,
right click and the R action reuse the dispatch path accepted for arrows and
Backspace, but whether Skyrim's item menus act on "XButton" from a forwarded
event is unknown. The true click ("Click" as a mouse button event) is an
experiment and may do nothing. Fast wheel spins can drop steps because only the
newest command is kept. The mixins are checked against method signatures only;
a wrong target would stop Minecraft at start-up.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Invisible blocks after a door fixed (0.10.10)

**Changed:** scene.rs: new mesh_origin(), which refuses a block mesh whose
origin is more than 64 blocks from the Minecraft player on any axis, plus a
test using the real numbers from the failing run. scene_draw.rs: uses it and,
for such a mesh, waits for the next one (SCENE_WAIT
reason=mesh_from_another_place) instead of returning an error. Versions
0.10.10. Done by Claude Code while the user was away.

**Why:** User tested 0.10.9: doors and interiors work and play continues, but
placed blocks were invisible indoors, and blocks placed earlier outdoors were
invisible (though still solid) after coming back out.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1` (report 20261011-012218)
and read it. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`, which
ran the tests first. No game run, install or commit.

**Result:** One cause for both symptoms. At 189587250 ms, the same millisecond
the interior's origin became ready, Skyrim logged `SCENE_DISABLED
message=invalid scene origin`. The newest mesh was still the outdoor one
(origin about 122,26,-580) while the player was already at about
-1245187,129,-4149243; mapped through the interior's origin it exceeded the
position limit, the drawing code treated that as fatal, and block drawing stayed
off for the rest of the session. Minecraft kept sending correct indoor meshes
(origin -1245195,121,-4149252) that were never read. Everything else in that
run worked: door in and out, recovery in 47 ms and 31 ms, 154 indoor collision
scans with 1242 to 2573 boxes each, block placement indoors accepted on both
Minecraft sides. This was a defect in my 0.10.9 change: I did not consider data
left over from the previous place. 21 tests pass (15 unit, 5 connection, 1
position). Packages 0.10.10: Minecraft 338541 bytes, Skyrim 218255 bytes.

**Still broken / not tested:** The fix is **not tested** in game. Whether
Skyrim's depth information is usable for hiding blocks behind walls indoors is
unknown: drawing was already off, so no indoor depth probe was logged. Hands
and HUD drawing use a separate pass and were not affected.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Doors and interiors: a Minecraft region per Skyrim place (0.10.9)

**Changed:** Planned in DESIGN first. saved_origin.rs: Origin::for_location and
the fixed square formula; restore uses it and returns 5 (recover) instead of 2
when the Minecraft position is too far to express. save_identity.rs: origin(),
recovery_target() and menu_authorized() use the place's origin; a derived one
is logged as SCENE_ORIGIN state=derived. skse.rs test: simulated loading door.
Java: BridgeRecovery reports a target outside the Minecraft world in chat and
log; BridgePairing reason-5 text reworded. Versions 0.10.9. DESIGN (plan and
backlog), ABI, README, STATUS updated. Done by Claude Code.

**Why:** User asked for doors and interiors, which also unblocks fast travel
and walled cities. The bridge refused every place except the saved one.

**Tested how:** Read saved_origin.rs, save_identity.rs and the skse.rs frame
flow. Wrote 5 unit tests for the formula and restore rule and extended the
simulated-player test with a door, recovery request, simulated teleport and a
step inside. `.\tools\build-step5b.ps1 -Offline -PackageOnly` ran the tests
then built. No game run, install or commit; no new logs read.

**Result:** 20 tests pass (14 unit, 5 connection, 1 position). Writing the
restore test exposed a real defect before any build: a Minecraft position more
than about 1.4 million blocks from a place's region made restore return 2
("invalid origin"), which never recovers; it now returns 5. Packages:
Passthrough-Minecraft-0.10.9.zip 338385 bytes, Passthrough-Skyrim-0.10.9.zip
218099 bytes. No wire, record or protocol change (28).

**Still broken / not tested:** All gameplay is **not tested**: R on a door
while the bridge steers the player, loading-screen and cell-change timing,
collision sampling inside an interior, Minecraft teleports of millions of
blocks and finding ground there, the way back out, walled cities, fast travel.
The tests use fake objects and prove only the bridge's own arithmetic and
hold/recover logic. Known limits: Minecraft height covers Skyrim z of about
-11,600..15,000 with the current saved origin; two different places whose IDs
share the same low bits would share a region (not detected); light-plugin
places can move if load order changes; a fast-travel marker may not be
selectable from Minecraft. 0.10.5 damage values still unverified in combat.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.9). F5, then R on a building door, walk inside, place a block, R
on the door to leave, go back in. Expect short holds and the block still there.
Report the result; logs will be collected directly. Expect SCENE_ORIGIN
state=derived, SCENE_RECOVERY request/accepted, origin_ready, no ERROR.

## 2026-10-11 Death and respawn verified (0.10.8)

**Changed:** README, STATUS, DESIGN and ABI record the accepted result and the
confirmed life-state reading. No code or build change. Done by Claude Code.

**Why:** User installed 0.10.8 and reports the respawn works and the bridge
reconnects.

**Tested how:** User gameplay. Ran `.\tools\collect-scene-logs.ps1` (report
20261011-005940) and read the death, recovery, alignment, link and error lines
on both sides.

**Result:** Minecraft SCENE_DEATH_SEND sequence=1 cause=genericKill with
immediate_respawn=true keep_inventory=true, then hold reason 7 shown. Skyrim:
RECV age_ms=19 decision=kill; RESULT health 100 -> -200; SCENE_DEATH_LIFE 1 at
+15 ms, 2 at +2.5 s; save reverted/loaded at +6.3 s; SCENE_DEATH clear;
recovery token 2 requested and accepted 31 ms apart; origin_ready; damage
ready. Minecraft reported ready 7115 ms after the death. Zero ERROR, DISABLED
or sender-disabled lines in the three files. Second attempt succeeded.

**Still broken / not tested:** Only `/kill` was exercised. Death from Skyrim
enemy hits, falls or other Minecraft damage, death during a Skyrim menu, and
death with no recent Skyrim save are **not tested**. A Skyrim-only death still
leaves Minecraft health unchanged. No attacks in this report, so the 0.10.5
damage scale is still unverified. F5/F9 not pressed in this run.

**Next:** Nothing to install. To review: `rg --no-ignore "SCENE_DEATH|SCENE_RECOVERY"
logs/scene-report-20261011-005940` (one line); expect kill, life 1 then 2,
clear, and an accepted recovery. Choose the next feature.

## 2026-10-11 Death reconnect repair, tests before build (0.10.8)

**Changed:** world.rs: hold reasons are constants (HOLD_DEAD=7,
MAX_HOLD_REASON) and the packet check uses them; new test encodes a reply for
every reason. death.rs: hold depends on health <= 0 plus grace, life state only
logged (SCENE_DEATH_LIFE). skse.rs: uses HOLD_DEAD; test fixture gains the
missing Addresses fields and is updated to current rules, with a simulated
death, reload and no second kill. save_identity.rs: test-only publish helper.
tools/build-step5b.ps1 runs `cargo test --offline` before building.
tools/collect-scene-logs.ps1 also keeps ERROR, LINK, RELEASED, ANCHOR, LOAD_,
PLUGIN_LOADED, HOOK_INSTALLED, NATIVE_READY and overlay error lines, and the
first 16 lines after a Minecraft "sender disabled" message. AGENTS.md: new
"Test before build" rule. Versions 0.10.8. DESIGN, ABI, README, STATUS updated.
Done by Claude Code.

**Why:** User tested 0.10.7: F5/F9 worked; after `/kill` both characters died
but the bridge never reconnected. User also asked for the Rust unit tests to be
fixed and run before every build, and granted standing log-collection access.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1` twice (reports
20261011-005050 and, after extending the collector, 20261011-005252) and read
them. Ran `.\tools\cargo.ps1 test --offline` until green, then
`.\tools\build-step5b.ps1 -Offline -PackageOnly`, which now runs the tests
first. No game run, install or commit.

**Result:** Cause found in the second report: `event=ERROR message=invalid
world snapshot` at the kill frame. The packet check allowed hold reasons up to 6
and I had added reason 7 in 0.10.6 without changing it, so the Skyrim adapter
rejected its own reply and disabled itself. The first report did not contain
that line because the collector kept only SCENE_ lines; my first reading of it
could not explain the silence. Evidence that the rest worked: death received
age_ms=5, health 100 -> -200, Skyrim save reverted/loaded 6.3 s later; 2
quicksaves and 6 quickloads dispatched once each; 4 recoveries accepted.
Tests: the stale skse.rs test first failed to compile, then failed at its first
gravity assertion because it predates save pairing; after the update all 15
pass (9 unit, 5 connection, 1 position). Packages 0.10.8: Minecraft 338052
bytes, Skyrim 218109 bytes.

**Still broken / not tested:** The reconnect after a death reload is **not
tested**; this is the second build of the feature. Life-state offset is
unconfirmed (read 0 right after the lethal hit). Whether Skyrim keeps calling
the player update while dead is unknown. A Skyrim-only death still does not
reset Minecraft health. 0.10.5 damage values remain untested in combat. The
simulated tests use fake objects, not Skyrim; they check the bridge's own
logic, not game behaviour. The separate background task for the test fixture
worked from the old commit and is superseded by this change.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.8). F5, then `/kill`. Expect the "You died" chat line, Skyrim's
death and reload, then movement again at the saved spot. Report the result;
logs will be collected directly. If it fails again, write the handoff.

## 2026-10-11 Skyrim quicksave/quickload keys (0.10.7)

**Changed:** New SkyrimSaves.java (keys F5/F9, remappable) and SkyrimSavesMixin
(Minecraft.handleKeybinds HEAD drains perspective-toggle clicks in bridge worlds
when the keys coincide). SkyrimMenus.frame sends commands 7/8 with context 0
while no Skyrim menu is open. InteractionOptionsMixin registers the two keys;
en_us.json names them. menu_input.rs accepts 7/8 through a gameplay gate and
dispatches "Quicksave"/"Quickload" key events via MenuControls. Versions 0.10.7.
DESIGN, ABI, README, STATUS updated. Done by Claude Code.

**Why:** User asked, before testing 0.10.6, for F5 to quicksave and F9 to
quickload Skyrim from Minecraft.

**Tested how:** Read CommonLib UserEvents.h, QuickSaveLoadHandler.h,
MenuControls.h and javap output for KeyMapping, Options and InputConstants in
the cached 26.3 jar. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`.
Checked the packaged DLL and JAR contain the new names. No game run, install,
log read, unit tests or commit.

**Result:** Rust and Fabric builds passed. Passthrough-Minecraft-0.10.7.zip
(337938 bytes) and Passthrough-Skyrim-0.10.7.zip (217997 bytes) in dist/step5b.
No wire layout change; protocol 28 as 0.10.6.

**Still broken / not tested:** Everything in game is **not tested**: whether
Skyrim's quicksave handler accepts a forwarded event, key registration and
rebinding, the perspective-toggle suppression, and recovery after F9. The
DESIGN section for this step was written alongside the code rather than before
it. Minecraft's world is neither saved nor rolled back by these keys. Requires
the paired save to be loaded, so F9 cannot be used to get out of an unpaired
save. Rust unit tests still do not compile.

**Next:** Install and test together with 0.10.6 using README "Test 0.10.7".

## 2026-10-11 Minecraft death becomes a Skyrim death and reload (0.10.6)

**Changed:** Planned in DESIGN first, after the user chose the rule. New
passthrough/src/death.rs (decide/hold/kill, vitals read), wired into skse.rs
frame() after lineage admission; damage.rs gains a pass-through flag for the
bridge's own lethal hit; position.rs/lib.rs/windows.rs/native.rs carry a death
number and time (record 456 bytes, protocol 28, link28, pt_mc_frame14). New
BridgeDeath.java and BridgeDeathMixin (ServerPlayer.die observer);
BridgeRulesMixin forces immediate respawn and keep inventory in bridge worlds;
BridgePairing shows hold reason 7; NativeLink sends the two new values.
Versions 0.10.6. Done by Claude Code.

**Why:** Death was undefined: Minecraft could die while Skyrim's character
lived on unprotected. User chose "Skyrim-style: reload the last save".

**Tested how:** Read CommonLib Actor.h/Actor.cpp/ActorState.h for DoDamage,
life state and offsets, and javap for ServerPlayer.die and GameRules. Read the
existing recovery and frame flow. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly`. No game run, install, log
read, unit tests or commit.

**Result:** Rust and Fabric builds passed. 0.10.6 packages were produced
(Minecraft 334356 bytes, Skyrim 216643 bytes) and then superseded in dist by
0.10.7, which contains this change.

**Still broken / not tested:** All of it is **not tested** in game. Specific
unknowns: DoDamage on the player with a null attacker (may not kill, may
crash); life state at Actor+0xC8 bits 21-24; whether death is visible to the
bridge inside the call; Skyrim's reload while its window is in the background;
recovery after that reload. A Skyrim-only death does not reset Minecraft
health. No death screen or message in Skyrim's window. Keep inventory and
immediate respawn are forced on every load of a bridge world, overriding the
player's own setting. Protection while the bridge is merely "not ready" is
unchanged. 0.10.5 damage balance is still untested too.

**Next:** Make a named Skyrim save first. Close games, run
`.\tools\install-build.ps1` (three INSTALLED lines, Fabric 0.10.7), then follow
README "Test 0.10.7": F5, walk, F9, then `/kill`. Expect Skyrim's death and
reload and a return to the saved spot. If Skyrim crashes or the character
survives, stop and report; one more designed attempt only.

## 2026-10-11 0.10.5 left untested; death/respawn investigation started

**Changed:** README and STATUS record that the user chose not to test 0.10.5
and what code reading shows about death today. No code or build change.
Done by Claude Code.

**Why:** User: "I won't test this. It sounds like it should feel right.
Continue with the next feature." Next feature per the earlier recommendation is
death and respawn.

**Tested how:** Read BridgeDamage.java, BridgeRecovery.java, PassthroughClient,
OverlayCapture conditions, damage.rs, recovery.rs and the skse.rs frame flow.
Nothing built, run or installed; no logs read.

**Result:** From code only: Minecraft death makes samples inactive; Skyrim
suspends the puppet and its character then takes native damage (not_ready
passthrough). After a Minecraft respawn the position mismatch should trigger the
existing recovery teleport back to Skyrim's position. No death-specific logic
exists, and the overlay is not shown while dead.

**Still broken / not tested:** 0.10.5 damage balance **not tested** (user
choice). Every statement about death above is from reading code, **not tested**.
Death behaviour is undefined by design until the user chooses a rule.

**Next:** Ask the user what dying should do, then write the plan in DESIGN
before any code. Nothing to install or test for this entry.

## 2026-10-11 Outgoing damage balance (0.10.5) and fuller backlog

**Changed:** Planned in DESIGN first. SkyrimAttack.java: new constant
DAMAGE_SCALE=1.75 replaces the x10 factor; `scale` added to the
SCENE_ATTACK_SEND log line. Versions 0.10.5 in Cargo.toml, gradle.properties,
fabric.mod.json and the SKSE plugin version field. ABI, README and STATUS
updated. Separately, as the user asked, the Backlog section in docs/DESIGN.md
now lists the other deferred and untested items gathered from README, STATUS,
MODLOG and DESIGN, grouped by area. Done by Claude Code.

**Why:** User said damage on Skyrim enemies is far too high (wooden sword 40
killed a 22-health wolf in one hit) and asked for the balance pass. 1.75 puts
Minecraft sword tiers level with Skyrim one-handed swords.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`.
No game run, install, log read, unit tests or commit.

**Result:** Rust and Fabric builds passed (existing linker and deprecation
notes only). Packages Passthrough-Minecraft-0.10.5.zip (330059 bytes) and
Passthrough-Skyrim-0.10.5.zip (216222 bytes) are in dist/step5b. Protocol 27
and all layouts unchanged; the DLL changed only in its version field.

**Still broken / not tested:** New damage values are **not tested** in game.
The Skyrim and Minecraft weapon base values behind 1.75 are from memory, not
read from game files. Fist at 1.75 may feel useless. Axes and other weapons,
enchantments, criticals and durability remain as before. Incoming damage is
unchanged. The backlog list is a gathering of existing notes, not a plan. The
user wrote "the new section in MODLOG"; the Backlog section lives in
docs/DESIGN.md, so the items went there. Rust unit tests still do not compile.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.5). Hit a surviving hostile enemy with a rested wooden sword twice,
then one rested punch. Expect about 7 and 1.75 health lost per hit; logs should
show scale=1.75 and matching native before/after. Report how it feels.

## 2026-10-11 Dialogue pointer accepted; pointer scaling to backlog

**Changed:** Added a Backlog section at the top of docs/DESIGN.md with pointer
speed scaling and the first-movement doubling defect, and marked the 0.10.4
pointer plan as accepted. README and STATUS record the acceptance. No code or
build change. Done by Claude Code.

**Why:** User confirmed the mouse works inside an NPC conversation and asked to
put mouse scaling in the backlog.

**Tested how:** User gameplay report. No new logs read, no tests or builds run.

**Result:** 0.10.4 dialogue pointer accepted by the user (menus and NPC
conversation), with the numeric check from report 20261011-001910 on record.

**Still broken / not tested:** Pointer speed scaling deferred. First small
movements per menu still travel double (no fix decision). Punching an enemy
after a menu not tried. Mouse wheel, right click and true click absent. Rust
unit tests still do not compile.

**Next:** Nothing to install or test. Choose the next gameplay increment.

## 2026-10-11 0.10.4 pointer verified from logs

**Changed:** README and STATUS record the numeric result. No code or build
change. Done by Claude Code.

**Why:** User ran the collector, supplied report scene-report-20261011-001910
and allowed reading it; also reported dialogue keys and after-menu block
placing/breaking work.

**Tested how:** Searched the three report files for SCENE_CURSOR and SCENE_MENU
events with grep; compared requested counts with pointer before/after values
and matched Minecraft command sequences against Skyrim receipts.

**Result:** 3 baselines, 65 logged moves (counters reach 380 and 51). All 92
logged axis movements: (after-before)/request = exactly 2, same sign. Screen
2560x1440, stored sensitivity 1. Minecraft logged 57 pointer sends (window
854x480) and 24 menu sends (8 from mouse, 16 from keys); Skyrim shows 24
dispatches and 24 returned results with identical sequence sets and no
duplicates. Zero ERROR/DISABLED lines. Attempt 1 works for the tested menus.

**Still broken / not tested:** Initial gain 1 is wrong (real 2), so the first
small movements in each menu go double until an 8+ count move; not fixed.
Pointer use inside NPC dialogue is not shown: sessions with pointer movement
had no key presses and the key sessions had no pointer movement, and logs do
not name the open menu. Punching after a menu not tried. Wheel, right click and
true click still absent. Unit tests still do not compile.

**Next:** No reinstall needed. To repeat the check:
`rg --no-ignore "SCENE_CURSOR state=move" logs/scene-report-20261011-001910`
(one line); expect after values differing from before by twice the request.
Ask user whether to fix the first-move doubling and whether speed feels right.

## 2026-10-11 User confirms 0.10.4 menu pointer

**Changed:** README and STATUS record the user's result. No code or build change.
Done by Claude Code.

**Why:** User installed 0.10.4 and reported back.

**Tested how:** User gameplay report only. No logs collected or read (permission
not granted to Claude Code). No tests or builds run for this note.

**Result:** While adding this note I found the one Windows-1252 dash byte in
STATUS.md ("bytes 240-255", 0.8.8 section) had become a replacement character
since my last check. I did not knowingly edit that line and the cause is not
established. Replaced it with a proper UTF-8 dash; README.md's byte is intact.

User: "That seems to work." With Skyrim's Tab menu opened from the
Skyrim window and Minecraft focused again, the pointer followed the mouse and
left click moved through the menu. Backspace left the menu; the bridge
reconnected and movement resumed.

**Still broken / not tested:** Not verified by numbers: pointer read-back,
learned speed factor, vertical direction learning, event counts. Hover and click
on NPC dialogue choices were not described separately from the Tab menu. Mouse
look, block breaking and punching after a menu were not reported. Mouse wheel,
right click and a true Skyrim click event are still not implemented. Rust unit
tests still do not compile (stale skse.rs fixture; separate task started by user).

**Next:** No reinstall or repeat test needed. If log permission is given, check
with `rg --no-ignore SCENE_CURSOR logs` after `.\tools\collect-scene-logs.ps1`;
expect one baseline then move lines whose after values differ from before.

## 2026-10-10 Mouse pointer in Skyrim dialogue (0.10.4)

**Changed:** Planned in DESIGN first. Minecraft: new SkyrimCursorMixin captures
mouse motion at MouseHandler.turnPlayer while a Skyrim menu is open; SkyrimMenus
keeps a running total and sends it with the menu context; SkyrimAttackMixin now
turns a left click in a Skyrim menu into the existing confirm action and skips
vanilla attack/mining for it. NativeLink calls pt_mc_frame13. Rust: position.rs
adds pointer context/totals at 424..440, lib.rs record 440 bytes/protocol 27,
windows.rs link27, native.rs frame13, menu_input.rs builds a MouseMoveEvent for
MenuControls and learns pointer speed from read-back, skse.rs resolves MenuCursor
403551 and MouseMoveEvent vtable 236703. Versions 0.10.4. Updated ABI, README,
STATUS. Done by Claude Code.

**Why:** User asked to make Skyrim's mouse work when talking to an NPC. Skyrim's
window has no focus, so it never receives the mouse; only forwarded keys worked.

**Tested how:** Read local CommonLib MenuCursor/MouseMoveEvent/MenuControls/
MenuEventHandler/IMenu sources and javap signatures for MouseHandler and
Minecraft in the cached 26.3 jar. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (twice, see mistakes below).
Ran `.\tools\cargo.ps1 test --offline --lib`. No game run, install, log read
or commit.

**Result:** Rust release and Fabric builds passed (existing linker note and Java
deprecation note only). Packages Passthrough-Minecraft-0.10.4.zip and
Passthrough-Skyrim-0.10.4.zip are in dist/step5b. The unit-test command FAILED
to compile: the skse.rs test fixture is missing Addresses fields hud_vtable and
interaction. That line is unchanged by this work and already lacked the fields;
no tests ran, including the new record round-trip check.

Mistakes made and corrected this session:
- My first build and test invocations failed only because I added output
  redirection, which PowerShell 5.1 turns into an error. Rerun without it.
- A scripted edit wrote a raw NUL byte instead of the two characters `\0` inside
  the "Cursor" name in menu_input.rs. Same compiled value, wrong source text.
  Corrected and rebuilt.
- Claude Code's edit tool converted Windows-1252 dash bytes (0x96) into the
  replacement character in 14 places in docs/ABI.md (0.8.7 to 0.8.9 byte ranges)
  and 2 places in MODLOG.md (0.8.8 entry) while adding text elsewhere in those
  files. Each sat between two digits of a byte range. Repaired by writing a
  proper UTF-8 dash in those 16 places. README.md
  and STATUS.md each still hold one such byte; their new sections were added by
  prepending bytes so that byte is untouched.

**Still broken / not tested:** All new gameplay is **not tested**: whether Skyrim
accepts the forwarded mouse move, the "Cursor"/0xA event naming (from memory of
Skyrim's control map, not a local source), pointer speed and vertical direction,
hover highlight, click-to-confirm, mixin application at runtime, and regressions
to mouse look, block breaking and punching. No mouse wheel, right click or true
Skyrim click event; right click still acts in Minecraft during a Skyrim menu.
Unit tests remain broken until the stale fixture is repaired (separate task).

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.4). Talk to an NPC with R, move the mouse, point at a choice, left
click. Expect Skyrim's pointer to follow, the choice to highlight and the click
to select it, with Minecraft's view still. Report what happened and whether logs
may be collected; expect SCENE_CURSOR baseline then move lines with changing
after_x/after_y. If the pointer never moves, stop: one more designed attempt
only, then a STATUS handoff.

## 2026-10-10 Point rules and whitelist at templates folder

**Changed:** AGENTS.md now names `templates/MODLOG-template.md` and
`templates/STATUS-handoff.md`. `.gitignore` whitelist drops the two root-level
template entries and allows `templates/` and `templates/*.md` instead. Still a
whitelist. No code, build or game-file change. Done by Claude Code (handover
from Codex while its usage is on cooldown).

**Why:** Both templates live in `templates/`, but the rules and whitelist named
the project root, so Git ignored the real files and the rule paths led nowhere.

**Tested how:** `git check-ignore` on both template paths before and after;
`git status --short` on the touched paths; spot-checked `logs`, `dist` and
`notes.md` are still ignored. No build or game run needed.

**Result:** Before: both templates matched the ignore-all rule (`*`). After:
`git check-ignore` prints nothing and exits 1 (not ignored) and `templates/`
shows as untracked (`??`). `logs`, `dist` and `notes.md` remain ignored.

**Still broken / not tested:** Nothing staged or committed. Git still records
the old root `MODLOG-template.md` as deleted (` D`); that clears when the
deletion and the new `templates/` files are committed together. AGENTS.md and
STATUS.md still say "Agent: Codex"; left unchanged, not asked to edit.

**Next:** To check, run `git status --short templates` (expect `?? templates/`)
and `git check-ignore templates/MODLOG-template.md` (expect no output).
Commit only when requested.

## 2026-10-10 Recharge combat trace accepted

**Changed:** README/STATUS record accepted0.10.3 sword/fist recharge. No code change.

**Why:** User identified final15seconds as relevant combat after enemy setup.

**Tested how:** Collected report20261010-225957; searched ignored log directory
explicitly, matched attack sequences and compared requests with before/after health.

**Result:**19 sends/receipts/results across9781ms. Sword intervals687/454/250ms
produce40/26.483198/14.195199 damage. Fist156/141ms gives5.92; rested fist10.
Native losses match within0.00005 from printed rounding; no duplicate results.

**Still broken / not tested:** Axe, block-priority regression and HUD recharge
not verified here. Full damage balance remains deferred per user feedback.

**Next:** No install/repeat fight needed. Review via `rg --no-ignore SCENE_ATTACK
logs/scene-report-20261010-225957` (one line); expect19 complete attack traces.
Continue next scoped feature when requested.

## 2026-10-10 Weapon attack recharge (0.10.3)

**Changed:** Recorded user feedback that damage is incredibly high; future10x
balance pass in DESIGN/README. Planned then implemented held-component ATTACK_SPEED
and elapsed-client-tick recharge in SkyrimAttack. Added diagnostic charge fields.
Lowered both spam guards500ms to100ms, protocol26 namespace and versions. Updated
README/STATUS/ABI. Full-strength damage unchanged.

**Why:** Continue gameplay features with weaker rapid attacks and item speed.

**Tested how:** Read local javap tickCount/ATTACK_SPEED signatures, reviewed code,
built using build-step5b.ps1 -Offline -PackageOnly. Rust/Fabric succeeded; existing
linker informational/Java deprecation warnings. No tests/game run or installation;
Gradle test NO-SOURCE. No commit.

**Result:**0.10.3 packages in dist/step5b. Gameplay **not tested**. Logs carry
speed, elapsed ticks, charge, multiplier, full damage and requested damage.

**Still broken / not tested:** HUD indicator not synchronized. Only emitted bridge
requests consume recharge; misses/real MC targets use vanilla behaviour separately.
No durability, enchantments, effects or criticals. Damage balance deferred as asked.

**Next:** Close games; `.\tools\install-build.ps1` (three INSTALLED/Fabric0.10.3).
Try sword attacks0.2-0.3s apart then wait two seconds/hit; expect lower quick-hit
request and full rested hit. Use a target surviving first hit; optional axe test,
then normal block break. Collect and search logs with rg --no-ignore.

## 2026-10-10 Sword and punch damage confirmed

**Changed:** README and STATUS record successful0.10.2 comparison. No source change.

**Why:** User reports one-hit wolf kill with sword and mudcrab kill with fist.

**Tested how:** Collected report20261010-224713; read SCENE_ATTACK events using
rg --no-ignore. Compared item computation, request and native before/after.

**Result:** Sword seq1 attribute4/request40, health22 ->-18; fist seq3 attribute1/
request10, health5 ->-5. Second calculation attempt succeeds. Additional fist
seq2 sent against earlier handle has no dispatch/result; reason not established.
User confirms visible kills; species and kill attribution not independently logged.

**Still broken / not tested:** Other weapon types, effects, durability, recharge
and block-priority regression **not tested** by this report.

**Next:** No rebuild/reinstall required. Review with `rg --no-ignore SCENE_ATTACK
logs/scene-report-20261010-224713` (one line); expect40 sword and10 fist with
matching native health changes. Continue next increment when requested.

## 2026-10-10 Held-item calculation repair (0.10.2, attempt 2)

**Changed:** Planned in DESIGN; SkyrimAttack uses player base attack and held
stack ATTRIBUTE_MODIFIERS.compute for MAINHAND instead of client total attribute.
Added base/observed/component-count diagnostics; versions and README/STATUS updated.

**Why:**0.10.1 wooden sword and fist both returned1 and dealt10 in report223504.

**Tested how:** Read local javap signatures for component compute and player base
attribute API. Built with build-step5b.ps1 -Offline -PackageOnly; Rust/Fabric pass.
No game run or test suite; Gradle test NO-SOURCE. No game writes or commits.

**Result:**0.10.2 packages built. Gameplay **not tested**. Protocol25/frame12
unchanged. Existing linker informational/Java deprecation warnings remain.

**Still broken / not tested:** Runtime weapon differentiation awaits confirmation.
Temporary player modifiers, enchantments, durability, recharge and criticals not
implemented. If attempt2 fails in game, stop and produce template-based handoff.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric0.10.2), compare fist then unenchanted wooden sword with one second between
hits. Expect sword damage greater than fist. Collect logs using rg --no-ignore.

## 2026-10-10 Sword versus fist comparison failed

**Changed:** README and STATUS record the failed0.10.1 runtime result; no code changes.

**Why:** User asked whether wooden sword and subsequent punch did different damage.

**Tested how:** Collected scene-report-20261010-223504 while games running;
searched SCENE_ATTACK_SEND/RECV/RESULT with rg --no-ignore.

**Result:** Wooden sword sequence1 attribute1.0 requested10, health22 ->12;
empty hand sequence2 attribute1.0 requested10, health12 ->2. Both dealt10.
Transport matches. Client attribute value did not include a sword increase.

**Still broken / not tested:** Weapon damage differentiation failed its first
runtime test; underlying attribute behaviour not yet investigated. No fix tested.

**Next:** Investigate held-item calculation when continuing. No reinstall needed.
Review with `rg --no-ignore SCENE_ATTACK logs/scene-report-20261010-223504`;
expect two sends and matching native dispatch/results showing equal damage.

## 2026-10-10 Held-item base damage (0.10.1)

**Changed:** Planned in DESIGN before code. SkyrimAttack captures Minecraft
ATTACK_DAMAGE*10 and logs item/attribute/request. NativeLink/frame12 transports
float at connection412; attack.rs validates it and passes it to DoDamage.
Updated protocol25, namespace, Rust/Fabric/SKSE versions, README/STATUS/ABI.

**Why:** Next scoped combat increment after verified fixed-strength punches:
make held weapons affect damage while retaining the empty-hand baseline.

**Tested how:** Read local javap API signatures for getAttributeValue and compiled
Rust/Fabric using build-step5b.ps1 -Offline -PackageOnly. First inspection command
had PowerShell brace syntax error; corrected. First build passed but review found
hardcoded Fabric manifest still0.10.0; corrected and rebuilt successfully.
No test suite run; Gradle test NO-SOURCE. No game installs or commits.

**Result:** 0.10.1 packages staged in dist/step5b. Native and Java builds pass.
Existing linker informational/deprecation warnings remain. Gameplay **not tested**.

**Still broken / not tested:** No weapon durability, enchantment-specific damage,
criticals or weapon-speed cooldown;500ms and210-unit range retained. Initial10x
scale may need balancing. Runtime held-item attribute and health results await
user test. Existing real Minecraft block/entity priority unchanged in code.

**Next:** Close games, run `.\tools\install-build.ps1`; expect three INSTALLED
lines and Fabric0.10.1. Compare empty hand vs unenchanted wooden sword on hostile
enemies with one second between clicks; sword should hit harder. Break a normal
Minecraft block. Collect logs and compare requests/results with rg --no-ignore.

## 2026-10-10 Controlled punch trace and correction

**Changed:** README and STATUS now record complete outgoing attack evidence and
correct the previous false missing-log diagnosis. No code or architecture change.

**Why:** Verify loaded build and trace punches following the user's controlled test.

**Tested how:** Ran collect-scene-logs.ps1 (report20261010-222744), read startup
records and searched using rg --no-ignore. Rechecked previous report221938.

**Result:** Three sends/dispatches/results, sequences1..3, target000EBC4D,
health22 ->12 ->2 ->-8 (10 per punch). Five incoming captures match five accepted
Minecraft applications. Fabric0.10.0/native protocol24. Earlier report also has
three complete punch traces. Previous rg directory searches silently skipped
ignored logs, causing my incorrect conclusion; logging itself was working.

**Still broken / not tested:** Kill processing/attribution, range rejection,
empty aim and block priority are **not tested** by these results. No new tests run.

**Next:** No reinstall/repeat fight needed. To reproduce the log review run
`rg --no-ignore SCENE_ATTACK logs/scene-report-20261010-222744`; expect three
SEND, three dispatch RECV and three RESULT lines. Continue combat when requested.

## 2026-10-10 User confirms outgoing punch

**Changed:** README and STATUS record successful user gameplay. No code/build change.

**Why:** User confirms punching damages a Skyrim enemy.

**Tested how:** Collected scene-report-20261010-221938; searched collected logs
and raw logs/step2 for attack events. Collector includes all event=SCENE_ lines.

**Result:** Gameplay confirmed by user. No SCENE_ATTACK events in report or
SEND/RESULT in raw bridge logs; numeric damage not independently verified.
README update succeeded; atomic STATUS replacement failed with access denied,
then completed by writing the already prepared bytes directly.

**Still broken / not tested:** Exact damage, range/empty aim rejection, block
priority and regressions are **not tested** by this report. Missing attack logs
unexplained. Existing weapon/animation limitations remain.

**Next:** No rebuild/reinstall needed. For another report run
`.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines and report path.
Choose next gameplay increment when requested.

## 2026-10-10 Basic outgoing punch (0.10.0)

**Changed:** Planned in DESIGN, added SkyrimAttack/startAttack mixin without
cancelling vanilla input. Real MC blocks/entities take priority. Snapshot publishes
crosshair handle; frame request includes sequence/handle/first-send timestamp.
Native attack.rs validates actor handle/type/range/cell/health and calls DoDamage
37335 with player source, fixed10 damage and Skyrim difficulty enabled. Reused
existing reference-counted ActorRef lifetime wrapper. Added post-attack load/menu
checks, protocol24, versions and documentation.

**Why:** User approved attacking Skyrim enemies from Minecraft while preserving
block breaking. User clarified the previous not-ready hit happened after closing
Minecraft while chased; it is expected native behaviour, not an active defect.

**Tested how:** Read local CommonLib DoDamage signature/Address Library ID, form
type and reference layout; local javap confirms Minecraft.startAttack returns
boolean. Built twice using build-step5b.ps1 -Offline -PackageOnly, including final
post-attack guards. Rust/Fabric passed. No test suite/game run; Gradle test NO-SOURCE.

**Result:** 0.10.0 packages ready. Protocol24, connection424, world size unchanged,
magic PTWORL12, pt_mc_frame11. No installs or commits. New gameplay **not tested**.
Existing linker informational and Java deprecation warnings remain.

**Still broken / not tested:** No weapon/enchantment/durability stats, Skyrim swing,
critical hits or knockback. Same-cell feet range is conservative; stale snapshot
handles may discard quick aiming changes. Native targeting/DoDamage health effect,
block-priority behaviour and combat reactions require game verification.

**Next:** Install, click a close hostile NPC twice, try out-of-range/empty aim,
break a real MC block, and collect outgoing attack/target-health logs. Confirm
incoming damage and dialogue regressions separately in the same session.

## 2026-10-10 Active incoming damage accepted (0.9.9)

**Changed:** Documentation only: README, DESIGN and STATUS record accepted live
nonlethal forwarding and the observed fallback limitation. No source/build change.

**Why:** User reports damage worked with the second interception attempt.

**Tested how:** Collected report20261010-220115 and compared native clamp/capture/
ack with Minecraft server apply events and health samples.

**Result:** Generation3 sequences1..3 captured once, applied once (accepted=true)
and acknowledged once. Raw damage6.789/4.526/4.526, max100; Minecraft requested
1.3578/0.9052/0.9052, health20 ->18.6422 ->17.737 ->16.831799. Native health100
at all three clamps and all exported health samples. Later Minecraft samples
rise to17.831799. No duplicate applications in these records.

**Still broken / not tested:** A fourth hit at177713671 passed through not_ready,
reducing native health to93.211 after coverage loss/save activity. Do not claim
unconditional native health protection. Exact cause of readiness loss not proven.
Other damage sources, lethal/death/respawn and full healing remain unverified.

**Next:** Stabilize policy for not-ready/disconnect protection and then death/
respawn, before broader combat. No new test required for this documentation update.

## 2026-10-10 Failed mutation hook and second clamp-hook attempt (0.9.9)

**Changed:** Replaced ModActorValue slot6 interception with primary player
CheckClampDamageModifier slot0x127. Prior clamp invoked once; health input/output
and current health logged. Negative returned health delta is queued under the
existing lease and replaced by zero when captured. Queue/ack/scaling/MC damage
unchanged. Updated DESIGN, version metadata, README/ABI/STATUS.

**Why:** Report 20261010-215421 shows readiness true/max100, two -6.789 probes,
but no mutation capture/passthrough records. Enemy damage bypassed our old virtual
entry. This is the second attempt at incoming damage forwarding, not a proven fix.

**Tested how:** Collected logs and inspected CommonLib Actor.h/Actor.cpp:
CheckClampDamageModifier returns float, takes ActorValue and delta, non-VR slot
0x127. Local capstone unavailable; no disassembly performed. Built package using
build-step5b.ps1 -Offline -PackageOnly. Runtime path/effect **not tested**.
No test suite or game run. No game-install writes or commits.

**Result:** First forwarding attempt failed despite readiness. Second interception
uses a documented interface with diagnostic logs to establish actual invocation.
No claim that the game uses it for these hits until logs confirm.

**Still broken / not tested:** Shared damage remains unverified, as does returning
zero from this callback. Death/respawn/scripted setters remain unsupported.

**Next:** Install 0.9.9 and take one nonlethal hit. Collect logs. If it fails,
stop hook variations and prepare STATUS-handoff per AGENTS.md two-attempt rule.

## 2026-10-10 Forward incoming damage to Minecraft (0.9.8)

**Changed:** Added damage.rs mutation hook and bounded shared queue; Minecraft
BridgeDamage validates context and applies each event once on integrated server.
Hook targets the actual player's ActorValueOwner+0xB8 slot 6, negative health
modifier 2/value 24. Suppresses only after enqueue with fresh server readiness.
Maximum-health fraction uses permanent health plus temporary modifier (38469).
Minecraft hurtServer(generic) keeps native armour/cooldown/absorption/death rules.
Added acknowledgement/readiness fields, versioned world damage payload, logs,
protocol 23 and package/docs updates. Existing mutation handler retained outside
readiness and for other actor values/modifiers.

**Why:** User explicitly requested building damage forwarding and preventing
separate native health loss after selecting Minecraft health authority. The
observed HandleHealthDamage callback is too late for suppression.

**Tested how:** Read CommonLib ActorValueOwner damage/restore implementation,
modifier enum, maximum-health computation; inspected Minecraft hurtServer API
using local javap. Built twice with build-step5b.ps1 -Offline -PackageOnly;
Rust/Fabric passed, Gradle test NO-SOURCE. No test suite or game run.

**Result:** Final packages Minecraft 326169 bytes / Skyrim 220335 bytes. Source
review caught damage count shadowing collision count; corrected before final
package and confirmed separate loops. Added serialized hook installation and
readiness diagnostics. Previous read attempt was blocked by approval-service usage
limit, resumed successfully. No game-install writes or commits.

**Still broken / not tested:** All new runtime behaviour is **not tested**.
Actual enemy mutation interception and maximum-health API need game confirmation.
Full death/respawn presentation, direct health setters and attack forwarding remain
out of scope. No durable event delivery through crashes; pending hits can expire
or be cancelled. Native damage resumes outside readiness/queue capacity. Existing
linker informational and Java deprecation warnings remain.

**Next:** Install, verify stable load/movement, take two nonlethal hits separated
by one second, retreat and heal using Minecraft food. Compare capture/apply/ack
and both health values before claiming the authority path works.

## 2026-10-10 Damage callback ordering established for two enemy hits

**Changed:** Documented 0.9.7 probe results in README, DESIGN and STATUS. No code
or package changes.

**Why:** User completed two enemy hits followed by regeneration.

**Tested how:** Collected report 20261010-120922 with collect-scene-logs.ps1;
read probe events and compared health snapshots by sender/world/epoch/snapshot.

**Result:** Hook installed successfully. Sequence 1 at 142166187: supplied=-6.789,
before=after=93.211. Sequence 2 at 142168406: supplied=-4.526,
before=after=89.694016. Both attacker 000EBC4D, on different callback threads.
measured_loss=0 inside both original calls: health mutation precedes this callback.
Sampled health drops from 100 to 93.256, then 94.16464 to 89.73967, returning to
100 at 142190703. All 587 received snapshots match out of 588 exports; client
health remains 20. No native ERROR/LOAD_FAILED events found.

**Still broken / not tested:** Callback-only suppression cannot implement Minecraft
health authority. Other damage sources, lethal hits and an earlier mutation path
are not tested. No damage forwarding or health writes implemented.

**Next:** Investigate the actual health-mutation entry point before suppression;
use the signed callback data only with verified semantics and duplicate guards.
No new test or build required for this documentation update.

## 2026-10-10 Minecraft health policy and individual damage probe (0.9.7)

**Changed:** User selected Minecraft-owned health and food/healing. DESIGN now
specifies staged damage-event verification, acknowledged transport, and eventual
Skyrim health suppression/death integration. Implemented only native damage probe
in skse.rs: PlayerCharacter vtable slot 0x104, original invoked once, before/after
health through corrected +0xB8 accessor, unique sequence/attacker/load revision/
thread diagnostics. Expanded vtable address bound and updated versions/docs.

**Why:** Sampled health deltas can combine damage/healing or loads. Verify actual
callback semantics before writing hearts or suppressing Skyrim damage.

**Tested how:** Read local CommonLib Actor::HandleHealthDamage declaration and
RelocateVirtual implementation (non-VR slot 0x104). Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly`: Rust/Fabric passed. No test
suite or game run; Gradle test NO-SOURCE.

**Result:** 0.9.7 packages built, protocol remains 22. Original damage behaviour
preserved intentionally. No installs, commits or health writes. New probe runtime
behaviour **not tested**; ownership policy is not yet implemented end-to-end.

**Still broken / not tested:** Callback ordering/amount semantics, shared damage,
maximum-health scaling, death/respawn, disconnect authority and attack forwarding.

**Next:** Install, verify loading, receive two enemy hits and inspect probe logs.
Only after evidence should the damage transport and health authority be enabled.

## 2026-10-10 Plan health ownership before damage writes

**Changed:** Added common damage, duplication, load-baseline and scaling rules to
DESIGN. Asked user to choose Minecraft or Skyrim health/healing authority.
Documentation only; no source, package, or game-file changes.

**Why:** Movement authority does not determine how food, healing and death should
work. Applying sampled health differences directly risks missed or duplicate hits.

**Tested how:** Read current design and lifecycle code references. No tests run.

**Result:** Common rules documented; health ownership decision pending. Existing
0.9.6 read-only build remains current. No new gameplay to test for this update.

**Still broken / not tested:** Damage writes, attack forwarding and death/respawn
synchronization remain unimplemented. No new runtime behaviour claimed.

**Next:** Resolve the requested gameplay preference, then plan and implement one
scoped nonlethal damage increment with diagnostics.

## 2026-10-10 Health observation accepted under damage and regeneration

**Changed:** Documentation only: README and STATUS record the successful health
test. No source, architecture or package changes.

**Why:** User reports travelling to an enemy, receiving two hits and waiting
for regeneration after each.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1`; parsed report
20261010-120304 by sender/world/epoch/snapshot and compared native health values.

**Result:** 1043 send and receive snapshots, all matching, zero mismatches or
missing send keys. Observed 100 -> 93.22922 -> 100 (14.953 seconds to full from
first sampled low), then 100 -> 93.28407 -> 100 (14.407 seconds). 1043 client
records keep Minecraft health at 20. No native ERROR/origin-block events found.
Sampling does not establish exact hit damage or exact regeneration start time.

**Still broken / not tested:** Cross-game damage/death and attacks are not
implemented. This confirms read-only health transport, not health synchronization.
No test suite run; no extra gameplay test required for the documentation update.

**Next:** Design damage ownership and death/load baselines before health writes.

## 2026-10-10 Verify 0.9.6 load stability and baseline health

**Changed:** Documentation only: README and STATUS record the user-run stability
check and numeric health comparison. No code or package changes.

**Why:** Verify the crash correction before requesting damage/regeneration tests.

**Tested how:** Collected `.\tools\collect-scene-logs.ps1` report
20261010-115741. Parsed native health records by sender/world/epoch/snapshot,
compared values, and inspected collected errors and final save events.

**Result:** 329 exports across 69719 ms; 327 received snapshots all match exactly,
zero mismatches or unmatched receive keys. 327 client records. Skyrim health 100,
Minecraft health 20. Later Skyrim save events show continued operation. No logged
ERROR/LOAD_FAILED/menu-disabled or Minecraft sender-disabled events found.
The load-time crash was not reproduced during this run; not a general stability
guarantee. Two intermediate exports were not received in latest-snapshot transport.

**Still broken / not tested:** Damage/regeneration and cross-game health writes
remain untested/unimplemented respectively. No automated test suite run.

**Next:** Use installed 0.9.6 for ordinary Skyrim damage/healing outdoors, then
collect logs again. No install or rebuild needed for this documentation update.

## 2026-10-10 Correct health-read offset after load crash (0.9.6)

**Changed:** Corrected world_export ActorValueOwner subobject offset from 0xB0
to 0xB8 for Skyrim 1.7.104. Updated version metadata, corrected DESIGN/ABI offset
claims and recorded failed 0.9.5 gameplay in README/STATUS. Protocol unchanged.

**Why:** User reports save-load crashes and could not test damage/regeneration.
I incorrectly treated the older Actor layout as valid for all SE/AE runtimes.
Local Actor.h AsActorValueOwner uses RUNTIME_CAST_ACCESSOR_VERSIONED with
threshold 1.6.629, old offset 0xB0, new offset 0xB8. The macro selects the new
member offset for this runtime. This is a concrete ABI error in the new call.

**Tested how:** Collected scene-report-20261010-115402, which ends after save
admission/alignment before health export. Read versioned accessor/macro and
built `.\tools\build-step5b.ps1 -Offline -PackageOnly`; Rust/Fabric passed.
No crash dump was inspected; logs alone do not establish the faulting instruction.
No game test or test suite run; Gradle test NO-SOURCE.

**Result:** 0.9.6 packages built. Runtime crash repair **not tested**. No install
writes or commits. Damage/healing remains untested, not a failed gameplay test.

**Still broken / not tested:** Must confirm save loading and first health read
survive. Then assess damage/healing. Existing informational linker warning remains.

**Next:** Install patch, load same save with paired Minecraft active, wait 30
seconds and walk briefly. Collect logs before asking for further damage testing.

## 2026-10-10 Read-only Skyrim health feed (0.9.5)

**Changed:** Planned combat in stages in DESIGN; implemented health observation
only. world_export reads ActorValueOwner::GetActorValue(kHealth) on the existing
player update thread. world snapshot carries f32 current health at reserved
header offset 68. Native SEND/RECV and Minecraft client logs include snapshot
identity and health; client log also records Minecraft health. Updated package
versions, protocol 22, PTWORL10 magic, README/ABI/STATUS. Packet size unchanged.

**Why:** Establish damage readings before choosing health/death authority or
applying damage across games. Dialogue/menu repairs were accepted in 0.9.4.

**Tested how:** Read local CommonLib Actor/ActorValueOwner/ActorValues definitions
(ActorValueOwner +0xB0 for SE/AE, vtable slot 1, health enum 24). Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly`; Rust/Fabric passed. No test
suite or gameplay run; Gradle test NO-SOURCE. New game behaviour **not tested**.

**Result:** 0.9.5 packages ready in dist/step5b. No game installs, commits, health
writes or attack dispatch. Nonfinite readings fail snapshot validation; health
logs require active admitted snapshots to avoid false disconnect zeroes.

**Still broken / not tested:** No cross-game damage, death synchronization or
attacks. Health readings under damage/healing need user verification. Sampling
follows collision exports and may miss short-lived changes; paused menus suspend it.

**Next:** User installs and experiences Skyrim damage/healing, then collect and
compare health logs. Decide health/death rules before implementing health writes.

## 2026-10-10 Dialogue and interior return acceptance

**Changed:** Updated README and STATUS with user-confirmed 0.9.4 behaviour.
No code, architecture or package changes.

**Why:** User confirms single-step dialogue selection, Enter and Backspace; also
reports intentional interior blocking and successful outdoor reconnection.

**Tested how:** User played both games. Collected logs directly with
`.\tools\collect-scene-logs.ps1`; read report 20261010-114651 and counted native
menu dispatch sequences. No automated test suite run.

**Result:** 14 dispatches, zero duplicate sequences, contexts stable across
callback threads. Interior cell 000133C8/worldspace 0 blocked reason 4 at
140786765; origin_ready resumes at 140792812 with unchanged saved origin.
User confirms the visible dialogue actions and outdoor return behaviour.

**Still broken / not tested:** Interior bridge play remains unsupported. Other
menu types, rebinding and held-key suppression are not tested in this report.

**Next:** The three scoped recovery repairs now have user acceptance. Choose the
next gameplay increment; no implementation started in this documentation update.

## 2026-10-10 Shared menu state repair (0.9.4)

**Changed:** menu_input.rs now keeps context/fingerprint/last-command in one
process-wide mutex. UI callbacks use a nonblocking lock, including dispatch, so
reentrant/contending callbacks skip safely. Only mapping handles remain thread-local.
No unsafe Send implementations added. State/receive/result logs include callback
thread ID. Updated versions, DESIGN plan, README, ABI and STATUS.

**Why:** 0.9.1 report showed oscillating context numbers and duplicate dispatch.
Different callback threads had independent state. This is the next scoped menu
repair; event routing is unchanged until runtime evidence warrants another change.

**Tested how:** Collected report 20261010-114219; recovery token 1 server/client
positions match and Skyrim accepted with error_squared=0 (47 ms after request).
Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`: Rust/Fabric passed.
No new tests or test suite run; Gradle test NO-SOURCE. Menu gameplay **not tested**.

**Result:** 0.9.4 packages built. Protocol remains 21, PTWORLD9; no save format
changes. No game install writes or commits.

**Still broken / not tested:** Actual dialogue highlight/confirm/back remains
unverified. Ordinary menu events might need a dialogue-specific path; do not
claim fixed merely from a returned dispatch. Save quickload/marker stability
beyond the observed recovery handshake is not independently established by logs.

**Next:** User tests R, single Down/Up, Enter and Backspace, then movement resume.
Collect logs; expect stable context and no repeated dispatch for the same sequence.

## 2026-10-10 Save-position recovery handshake (0.9.3)

**Changed:** Added native recovery.rs and Minecraft BridgeRecovery, using the
saved origin to convert the loaded Skyrim position into a Minecraft teleport.
Integrated-server connection performs the teleport; server/client agreement
produces a token acknowledgement. Movement/rendering remain held until native
acceptance and fresh coverage. Added timeout/session/load/location guards and
teleport deduplication across transient missing snapshots. Added logs, versions,
README/ABI/STATUS and implementation plan in DESIGN. PBOR mapping is unchanged.

**Why:** Step 2 of the recovery plan. Step 1 accepted by the user; collected report
20261010-113517 confirms outdoor cells 9731 and 9712 accepted with worldspace 3C
and mapping preserved. Previously, matching save lineage could fail reason 5
because current Minecraft and loaded Skyrim positions disagreed.

**Tested how:** Read collected logs, local javap teleport API signatures and source.
Built twice with `.\tools\build-step5b.ps1 -Offline -PackageOnly`, including a
final rebuild after transient-gap deduplication. No test suite or game run;
Gradle test NO-SOURCE. Both builds passed with existing informational warnings.

**Result:** 0.9.3 packages: Minecraft 311896 bytes, Skyrim 212826 bytes. Protocol 21,
PTWORLD9 397488 bytes, pt_mc_frame9; connection remains 384 bytes. No installs,
commits or game-file writes. New game behaviour **not tested**.

**Still broken / not tested:** Dialogue arrows/context duplication remain pending.
Recovery, quickload, marker stability and stale-world guards need game testing.
No historical inventory rollback, different-worldspace/interior travel, or height
outside Minecraft's build limits. After timeout the bridge holds for diagnosis.

**Next:** User installs 0.9.3 and tests the previously failing paired load plus
quicksave/walk/quickload. Collect token/position logs, then address menu state.

## 2026-10-10 Outdoor cell continuity workaround (0.9.2)

**Changed:** Planned three independent recovery steps in docs/DESIGN.md. Implemented
step 1 only: Origin.same_location permits distinct nonzero cells within the same
nonzero worldspace; interiors require exact cell. Used by saved-origin restore and
menu eligibility. Added outdoor_cell_accepted diagnostic, bumped package versions,
and updated README/ABI/STATUS. No saved-origin format, coordinates or protocol change.

**Why:** Collected scene-report-20261010-112905 directly. Walking from cell 00009732
to 00009731, worldspace 0000003C, failed reason 4 at time_ms=139650046. The apparent
range limit was an exact-cell check. Initial load separately failed reason 5 after
successful lineage admission. Menu tokens alternate and sequence 10 dispatches
twice, consistent with separate thread-local state across UI callback threads.

**Tested how:** Read collected logs and source. Ran
`.\tools\build-step5b.ps1 -Offline -PackageOnly`; Rust and Fabric build passed.
No new tests or test suite run; Gradle test NO-SOURCE. Game behaviour **not tested**.

**Result:** 0.9.2 packages ready in dist/step5b. Existing coordinate mapping,
Minecraft world UUID/lineage checks and 140-unit position guard are preserved.
Cell transitions still clear/rebuild collision coverage through the existing path.

**Still broken / not tested:** Save-position recovery and dialogue arrows remain
known failures. Native menu dispatch has duplicate/context problems; use Skyrim
controls for choices until step 3. Outdoor crossing and marker stability need user
verification. Doors, different worldspaces and fast travel remain unsupported.

**Next:** Install and cross the previously failing outdoor boundary in both
directions; collect logs. Then implement explicit save-position resynchronization
without shifting existing blocks, followed by one shared menu command state.

## 2026-10-10 Menu navigation from Minecraft (0.9.1)

**Changed:** Added SkyrimMenus key bindings and native menu_input UI tasks; extended
NativeLink, position transport, connection status and save-load revision tracking.
Protocol 20 uses a 384-byte connection. Minecraft holds movement and suppresses
scene export during fresh Skyrim menu status. Added menu send/receive/state logs.
Updated DESIGN before implementation, plus README, ABI, STATUS and package versions.

**Why:** NPC activation works; the next gameplay increment is selecting dialogue
choices and leaving Skyrim menus without switching controls to Skyrim.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`.
Read local CommonLib ABI definitions. No game testing or test suite was run;
Gradle reported test NO-SOURCE.

**Result:** Rust and Fabric builds passed. Packages: Minecraft 304808 bytes,
Skyrim 209951 bytes. Existing linker informational warning and Java deprecated-API
note remain. Post-build inspection was temporarily blocked by automatic approval
review's usage limit; it resumed successfully on continuation. No install writes.

**Still broken / not tested:** All new menu behaviour is **not tested** in game.
No mouse/text input, held-key repeat or shared item inventory. Rapid/simultaneous
commands may be dropped because the transport retains only the latest command.
Different Skyrim menus may handle these ordinary menu events differently.

**Next:** Install 0.9.1, test NPC dialogue arrows/Enter/Backspace and a pausing
container, then collect logs and compare sequence/action/context and movement hold.

## 2026-10-09 Confirm NPC interaction (0.9.0)

**Changed:** Updated README and STATUS with confirmed NPC interaction. No code or package changes.

**Why:** User reports they can talk to Skyrim NPCs using the new interaction key.

**Tested how:** Collected report logs/scene-report-20261009-214457 and compared SCENE_INTERACT events.

**Result:** Minecraft sent sequence=1 at 21:44:01. Skyrim dispatched that sequence once to target_handle=1058908 with age_ms=0 and logged return. Menu blocked=true followed 16 ms later; blocked=false followed later. Three additional Minecraft input presses were discarded as inactive_or_menu. User observation confirms NPC conversation opened; the native return alone would not prove that.

**Still broken / not tested:** Menu navigation from Minecraft is not implemented. Non-NPC activation, rebinding persistence, held-key suppression and empty-target handling are not confirmed by this report.

**Next:** Implement Skyrim menu input routing as a separate increment when requested. No rebuild needed for this evidence update.

## 2026-10-09 Add Minecraft-to-Skyrim interaction key (0.9.0)

**Changed:** Planned gameplay-first priority in DESIGN. Added remappable R binding through an Options.load mixin and English label. Each eligible press sends a short-lived sequence with native issue timestamp and duplicate/expiry/cooldown/resume checks. Calls Skyrim ActivatePickRef using its crosshair target after pairing/origin admission. Added SCENE_INTERACT_SEND/RECV/RESULT/INPUT/MENU logs. Menu counters suspend bridge movement/look/render control, including dialogue; menus/load/cell are rechecked after activation. Bumped protocol 19/link19, C pt_mc_frame7 and packages to 0.9.0; updated README/ABI/STATUS. No menu navigation, combat or travel work bundled.

**Why:** User wants practical gameplay interaction before further travel/save infrastructure.

**Tested how:** Read local CommonLib activation/singleton/UI definitions and Minecraft signatures with javap; inspected Options constructor ordering. Built tools/build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE), visual game inspection or agent writes to game installs.

**Result:** Final Rust/Fabric builds passed; both 0.9.0 packages produced. First Java build failed because org.lwjgl.glfw is not exposed by the current Minecraft classpath. Replaced it with verified Minecraft InputConstants.KEY_R; second build succeeded. Existing MSVC informational link and Java deprecated-API notices remain. A documentation write using Windows default encoding failed on an arrow character and truncated README. Recovered its full 0.8.9 contents from Git blob 653cfab86a44a601354d0f512072104332a12e4d, reapplied the known restart-acceptance update, then added 0.9.0 instructions using a byte-preserving temporary-file replacement. No commits, staging or Git resets performed.

**Still broken / not tested:** Binding/rebinding, runtime mixin loading, activation/reach, held-key suppression, menu gating/recovery and gameplay regressions are **not tested**. Function return is not object-action confirmation. Native menu input still requires Skyrim's window. Cross-cell doors remain blocked by saved-origin location checks. Target is Skyrim's current pick, not a Minecraft-selected object. Rapid presses can be dropped by latest-slot/cooldown rules.

**Next:** Install both adapters; test a nearby in-place lever/object with R, empty aim, held R, inventory suppression and rebinding. Collect logs directly and compare sequences to native dispatch plus user-observed effects. Menu navigation is the next increment.

## 2026-10-09 Accept scoped 0.8 restart milestone

**Changed:** Updated README, DESIGN and STATUS to record acceptance of the same-location restart milestone. No gameplay code or package changes.

**Why:** User followed the save/quit/restart procedure and reports the placed block and inventory remained present when using Resume Bridge.

**Tested how:** Collected report logs/scene-report-20261009-212809. Compared SCENE_ORIGIN state=saved from skyrim-19424-89071390.log with state=restored from skyrim-17024-89233968.log. Exact text comparison of world ID, cell/worldspace and origin fields returned True. Read current Minecraft pairing/inventory diagnostics.

**Result:** Saved, loaded and restored Minecraft origin [131.30122659251012, 100.0, -45.740949485334895] and Skyrim origin [17405.072265625, -47335.90234375, -141.60723876953125] match exactly. Cell 00009732/worldspace 0000003C and Minecraft UUID bytes match. Minecraft reports paired lineage and 62 dirt on join and later saves. User confirms the block/inventory remained. This meets the narrowed 0.8 restart acceptance scope; visual landmark placement was not inspected by the agent.

**Still broken / not tested:** The earlier pre-save session logged reason=5 after Minecraft changed from below-world Y to Y=100, then reloaded a legacy Skyrim save and created the final persisted origin. Cause of that coordinate change is not established; no claim that every setup attempt passed. Unrestricted altitude/travel, interiors, rollback/save revisions and auto-selection remain 0.9 work. Wrong-world, corrupt-origin and form-resolution failure cases are not gameplay-tested. This does not prove complete save-state equality.

**Next:** Begin 0.9 planning when requested, keeping 0.8.9 as the accepted same-location practice baseline. No rebuild needed for this documentation-only update.

## 2026-10-09 Implement scoped 0.8 restart alignment (0.8.9)

**Changed:** Updated DESIGN before code to finish 0.8 with same-location restart alignment; moved save revision rollback, doors/interiors, location construction and coordinate remapping to 0.9. Added saved_origin.rs and PBOR SKSE serialization with Minecraft world identity, Skyrim cell/worldspace IDs and both origin vectors. Restores after resolving saved form IDs, exact world/location checks and a 140-unit proximity guard. Invalid origin data remains blocked and is marked on subsequent saves. Added world UUID transport (from the existing bridge folder UUID), nonoverlapping 288-byte connection record, protocol 18/link18, pt_mc_frame6, PTWORLD8 hold reason and Minecraft messages. Updated the existing layout fixture header constant (not run), versions, README/ABI/STATUS.

**Why:** Re-anchoring current player positions on every restart shifts construction. A persisted mapping shared by movement/collision/rendering should keep the practice world fixed without implementing full save-history synchronization.

**Tested how:** Inspected current source, callback lifecycle, complete wire offsets, decoding bounds and admission order. Built tools/build-step5b.ps1 -Offline -PackageOnly successfully twice (final build after replacing a zero-length invalid marker with a nonempty marker). No automated tests added/run (Gradle NO-SOURCE), gameplay inspection, game-install writes or save-file edits by the agent.

**Result:** Rust and Fabric builds passed; both 0.8.9 packages created. No compile failures. Existing informational MSVC linker and Java deprecated-API notices remain. A prior attempted plan/edit was stopped by automatic approval review's usage limit before execution; after user continuation, source inspection confirmed no partial implementation and the work resumed.

**Still broken / not tested:** Restart-origin equality, SKSE origin form resolution, legacy first initialization, context/distance rejection and regression gameplay are **not tested in game**. Same-lineage older saves are not revision-matched. One practice cell/origin only; cell changes hold. Minecraft folder renames/copies are not independently identified (copies with the same folder UUID share identity). Save write success is not atomic two-game persistence. Legacy saves without PBOR establish a new initial mapping and cannot recover earlier mapping. 0.8 remains pending acceptance, not declared complete.

**Next:** Install both adapters. Place a marker by a landmark, stop, save Skyrim to a new slot with both games connected, Save and Quit Minecraft, close both and reopen those exact saves. Check marker stays fixed and gameplay resumes; collect logs directly and compare PBOR origin coordinates across the two sessions. Keep previous Skyrim slot for rollback.

## 2026-10-09 Confirm missing/mismatched identity rejection and recovery

**Changed:** Updated README and STATUS with completed 0.8.8 admission-test evidence. No code/package changes.

**Why:** User reports that saving an old unidentified Skyrim slot produced a mismatch, and loading the original paired slot restored the bridge as expected.

**Tested how:** Ran the collector and read SCENE_PAIRING, SCENE_ADMISSION and SCENE_SAVE_ID in report logs/scene-report-20261009-205719.

**Result:** Minecraft reports matched at 20:55:51, missing_skyrim_identity at 20:56:19, mismatch at 20:56:25 and matched at 20:56:47. Skyrim admission is true for paired ID 1E111A315F021C4B82760DF261418370, false for zero ID, false for newly saved ID 4280A21499D6A440A2173B97DDA9308D, then true again for the original ID. Expected Minecraft ID stayed unchanged. User confirms observed blocking and recovery.

**Still broken / not tested:** Save revision equality, persistent origin restoration and automatic world selection remain unimplemented. New unpaired-world first admission is not covered by this test.

**Next:** Continue persistent origin/restored-coordinate design. No rebuild needed for this evidence update.

## 2026-10-09 Confirm matched-save admission repair (0.8.8)

**Changed:** Updated README/STATUS with successful retest evidence. No code/package changes.

**Why:** User reports movement and rendering work normally again.

**Tested how:** Ran the collector and read report logs/scene-report-20261009-205054.

**Result:** Three Minecraft world openings report eligible=true and matched lineage 1E111A315F021C4B82760DF261418370. Skyrim reports SCENE_ADMISSION allowed=true with identical expected/loaded bytes. Combined with user feedback, matched-save startup and normal movement/rendering are confirmed.

**Still broken / not tested:** Mismatched-lineage rejection/recovery and new-world admission remain **not tested**. Persistent origin restoration and save revision matching remain unimplemented.

**Next:** Test a different Skyrim lineage followed by returning to the paired slot. No rebuild needed.

## 2026-10-09 Repair admission startup and identity encoding (0.8.8)

**Changed:** Recorded repair plan. BridgeWorld.marked no longer caches results before server.overworld exists; inactive position sends no longer request pairing metadata. Moved pairing ID from overlapping bytes 216–231 to unused bytes 240–255. Bumped protocol 17/link17 and packages to 0.8.8. Updated README/ABI/STATUS.

**Why:** User's 0.8.7 matched-save test failed. Collected report 20261009-204741 shows eligible=false three times and no pairing/admission events, while Skyrim loaded the correct lineage. Source tracing found new expected-ID lookup running during loading and permanently caching false. Also found Record.encode overwrites ID bytes with render_ms/render_session, which would prevent matching after eligibility was fixed. Both are agent-introduced 0.8.7 bugs.

**Tested how:** Collected logs directly, inspected cache call order and complete Record encode/decode field ownership. Built tools/build-step5b.ps1 -Offline -PackageOnly: Rust/Fabric passed. No automated tests added/run (Gradle NO-SOURCE) or save/install writes.

**Result:** Replacement 0.8.8 packages built; no compile failures. Existing informational MSVC and Java deprecated-API warnings remain. This is the first repair attempt after the failed 0.8.7 gameplay test.

**Still broken / not tested:** Runtime activation/admission recovery is **not tested**. No mismatch test was performed by the user; defer it until matched-save activation works. No save or pairing migration required.

**Next:** Install both adapters and retest only existing paired saves. Collect eligible/matched/admission logs before further changes.

## 2026-10-09 Verify pairing and enforce native admission (0.8.7)

**Changed:** Collected report 20261009-204217 and confirmed user-tested pairing. Planned native admission in DESIGN, then added expected ID to position samples, native pre-control comparison, identity-only rejected replies and admission flag in world snapshots. Java discards non-admitted geometry and suppresses scene/overlay export until fresh admitted coverage. Updated protocol to 16/PTWORLD7/link16, C entry to pt_mc_frame5, versions and documentation.

**Why:** Diagnostic pairing warned on mismatch but previously allowed mismatched Minecraft worlds to control Skyrim. Native checks must precede movement, look and rendering while still permitting first-time pairing discovery.

**Tested how:** Read SCENE_PAIRING in collected logs: unpaired at 20:38:44, paired/matched at 20:39:21, matched again at 20:39:36, all ID 1E111A315F021C4B82760DF261418370. Inspected native and Java transport/admission paths; built with tools/build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE) or game-install writes.

**Result:** Pairing/reopen confirmed by user and logs. Rust and Fabric builds passed and 0.8.7 packages were produced. Runtime 0.8.7 admission is **not tested**. Existing informational MSVC linker and Java deprecated-API warnings remain.

**Still broken / not tested:** Matching, mismatch, recovery and initial pairing admission need gameplay testing. Same-lineage older save revisions still pass. No save revision pairing, restored origin, automatic world selection or construction remapping. Released Skyrim can move normally under its own control.

**Next:** Install both adapters, check matched movement, mismatched hold and recovery to the original paired Skyrim save; collect logs directly when testing completes.

## 2026-10-09 Add explicit Minecraft save pairing (0.8.6)

**Changed:** Planned diagnostic pairing before code. Added published Skyrim lineage bytes to world snapshots, changed protocol to 15/PTWORLD6/link15 and size to 397456 bytes. Added BridgePairing confirmation, versioned world-root metadata, create-only writes, validation and matched/mismatch/missing/error logs and messages. Updated versions, README, DESIGN, ABI and STATUS.

**Why:** Associate the Minecraft bridge world with the Skyrim save lineage before implementing automatic save selection or restored coordinate origins.

**Tested how:** Inspected serialization, transport and metadata paths; built `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Rust and Fabric passed. No automated tests added/run (Gradle NO-SOURCE), no agent writes to game installs or existing saves.

**Result:** Both 0.8.6 packages built. Existing informational MSVC link and Java deprecated-API warnings remain. No compile failures. Earlier source read was blocked by automatic approval review's usage limit, not an unsafe-action determination; it executed successfully after the user resumed on Oct 9.

**Still broken / not tested:** Pair/decline/reopen/mismatch and IO failure paths are **not tested in game**. Confirmation uses the most recently received snapshot and does not provide a native handshake. Mismatch is a warning, not a movement gate. No origin restoration, save revision matching, plugin fingerprint, automatic save selection or pairing-file repair. Runtime writes only the new metadata file after user acceptance, never overwriting it. SKSE WriteRecord success is not whole-save durability proof.

**Next:** Install both adapters, accept pairing with the identified Skyrim save, quit/reopen Minecraft and check matched without re-prompting. Collect logs directly after the user finishes. Implement admission handshake before claiming mismatched worlds are prevented from controlling the puppet.

## 2026-10-08 Verify Skyrim save identity across restart (0.8.5)

**Changed:** Updated README and STATUS with confirmed save-identity persistence. No code or package changes.

**Why:** User saved an existing Skyrim game to a new slot, fully restarted Skyrim through SKSE, loaded that slot and saved to another new slot. Minecraft was not run and is unnecessary for this identity test.

**Tested how:** Ran the collector, producing scene-report-20261008-213256. The collector includes only the latest native session, so also read SCENE_SAVE_ID entries in the two most recent project-local Skyrim logs: skyrim-27040-3073265.log and skyrim-14360-3129937.log. Minecraft files in the report are older and were not treated as evidence for this test.

**Result:** Process 27040 logged missing_record, then created/saved ID 1E111A315F021C4B82760DF261418370. Process 14360 logged loaded/saved with the exact same ID. Identity persistence across a full process restart and subsequent Save As is confirmed.

**Still broken / not tested:** Minecraft pairing, restart-origin restoration, save revision matching and malformed-record handling remain unimplemented or not tested, as applicable. This confirms lineage identity only.

**Next:** Pair Minecraft save metadata to the Skyrim lineage and define revision/origin restoration checks. No rebuild needed for this documentation update.

## 2026-10-08 Add Skyrim save-lineage identity (0.8.5)

**Changed:** Planned save identity in DESIGN before code. Added save_identity.rs using SKSE serialization version 4 callbacks, a versioned 16-byte bridge ID generated by Windows CoCreateGuid, and SCENE_SAVE_ID diagnostics. Revert clears state; load restores identity; save creates one only when absent. Invalid/duplicate/unsupported records block writing our record for that loaded state. Registered callbacks after messaging succeeds, updated package versions and previously stale SKSE plugin version metadata, and updated README/ABI/STATUS. Protocol remains 14.

**Why:** Cell/worldspace IDs cannot distinguish Skyrim playthroughs. Persistent pairing needs a save-lineage identity before origin restoration can be safe.

**Tested how:** Inspected local CommonLib SKSE interface IDs, callback signatures and struct field order. Built `.\tools\build-step5b.ps1 -Offline -PackageOnly`: Rust and Fabric passed. No automated tests added/run (Gradle NO-SOURCE), game-install writes or save-file edits by the agent.

**Result:** 0.8.5 packages created. Existing informational MSVC linker warning remains; no compile failures. Disk writes are performed by SKSE only when the user saves in-game.

**Still broken / not tested:** Save/restart/load identity persistence and regression gameplay are **not tested**. This does not yet pair Minecraft, restore origins, fingerprint plugin load order or identify a save revision. Save copies/branches inherit identity; saves predating the record get a new one. Invalid record handling does not abort the game's overall save or preserve bad record bytes. No migration of existing saves by the agent.

**Next:** Install, save Skyrim to a new slot, fully restart and load that slot, save again, then directly collect logs to compare saved/loaded IDs. Keep the previous slot for rollback.

## 2026-10-08 Record pause-alignment gameplay feedback

**Changed:** Updated README and STATUS with user-confirmed behaviour and collected evidence. No gameplay code or package changes.

**Why:** User reports 0.8.4 behaves exactly as expected in the pause test.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1` under standing authorization and read SCENE_ALIGNMENT events from report `logs/scene-report-20261008-212546`.

**Result:** Three suspended origins were reused with reason=same_session_and_position. Two were rejected with reason=skyrim_moved_while_suspended: one with substantial horizontal movement, another with a 5.55-unit vertical change. Initial connection created an origin as expected; the final suspension has no recorded resume.

**Still broken / not tested:** User confirms the observed result, but logs do not prove every menu pause retains alignment. Cause of vertical movement during suspension is not established. Full-restart alignment, save binding and door travel remain unimplemented/not tested.

**Next:** Continue persistent alignment planning while retaining the documented movement-during-suspension limitation. No rebuild needed for this documentation update.

## 2026-10-08 Preserve alignment across temporary pauses (0.8.4)

**Changed:** Added a DESIGN plan before code. Skyrim retains an in-memory suspended origin while releasing normal camera/gravity/render/collision control. Resume requires matching Minecraft session/world, no Skyrim cell/load reset, and Skyrim player displacement at most one unit since suspension. Logs SCENE_ALIGNMENT suspended/resumed/new_origin with reasons. Bumped packages to 0.8.4, retained protocol 14, and updated README/STATUS.

**Why:** Previously every menu or missing sample discarded the origin, allowing a new relative alignment on resume and potential construction drift.

**Tested how:** Inspected release/reset/sample lifecycle and built `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Rust/Fabric builds succeeded; no automated tests added/run (Gradle NO-SOURCE), no game-install writes.

**Result:** 0.8.4 packages built. Existing informational MSVC linker warning remains. No compile failures.

**Still broken / not tested:** Pause/resume stability and regression gameplay are **not tested**. Movement while disconnected rejects reuse and can still realign construction. No restart persistence, save binding, door travel or construction remapping. Sprint unchanged.

**Next:** Install, place a block beside a landmark, pause/resume each game while stationary, and check it remains fixed. Collect SCENE_ALIGNMENT decisions directly when the user finishes testing.

## 2026-10-08 Collect and compare 0.8.3 location logs

**Changed:** Recorded user authorization in AGENTS.md to run the collector and read collected logs whenever needed. Updated README with measured location-feed results; no code/package changes.

**Why:** User completed testing and authorized direct log collection.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1`; read report `logs/scene-report-20261008-212004` and compared SCENE_LOCATION_SEND/RECV pairs.

**Result:** Both sides match cell_id=00009732 and worldspace_id=0000003C at epoch=1 snapshot=1 and epoch=3 snapshot=17. Movement logs show an initial 6444 ms wait for a snapshot and a later 140 ms wait, both returning to ready. The cause of the later reset is not established by this report.

**Still broken / not tested:** Full-restart stability and location transitions are **not tested** by these same-session pairs. Persistent alignment/save binding remains unimplemented. No rebuild needed for documentation changes.

**Next:** Use direct collection for subsequent diagnostics; continue stable alignment work in a separate increment.

## 2026-10-08 Export Skyrim location identity (0.8.3)

**Changed:** Planned the first stable-location increment in DESIGN. Added cell/worldspace form IDs to world snapshots and Java receipt diagnostics, using reserved header bytes 112/116. Added SCENE_LOCATION_SEND/RECV transition logs. Bumped protocol to 14/PTWORLD5/link14 and package versions to 0.8.3. Updated README, ABI and STATUS.

**Why:** Persistent construction alignment needs game location identifiers rather than process addresses before save binding or coordinate remapping can be implemented.

**Tested how:** Inspected local CommonLib TESForm/TESObjectCELL layout, including the AE >=1.6.629 offset change. Built `.\tools\build-step5b.ps1 -Offline -PackageOnly` successfully for Rust and Fabric. No automated tests added/run (Gradle test NO-SOURCE), game inspection or game-install writes.

**Result:** Both 0.8.3 packages built in dist/step5b. Existing MSVC informational link warning and Java deprecated-API notice remain. No compilation failures. Snapshot size stays unchanged; incompatible older adapters are isolated by the new namespace.

**Still broken / not tested:** Runtime location reads, matching IDs across both logs and restarts, and regression gameplay are **not tested**. No save binding, construction remapping, door travel or stable alignment yet. Form IDs depend on plugin load order and cannot identify a save. Sprint work is deferred at the user's request.

**Next:** Install both adapters, collect a report in the same Skyrim cell, restart and compare location IDs. Then design persistent binding with plugin/save identity and alignment metadata before moving construction.

## 2026-10-08 Sprint report analysis (0.8.2)

**Changed:** Recorded analysis only; no gameplay code or package changes.

**Why:** User reported no apparent sprint speed increase and supplied scene-report-20261008-211142 after installing the current build.

**Tested how:** Read the three supplied report files. Matched Minecraft SCENE_BOUNDS snapshot IDs to native SCENE_SEND timestamps and calculated horizontal displacement divided by elapsed time across ten-snapshot windows. Read coverage transitions and Skyrim scan timings. No game inspection or automated tests.

**Result:** The 21:11:17–18 window measured 4.329 blocks/s. Later windows at 21:11:22–26 measured 5.261, 5.800, 5.627 and 5.716 blocks/s, showing faster movement consistent with sprinting. The only logged movement hold was the initial 548 ms wait for the first snapshot; no later coverage holds were recorded. Skyrim logged 256 scans, averaging 142.13 ms (109–364 ms). These are snapshot-window averages, not sprint-state measurements.

**Still broken / not tested:** The report does not record sprint key/state, so activation on every attempt is not proven. Sprint FOV and Skyrim sprint animation are not bridged. Some slower windows could reflect turns, obstacles or input changes; their cause is not established. The earlier supplied 210742 report used an older movement format and also showed faster movement near its end. A first read-only calculation command had a PowerShell brace error; corrected calculation succeeded.

**Next:** Explain the measured speed increase; investigate explicit sprint-state/input diagnostics if speed still fails on a repeatable attempt.

## 2026-10-08 Predict collision coverage ahead of movement (0.8.2)

**Changed:** Planned bounded predictive scan placement in DESIGN before code. Native exporter derives velocity from distinct rendered samples, rejects gaps/teleport-like motion, and shifts each new scan ahead using prior scan duration plus 150 ms (horizon 200–500 ms; lead capped at 1.5 horizontal/.75 vertical blocks). Keeps the scan fixed until completion and retains radius/density/work/freshness limits. Added collected SCENE_WORLD_SCAN timing/lead/publication diagnostics. Split Java coverage failure reasons and added hold duration/requested-bounds/coverage diagnostics. Updated versions to 0.8.2, retaining protocol 13, README/ABI/STATUS and recorded confirmed save resuming.

**Why:** Refreshing only around the player's scan-start position lets sustained motion approach stale coverage edges. A bounded lead can provide more forward coverage without increasing the ray count before measuring performance. Logs must distinguish bounds holds from stale/missing snapshots.

**Tested how:** Inspected exporter, sample timestamps, snapshot lifetime and Java movement gate. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE), gameplay-log reads, visual game inspection or game-install writes. Final package files confirmed present.

**Result:** Rust release and Fabric build passed; both 0.8.2 ZIPs are in dist/step5b. Existing informational MSVC and Java deprecated-API warnings remain. No compile failures. Normal shell setup failed before execution; reviewed escalated project-only execution succeeded. Existing SCENE_ collector includes the new diagnostics without script changes.

**Still broken / not tested:** Smoother travel, sharp turns, jumps/drops, lead stability and hold reductions are **not tested in game**. Radius remains four blocks and sampling can miss thin shapes. Sudden motion and missing coverage still hold. No larger region, snapshot merging, exact shape export, scale/reach change, fluid containment or persistent Skyrim save/coordinate mapping. Hold durations measure check-state transitions, not rendered input latency.

**Next:** Close games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Resume the same bridge and follow README's walking/sprinting/turning/slope/stair/jump/short-drop route. Briefly pause/resume Skyrim and verify safe hold behavior. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines) and supply the path plus observed pauses. Compare SCENE_WORLD_SCAN lead/timing with client/server SCENE_BRIDGE_MOTION reasons and hold_ms before expanding coverage further.

## 2026-10-08 Resume existing bridge saves and inventory diagnostics (0.8.1)

**Changed:** Planned this save-workflow increment in DESIGN. Added Resume bridge on Singleplayer, asynchronously reading Minecraft save summaries and offering the latest marked save by name/folder before opening through the normal loader. Added a new-world confirmation explaining separate inventory. Updated bridge messages. Added server join/save-return inventory logs with save/player identity, slot item IDs/counts, six equipment slots and XP. Updated versions to 0.8.1, retaining protocol 13, and updated README/ABI/STATUS. Recorded user-confirmed basic empty-world success.

**Why:** User raised inventory persistence when starting a world. New Skyrim bridge is first-time setup; existing Minecraft saves already own their player data and builds. Resuming must not silently create a fresh save or transfer inventory between unrelated saves.

**Tested how:** Inspected cached Minecraft save-summary, world-loader, confirmation-screen and player/inventory APIs. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE), gameplay-log reads, game inspection or game-install writes. Confirmed both 0.8.1 ZIP package files exist.

**Result:** Final Rust/Fabric builds passed. Initial Java compile failed because screen handling moved from Minecraft to Minecraft.gui; corrected using inspected signatures and rebuilt successfully. Existing informational MSVC and Java deprecated-API warnings remain. Two normal shell calls failed before execution with sandbox setup-refresh errors; reviewed escalated project-only commands succeeded. No save migration or independent inventory writer was introduced.

**Still broken / not tested:** Resume UI, runtime hooks, missing/locked/multiple-save cases and inventory persistence round trip are **not tested in game**. Diagnostics do not compare item components, enchantments, ender chests or disk durability. Resume selects the most recently played Minecraft bridge, not the matching Skyrim save. Stable construction alignment, paired save/rollback, long-travel coverage and height-window mapping remain pending. Marked saves still rely on their generated display-name marker.

**Next:** Close games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Resume the existing bridge, note distinctive inventory/equipment/XP, Save and Quit to Title, then Resume bridge again and compare. Keep Minecraft open and run .\tools\collect-scene-logs.ps1 (three COLLECTED lines); provide that report. Compare save_return and subsequent join_return for the same save/player. New bridge > Cancel should leave the existing save intact. Travel coverage remains the next separate implementation increment.

## 2026-10-08 Empty bridge world and local movement gate (0.8.0)

**Changed:** Planned step 1 details before code. Added New Skyrim bridge to Singleplayer, creating a unique marked save via Minecraft world creation with 128 air layers, void biome, no structures/decoration/lakes/chest and no natural mob spawning. Existing saves are not converted; active position and world suppression are gated to eligible saves. Added startup-ground and fresh-coverage movement hold on client/server, clearing velocity/fall distance while continuing stationary alignment samples. Added scan centre/radius to world snapshot headers, protocol 13 / PTWORLD4, coverage validation and SCENE_BRIDGE_WORLD/CHUNK/COVERAGE/MOTION logs. Packaged 0.8.0; updated README, DESIGN, ABI and STATUS.

**Why:** Minecraft generated terrain must not constrain Skyrim travel. The first implementation removes that second landscape entirely using a new save, while preserving player construction and Skyrim collision. An empty world needs a bootstrap hold so it does not drop the player before collision is available.

**Tested how:** Inspected cached 26.3 world-creation, flat generator/spawn, movement, gamerule and message APIs. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added or run; Gradle test tasks NO-SOURCE. No game visual inspection, gameplay-log reads or game-install writes. Confirmed final 0.8.0 package files exist. Collector already includes SCENE_BRIDGE_ via its existing filter.

**Result:** Rust release and final Fabric builds passed. First Java build failed on the removed displayClientMessage method; corrected using inspected sendSystemMessage/sendOverlayMessage APIs, then rebuilt successfully and rebuilt final diagnostics. Existing MSVC informational and Java deprecated-API warnings remain. Packages are dist/step5b/Passthrough-Minecraft-0.8.0.zip and Passthrough-Skyrim-0.8.0.zip. Matching JAR and both DLLs are required.

**Still broken / not tested:** UI/mixin application, creation/empty chunks, spawn height, readiness/hold/resume, server movement correction and actual Skyrim-only terrain traversal are **not tested in game**. The scan radius is still four blocks; fast movement/drop edges may hold. No proactive coverage expansion, infinite altitude, coordinate-window remap, stable construction across Skyrim origin/cell changes, old-build migration or matched saves. Normal-axis placement gaps, scale/reach calibration and fluid containment remain pending. Initial chunk counts run after terrain generation; expected zero is not yet observed in gameplay.

**Next:** Close both games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Stand on Skyrim solid ground; Minecraft Singleplayer > New Skyrim bridge. Expect initial waiting then ready; walk/jump a local route without Minecraft terrain, place/break stone to check deliberate collision, and test holding when Skyrim pauses/stops. Check ordinary saves show bridge inactive and retain normal Minecraft rendering. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines) and supply the report. Check eligible=true, generated non_air_blocks=0, coverage bounds and both-side motion transitions before moving to travel coverage improvements.
## 2026-10-08 Plan removal of generated Minecraft terrain from bridge gameplay

**Changed:** Replaced the practice-terrain note in DESIGN with the next-priority plan: dedicated empty bridge save, bridge-world gating and safe startup, travel-ready collision coverage, stable Skyrim-coordinate persistence with bounded local windows, then construction verification and resumed fluid/scale work. Added README limitation/priority notice. Documentation only.

**Why:** User explicitly requires Skyrim travel to be independent of Minecraft caves/mountains. Retaining generated terrain was never the intended finished behavior. Removing only rendering or one collision query leaves other Minecraft terrain interactions in place.

**Tested how:** Read current design, collision mixin and bridge lifecycle references. No tests, build, game inspection or gameplay-log reads. User says buckets appear to work but this remains tentative.

**Result:** Concrete implementation order and acceptance criteria are recorded. Existing worlds are preserved; a separate empty save eliminates generated terrain across physics, rendering, targeting and fluids. Skyrim collisions and deliberately placed blocks still constrain movement. Coverage/lifecycle and coordinate limits are explicit follow-up requirements rather than a claim that a void world alone solves every travel issue.

**Still broken / not tested:** Current binaries still contain the reported restriction. New empty-world setup, startup hold, coverage changes, coordinate-window remapping and persistent world/cell binding are unimplemented and **not tested**. Scale/reach and fluid containment remain pending.

**Next:** Implement only step 1 of the new plan when requested. No rebuild/reinstall or gameplay test needed for this plan. Future implementation must provide exact world-setup controls and use .\tools\install-build.ps1 / .\tools\collect-scene-logs.ps1 with expected three INSTALLED / three COLLECTED lines, plus logged empty chunks and correct Skyrim grounding before movement resumes.
## 2026-10-08 Filled bucket placement on Skyrim surfaces (0.7.3)

**Changed:** Planned bucket targeting in DESIGN before code. Added water/lava buckets to the existing contact-preserving target path. Added an Item.useOn hook restricted to those exact items, air destinations and offline integrated-server sessions. Reused intent validation, then called Minecraft's bucket emptying/extra-content and item replacement helpers at the destination. Added SCENE_BUCKET diagnostics. Updated versions to 0.7.3, retaining protocol 12. Updated README/ABI/STATUS and recorded scale/reach feedback.

**Why:** The user accepts block placement for this iteration and requests the next design feature. Buckets need their own item-use path. Scale looks large and reach feels short to the user; calibration is pending rather than mixed into this feature.

**Tested how:** Read cached 26.3 BucketItem, ItemStack and client item-use signatures/call paths to preserve bucket inventory replacement and avoid double item-use statistics. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added or run (Gradle NO-SOURCE). No gameplay-log reads, visual game inspection or game-install writes.

**Result:** Rust release and Fabric build succeeded; both 0.7.3 ZIPs are packaged in dist/step5b. Existing informational MSVC and deprecated Java API warnings remain. No build failures. Normal Minecraft bucket targets, empty and special buckets remain on vanilla paths. Passing null hit to emptyContents prevents fallback into a different adjacent cell.

**Still broken / not tested:** Bucket terrain targeting, runtime hook, survival empty-bucket return, creative retention, source pickup, prediction/server correction and regressions are **not tested in game**. Fluids still do not interact with Skyrim terrain; no invisible support or simulation changes. Scale remains 70 Skyrim units/block, normal reach and collision radius four. True surface normals, fluid containment and previous rendering issues remain pending.

**Next:** Close both games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Follow README 0.7.3: give water/lava buckets and pour on separate Skyrim floor targets without starter blocks; expect fluid at the selected cell but still flowing through Skyrim. Check ordinary bucket use, waterlogging, creative retention, survival empty-bucket return and source pickup, plus stone placement/movement/HUD. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines); supply the report and compare placement intent IDs with SCENE_BUCKET results. Terrain-aware fluid simulation is the next separate step.
## 2026-10-08 Separate terrain contact from placement packet (0.7.2)

**Changed:** Wrote the resumed design before code. Removed ray/grid-plane tangential shifting. Anchored destination cells to actual contact and clamped a separate packet point inside the cell. Added an offline intent mailbox (16 entries, 500 ms expiry, one-time server consumption, reset clearing) carrying contact/origin and player/session/hand/item identity. Revalidate actual surface on both sides; vanilla still owns placement, prediction and inventory. Added intent IDs and mapping=contact_cell logs. Bumped packages to 0.7.2, retaining protocol 12. Updated README/ABI and rewrote STATUS using templates/STATUS-handoff.md supplied by the user.

**Why:** The user resumed after two inadequate iterations. Earlier logs showed that solving packet geometry by moving the destination caused placement near the player. The contact and packet point now have separate roles.

**Tested how:** Inspected cached 26.3 prediction/item-use call paths; performUseItemOn runs before the normal packet is sent. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added or run (Gradle NO-SOURCE), game inspection, new gameplay-log reads or game-install writes.

**Result:** Rust release and Fabric builds succeeded. Both 0.7.2 ZIPs exist in dist/step5b. Existing MSVC informational and Java deprecated-API warnings remain. No build failures. An apply_patch delete/add combination was rejected and replaced by a direct write; a README heading lookup failed and was corrected using existing headings. The handoff template was under templates, not missing as previously reported.

**Still broken / not tested:** In-game targeting, client/server intent association, prediction/rollback, expiry and inventory behavior are **not tested**. Sampled box-edge faces, four-block coverage and conservative overlap rejections remain. Normal-axis rounding still leaves sub-block gaps. Buckets, fluid containment and true surface normals are not added. This build is not claimed to solve all placement failures.

**Next:** Close games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Follow README's near/far floor and wall procedure plus ordinary adjacency and survival consumption checks. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines) and supply the report. Compare intent IDs and contact-anchored cell coordinates against SCENE_PLACE_RESULT.
## 2026-10-08 Confirm displaced terrain placement; stop and hand off

**Changed:** Created STATUS.md and marked the placement blocker in README/DESIGN. No code change, build or install. STATUS-handoff.md was missing; a file search found no handoff template.

**Why:** User reports normal Minecraft adjacency works, but direct Skyrim placement is pulled toward the player. Two real placement iterations (0.7.0/0.7.1) have not met the goal; AGENTS.md requires stopping and documenting rather than trying more variations.

**Tested how:** Read only the three explicitly supplied scene-report-20261008-161512 files. Summarized placement decisions and compared native received scene metadata with sent metadata. No automated code tests, game inspection or other log reads.

**Result:** At snapshot 514 the sampled contact is 3.007263 blocks from the eye but the selected cell centre is only 0.872835 blocks away, with 1.589780 blocks horizontal displacement from the contact. Snapshot 707 records client/server successful dirt placement into the same displaced destination. Server decisions: 13 accepted, 5 occupied_terrain, 1 no_fresh_surface. All 875 received scene records match sent metadata/checksums. Source and logs identify 0.7.1's rounded grid-plane projection as a cause of destination movement; matching transport/server decisions do not establish correct placement.

**Still broken / not tested:** Direct terrain placement remains incorrect. Other misses, sampled-box edge normals and vanilla failures remain separate uncertainties. No replacement fix is implemented or tested. Wall placement is unconfirmed. Buckets/fluid containment remain pending.

**Next:** Review STATUS.md's proposal to separate terrain contact, destination cell and packet point, with explicit server validation and preserved surface normals where needed. No further patch under this investigation. No rebuild/reinstall needed. Future authorized reproductions: .\tools\collect-scene-logs.ps1, expecting three COLLECTED lines; read only user-supplied reports.
## 2026-10-08 Correct shallow-angle floor targeting (0.7.1)

**Changed:** Planned and corrected grid targeting: intersect the aiming ray with the outward rounded grid plane before choosing tangential cell coordinates. Previously, a shallow ray could miss the cell directly above the original contact and silently lose its target. Added throttled SCENE_TARGET_MISS and target decision/eye/reach/distance logs. Updated versions, README, DESIGN and ABI; protocol and collision-feed coverage are unchanged.

**Why:** User reports floor placement works only when very close. Source inspection found a concrete geometric rejection that can cause this; it is not established as the sole cause.

**Tested how:** Read the latest Minecraft log from the installation path named in AGENTS.md. Initial sandbox read was denied; escalated read succeeded. Logs show client/server occupied_terrain rejections, accepted targets followed by vanilla Fail, and matching successful dirt placement. Inspected the four-block-radius exporter and Java targeting. Built `.\tools\build-step5b.ps1 -Offline -PackageOnly`; no automated tests added or run (Gradle NO-SOURCE). No game visual inspection or installation writes.

**Result:** Rust release and Fabric builds succeeded, packaging both 0.7.1 ZIPs. Existing informational MSVC and Java deprecated-API warnings remain. No build failure. Removed the requirement that a shallow ray cross the cell vertically above the original floor hit, while retaining reach and obstruction checks against the actual surface.

**Still broken / not tested:** The range improvement is **not tested in game**. Four-block collision coverage still limits targeting; increasing it is not part of this correction. Terrain-overlap and vanilla placement failures remain possible, with the cause of the observed vanilla Fail results unconfirmed. Sampled tile edges, uneven ground and fractional-height grid gaps remain limitations. Wall placement has not been confirmed by the user. Buckets and fluid containment remain pending.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Follow README's 0.7.1 floor procedure, aiming at progressively shallower angles within coverage. Hold a failed aim steady for two seconds and click once, then run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Compare SCENE_TARGET/MISS with client/server placement decisions and final states before attributing any remaining short range to a particular cause.
## 2026-10-08 Direct placement against Skyrim terrain (0.7.0)

**Changed:** Expanded the step 6a plan before implementation. Tagged terrain and NPC collision shapes separately. Added terrain ray targeting after Minecraft's normal pick, preserving nearer vanilla hits. Selected the first whole grid cell outside the sampled surface, with a packet click point at the ray's entry into that cell. Added client/server air-target validation for fresh surfaces, reach, cell/face agreement, vanilla occlusion and terrain overlap, then retained normal BlockItem placement. Added SCENE_TARGET/SCENE_PLACE/SCENE_PLACE_RESULT logs. Updated package versions, README and ABI; retained protocol 12.

**Why:** User requested the next step after observing that block placement and water still only interact with Minecraft blocks. This increment implements solid-block placement first; fluid interaction remains separate.

**Tested how:** Read existing collision code and cached Minecraft signatures/call paths for targeting, BlockPlaceContext, item use and server packet validation. Built and packaged twice with `.\tools\build-step5b.ps1 -Offline -PackageOnly`, after the final call-path corrections. No automated tests added or run; Gradle test tasks were NO-SOURCE. No game inspection, game-install writes or extracted assets saved.

**Result:** Both builds succeeded. Final dist/step5b packages are Passthrough-Minecraft-0.7.0.zip and Passthrough-Skyrim-0.7.0.zip. Existing Java deprecated-API and informational MSVC warnings remain. Inspection caught two initial design mistakes before release: Entity.pick is not the current crosshair call path, and using the actual terrain contact as the packet point can violate vanilla's destination-cell distance check. Replaced the hook with Minecraft.pick and separated actual surface from the packet point. Initial shell rg wildcard paths produced path errors; subsequent searches used directory/glob arguments. No build failures.

**Still broken / not tested:** Runtime mixin application, direct terrain placement, packet acceptance, server correction, lifecycle changes and gameplay regressions are **not tested in game**. Grid alignment can leave gaps under one block. Thin/irregular sampled surfaces may be missed or cause occupied_terrain rejection. Support-dependent blocks retain vanilla restrictions. Buckets, water containment, terrain mining and Skyrim NPC collision with placed blocks are not added. Deferred cracks/camera issues remain.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Use `/gamemode creative` and `/give @s minecraft:stone 64`; place on Skyrim ground and walls without a Minecraft starter block. Check existing block placement, breaking, movement, NPC exclusion, reach, compass and drops. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Compare SCENE_TARGET with client/server SCENE_PLACE decisions and SCENE_PLACE_RESULT state. Bucket targeting and terrain-aware fluid simulation follow separately.
## 2026-10-08 Record terrain placement and fluid interaction limits

**Changed:** Recorded the user's 0.6.8 observations in README and added the next interaction sequence to DESIGN: terrain targeting/solid-block placement, bucket targeting, then fluid flow against terrain. Documentation only.

**Why:** Skyrim geometry currently supplements entity collision queries. It does not replace the Minecraft block data used by placement targeting and fluid simulation.

**Tested how:** User reports visible water flowing through Skyrim terrain and placement requiring existing Minecraft blocks. Read current collision mixin, source references and design. No new tests, build or game inspection.

**Result:** Both observations match unimplemented interaction paths. This is not evidence that movement collision has stopped working. No code fix has been made.

**Still broken / not tested:** Direct placement on Skyrim surfaces and fluid containment by Skyrim terrain are unimplemented. Lava, waterlogging and broader fluid rendering remain **not tested** by this report.

**Next:** Implement surface targeting and solid-block placement first, with matching client/server validation and logs. Buckets and terrain-aware fluid simulation follow separately. No rebuild or reinstall for these documentation changes. For additional evidence run `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines.
## 2026-10-08 Step 5b water and lava surfaces (0.6.8)

**Changed:** Planned fluid surfaces in DESIGN before code. Added FluidCapture using Minecraft's FluidRenderer for still/flowing/waterlogged cells, section-coordinate rebasing, atlas remapping, tint/light and opaque/translucent routing. Added fluid counts at scene offsets 120/124, validation and SCENE_FLUID/SCENE_FLUID_DRAW plus send/receive diagnostics. Bumped adapters to 0.6.8 and protocol 12. Updated README and ABI. Glass/ice cracks remain deferred per the request to move to other features.

**Why:** Add another kind of world content using the existing geometry and pre-HUD rendering paths.

**Tested how:** Inspected local SkyCraft reference and cached Minecraft FluidRenderer/VertexConsumer signatures and vertex call. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game inspection, installation writes or tracked game assets.

**Result:** Rust release and Fabric builds succeeded; dist/step5b contains both 0.6.8 ZIP packages. Existing informational MSVC import-library and Java deprecated-API warnings remain. Initial lookup of a nonexistent BlockRenderDispatcher class failed; direct FluidRenderer inspection supplied the needed API. No build failures.

**Still broken / not tested:** Fluid appearance, flow/waterlogging, negative-coordinate alignment, transparency, depth, timing and regressions are **not tested in game**. Textures use static frames. Fluids follow Minecraft blocks, not Skyrim geometry; no underwater effects or Skyrim-water import. Existing glass/ice cracks and camera/world-bleed issues remain deferred.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Follow README's creative water/lava basin and flowing-water procedure, checking Skyrim walls and compass, waterlogging, drops and opaque cracks. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Compare fluid counts across matching send/receive/draw snapshots. Both adapters must use protocol 12.
## 2026-10-08 Confirm 0.6.7 and record missing glass/ice cracks

**Changed:** Updated README and DESIGN with user-confirmed drop-extension success and missing glass/ice breaking animation. Documentation only.

**Why:** Separate the working transparent/non-full drops from the remaining crack-rendering limitation.

**Tested how:** User gameplay report and inspection of CrackCapture's MODEL/isSolidRender/full-collision-shape filter. No new logs supplied or automated tests run.

**Result:** Drop extension is user-confirmed. Cracks still deliberately skip non-opaque/non-full blocks; glass and ice are outside the implemented crack path. No code fix has been made for that limitation.

**Still broken / not tested:** Glass/ice cracks are absent. Numerical 0.6.7 comparison and broader rendering/regression cases are **not tested** in this report.

**Next:** Extend crack geometry and transparent-surface ordering as a separate rendering increment. No rebuild or reinstall needed for these notes. Future logs: `.\tools\collect-scene-logs.ps1`, expecting three COLLECTED lines.

## 2026-10-08 Extend transparent and non-full drops (0.6.7)

**Changed:** Assessed this as a bounded extension of the existing model/transparency paths and planned it in DESIGN. BlockDropCapture now gathers directional and unculled default-model quads (maximum 96) without the opaque/full-cube restriction. Opaque/cutout faces remain in the item prefix; translucent drop faces join the sorted placed-face collection. Added dual-buffer rollback and removal of old dynamic faces on publication retries. Added explicit model and translucent-item counts at header offsets 112/116, revised validation/logging, and bumped adapters to 0.6.7 / protocol 11. Updated README and ABI.

**Why:** User authorized extending glass/pane/ice drops if it did not require a difficult redesign. Existing shaders, pre-HUD rendering and blending can handle the new geometry; no renderer redesign was needed.

**Tested how:** Inspected existing geometry, transparency, capacity and retry paths. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game visual inspection or game-folder writes.

**Result:** Rust release and Fabric builds succeeded; packages are dist/step5b/Passthrough-Minecraft-0.6.7.zip and Passthrough-Skyrim-0.6.7.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. No build failures. The confirmed HUD hook and shader passes are retained; transparent drops sort together with placed transparent faces rather than using an independent layer.

**Still broken / not tested:** Glass/pane/ice model appearance, transparency, pickup, mixed-scene sorting, buffer rollback/capacity and regressions are **not tested in game**. Default-state models can differ from dedicated item models; panes use default-state geometry. Special block-entity renderers, item-specific components, out-of-block geometry and models over 96 quads use the icon fallback when available. Existing approximate sorting, both-sided opacity, roughly 10 Hz updates and work-budget limits remain. No broader rendering completeness is claimed.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Give/drop glass, glass panes, ice, stained glass and red sand using README's commands. Expect 3D drop geometry, transparent ice/stained glass, red sand still 3D, normal pickup and the compass staying visible. Check apple icons, placed glass and cracks. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines); compare models/items, opaque item_vertices and translucent_item_vertices on matching native snapshots. Client SCENE_ITEM should report block_model geometry and translucent counts where applicable. Both adapters must use protocol 11.

## 2026-10-08 Confirm HUD layering and record drop-model limits

**Changed:** Updated README and DESIGN with the user's confirmation that the HUD is no longer blocked, glass/panes/ice drops remain 2D, and red sand drops remain 3D. Documentation only.

**Why:** Distinguish the successful compass fix from the existing limits of dropped-block rendering.

**Tested how:** User gameplay report and inspection of BlockDropCapture eligibility checks. No new logs supplied or automated tests run.

**Result:** HUD layering is user-confirmed. The 3D drop exporter requires solid rendering, full-cube collision, one quad per direction and no translucent faces; excluded models use the icon fallback. Placed-block transparency does not change those eligibility rules. Reported glass/pane/ice behavior is consistent with this implementation, rather than evidence of a new regression.

**Still broken / not tested:** Translucent/non-full 3D drops remain unimplemented. Broader menu/HUD-hidden cases and numerical hook logs are **not tested** in this report.

**Next:** A separate increment can extend dropped models to transparent and non-full geometry. No rebuild, reinstall or additional gameplay test needed for these notes. For future reports run `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines.

## 2026-10-08 Move world drawing before Skyrim HUD (0.6.6)

**Changed:** Recorded user-confirmed transparency and the compass overlap. Planned the correction in DESIGN before code. Added a HUDMenu PostDisplay vtable hook (AE ID 215362, slot 6), drawing world geometry before chaining the original HUD display. Reused the Present renderer's device/context with isolated state restoration and fresh back-buffer references. Removed world drawing from Present; hands/HUD remain there. Added SCENE_STAGE_HOOK/STAGE/WAIT/DISABLED diagnostics. Updated both package versions to 0.6.6, retaining protocol 10. Updated README and ABI.

**Why:** The previous Present world pass ran after Skyrim UI and could cover the compass. This changes the timing of world drawing without altering projection, blending or geometry transport.

**Tested how:** Inspected current Present path, local SkyCraft render hooks and pinned CommonLib HUDMenu/IMenu vtable declarations and IMenu::PostDisplay implementation. The latter invokes uiMovie->Display. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game inspection or installation writes.

**Result:** Rust and Fabric builds succeeded; dist/step5b contains Passthrough-Minecraft-0.6.6.zip and Passthrough-Skyrim-0.6.6.zip. Only the existing informational MSVC import-library warning appeared in Rust; Java compile was up-to-date. No build failures. Geometry has one call site in the pre-HUD world method and no Present fallback. The original HUD method is called even if the new stage fails.

**Still broken / not tested:** Compass visibility, HUD hook invocation/render-thread timing, whether the back buffer at that point receives the final world image, menu transitions, transparency and other regressions are **not tested in game**. A first frame can skip world drawing until Present initializes its renderer. HUD-hidden modes or other HUD hooks can suppress this stage; repeated SCENE_STAGE_WAIT records identify absence on the Present thread. Unsupported multisampling or errors disable the new stage. There is no late fallback, because that would repeat the compass bug. This is not a verified ordering guarantee for every Skyrim menu or UI mod. Previously deferred camera/world-bleed issues remain.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild needed). With Skyrim HUD visible, put a tall Minecraft wall across the compass on screen. Expect compass above the blocks; repeat with stained glass and check drops, cracks, depth, hands/HUD and menu open/close. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Expect SCENE_STAGE_HOOK installed=1, recurring SCENE_STAGE stage=before_skyrim_hud present_world_draw=0 and SCENE_DRAW; one startup wait is possible. Send all three files and the visual result, especially if geometry disappears or still covers the compass.

## 2026-10-08 Step 5b translucent placed blocks (0.6.5)

**Changed:** After the user selected more step-5b rendering, wrote the translucent-block increment in DESIGN. Added a bounded TranslucentMesh buffer with quad-centre far-to-near sorting at publication. SceneCapture now exports translucent MODEL faces with texture/vertex alpha, reserves their capacity before drops, and sends their vertex count at header offset 108. Skyrim draws opaque blocks/items, cracks, then the translucent slice with alpha blending, depth testing and no depth writes. Added SCENE_TRANSLUCENT timing/count/eye diagnostics and matching send/receive/draw counts. Bumped both adapters to 0.6.5 and protocol/namespace to 10; updated README and ABI.

**Why:** Add placed stained-glass/ice rendering while retaining the working opaque/drop/crack paths and Skyrim wall occlusion. The user chose to continue rendering before step 6 interactions. Ground setup and deferred camera/world-bleed issues remain separate.

**Tested how:** Inspected existing Minecraft material-layer/quad export and D3D blend/depth code. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game visual inspection or game-folder writes.

**Result:** Rust release and Fabric builds succeeded; packages are dist/step5b/Passthrough-Minecraft-0.6.5.zip and Passthrough-Skyrim-0.6.5.zip. Existing informational MSVC import-library and Java deprecated-API warnings remain. No build failures. Total geometry retains the 262140-vertex limit; items/cracks cannot consume reserved translucent space. Rendering order places glass over the cracks of opaque blocks behind it.

**Still broken / not tested:** Runtime HLSL behavior, glass/ice appearance, face order, occlusion, capacity behavior and regressions are **not tested in game**. Sorting is by quad centroid at roughly 10 Hz using Minecraft's eye; intersecting surfaces and camera differences may sort incorrectly. Disabled face culling can increase opacity for closed translucent models. Sorting is vertex-bounded but has no separate time budget; timing is logged. Fluids, refraction, colored lighting, animated textures and translucent 3D dropped-block models remain unsupported. Previously deferred issues are unchanged.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Give/place glass, blue/red stained glass and ice using README's exact commands. Expect Skyrim/blocks visible through translucent pixels, opaque surfaces hiding glass behind them, and Skyrim walls hiding both. Check overlapping colored blocks, cracks through glass, drops and hands/HUD. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines), then compare translucent_vertices on matching SEND/RECV/TRANSLUCENT_DRAW snapshots. Stained glass/ice should produce nonzero counts; plain glass may use the cutout pass. Remove translucent blocks to check the count clears when none remain nearby. Install both adapters for protocol 10.

## 2026-10-08 Record successful 0.6.4 drop appearance

**Changed:** Updated README and DESIGN with the user's confirmation that dropped items and blocks appear as they should. Documentation only.

**Why:** Preserve the gameplay result before selecting the next implementation increment.

**Tested how:** User gameplay report. No new log files supplied or agent-run tests.

**Result:** Item/block appearance is user-confirmed. The earlier 0.6.3 pickup and disappearance confirmation remains recorded.

**Still broken / not tested:** Numerical mixed model/icon counts, broader occlusion/resource-pack/regression cases are **not tested**. Deferred camera/world-bleed issues and remaining rendering/interaction features are unchanged.

**Next:** No rebuild or reinstall needed for these notes. Future log collection uses `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines. The design has remaining 5b rendering work and step 6 NPC interactions; direction is being clarified before code.

## 2026-10-08 Confirm icon gameplay and add 3D full-block drops (0.6.4)

**Changed:** Recorded the user's confirmation that 0.6.3 drops spin, can be picked up and disappear. Planned the next increment in DESIGN, then added BlockDropCapture for supported default-state full opaque blocks: six directional model quads, original face UVs, default block tint, quarter-block scale and existing bob/spin. Other items and unsupported geometry retain the icon fallback. Added an explicit item entity count at scene header offset 104; validated and logged mixed six-/36-vertex items on both sides. Updated versions to 0.6.4 and protocol/namespace to 9. Updated README, DESIGN and ABI.

**Why:** Improve the successful first iteration's flat block drops one step at a time, while preserving Minecraft-owned item motion and pickup. Camera/world-bleed/terrain changes remain deferred.

**Tested how:** Read local SkyCraft's full-block drop approach and cached Minecraft model/quad/tint API signatures. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No games launched or visually inspected, no game-folder writes. No fresh item logs were supplied for numerical comparison of the user-confirmed 0.6.3 behavior.

**Result:** Both Rust release and Fabric builds succeeded; dist/step5b contains Passthrough-Minecraft-0.6.4.zip and Passthrough-Skyrim-0.6.4.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. An initial API lookup used a nonexistent top-level QuadMaterialInfo name; the returned BakedQuad signature identified the nested BakedQuad.MaterialInfo class and the corrected lookup succeeded. No build failures.

**Still broken / not tested:** New 3D appearance, face orientation, tint, pickup/occlusion and regressions are **not tested in game**. Only six-face opaque full-block models are supported; additional/unculled quads and non-full/transparent blocks fall back to icons. Default block state ignores item-specific components. Item lighting/glint, arbitrary 3D item models, animated textures and interpolation between roughly 10 Hz exports remain unimplemented. Existing scan/item/time budgets can omit items; individual model errors roll back that item and log an error. The 0.6.3 user report confirms appearance/spin/pickup only, not all timing/occlusion cases.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild needed). Give dirt, oak logs and apples with `/give @s minecraft:dirt 8`, `/give @s minecraft:oak_log 8`, `/give @s minecraft:apple 8`; drop each using Q. Expect small 3D dirt/log blocks with distinct log bark/end faces and a flat apple icon. Check pickup, wall/block occlusion, cracks, textures, hands/HUD and movement/look. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft (three COLLECTED lines). Client SCENE_ITEM should report geometry=block_model vertices=36 or geometry=icon vertices=6; matching native records should agree on items and item_vertices, with model/icon counts in SCENE_ITEM_DRAW.

## 2026-10-08 Step 5b first dropped-item icons (0.6.3)

**Changed:** Added the dropped-item increment to DESIGN before code. Added DropCapture using real ItemEntity instances, interpolated positions, Minecraft bob/spin and the item model's particle sprite. Packed block/item atlases plus crack textures in memory; added identity-based sprite UV mapping and remapped block UVs for either combined layout. Appended item vertices before cracks in each completed mesh. Added header offset 100 item count, validation and item send/receive/draw diagnostics. Bumped adapters to 0.6.3 and protocol/namespace to 8. Updated README and ABI with limits and manual procedure.

**Why:** Continue step 5b with visible drops and pickup, using SkyCraft's simple icon fallback as the first bounded increment. Minecraft retains all item gameplay. Terrain/world setup, camera orbit and world bleed remain separate.

**Tested how:** Read the local SkyCraft item fallback and runtime atlas approach, inspected Minecraft 26.3 public ItemEntity, item-model, sprite and entity-iteration signatures, and built both adapters with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No games launched or visually inspected, and no game-folder writes.

**Result:** Rust release and Fabric builds succeeded. Packages are dist/step5b/Passthrough-Minecraft-0.6.3.zip and Passthrough-Skyrim-0.6.3.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. No build failures. Geometry stays in the existing solid depth/alpha-cutout pass; cracks remain last. New snapshots replace dynamic geometry rather than appending it again on retries.

**Still broken / not tested:** Dropping, falling, pickup, merging, wall occlusion, combined atlas reload and existing-feature regressions are **not tested in game**. These are flat particle-sprite icons, not full 3D models; no item lighting/tint/glint or animated textures. One icon represents a whole stack. Updates remain roughly 10 Hz and can look stepped. The export checks a 3 ms deadline between entities, scans at most 4096 entities and attempts at most 64 nearby items; limits may omit items, and one model-resolution call can exceed the budget. Unsupported sprite atlases are skipped, individual runtime errors are logged, and an oversized combined atlas disables scene export. Neither correctness nor performance under resource packs/crowded scenes is claimed. Both adapters must be installed together for protocol 8.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines, no rebuild needed). In offline first-person practice worlds, use `/give @s minecraft:apple 8` and `/give @s minecraft:dirt 8`, then drop each with Q. Expect bobbing/spinning apple and dirt-face icons in Skyrim, hidden by walls/blocks and removed on Minecraft pickup. Mine dirt in survival to create a natural drop; check cracks, ordinary textures, hands/HUD, movement and look. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three COLLECTED lines. Compare SCENE_SEND/RECV/ITEM_DRAW item_vertices (six per icon; zero when none remain) and client SCENE_ITEM positions/IDs/counts.

## 2026-10-08 Confirm 0.6.2 crack logs and explain ground rendering

**Changed:** Updated README and DESIGN with the user's successful gameplay report, numerical crack results and expected Minecraft ground inclusion. Documentation only.

**Why:** User supplied logs/scene-report-20261008-084544 and asked whether Minecraft ground should render.

**Tested how:** Parsed all three report files. Matched every received record to its sent kind/sender/world/snapshot/atlas and compared vertex count, dimensions, byte count, checksum, origins and crack count. Matched Skyrim crack-draw counts and solid-plus-crack totals to received snapshots. Inspected SceneCapture's region/block selection.

**Result:** All 400 received records match (399 meshes and one atlas); all 399 crack-draw counts match. There are 45 snapshots/draws with 36 crack vertices, 354 with zero, all stages 0–9, and four transitions from active cracks to zero. All 87 depth probes are usable; only one startup wait, no reported SCENE_ERROR/DISABLED/BUILD_DROPPED, and no skipped crack shapes in this run. User reports gameplay works as expected. Ground rendering follows current code: all supported nearby blocks are included, with no distinction between generated and player-placed terrain.

**Still broken / not tested:** Logs alone do not distinguish cancellation from completion for each clear, or prove pixel-level occlusion. Exhaustive graphics/resource-pack cases are **not tested**. A mostly empty/void Minecraft world is not set up by this mod; generated floor and its collision remain. Previously deferred camera/world-bleed issues and unimplemented dropped items remain.

**Next:** No rebuild, reinstall or additional gameplay test needed for this documentation update. For future reproductions, run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three COLLECTED lines. Ground removal would require a separate world-setup change, not just hiding its graphics.

## 2026-10-08 Step 5b breaking cracks on full opaque blocks (0.6.2)

**Changed:** Wrote the next increment in DESIGN before implementation. Added CrackCapture using Minecraft's existing destructionProgress map and resource manager. Appended ten runtime crack textures to the in-memory atlas, remapped block V coordinates, and appended up to 32 full-cube crack overlays to complete mesh snapshots. Added a trailing crack-vertex count at header offset 96, validation, transparent darkening after the solid draw, and depth testing without crack depth writes. Bumped both adapters to 0.6.2 and the shared namespace/protocol to 7. Added client stage/count and native send/receive/draw count logs. Updated README and ABI with procedure and limits.

**Why:** Continue step 5b one piece at a time, addressing the previously missing breaking animation. Camera orbit and world bleed remain deferred; dropped items are a separate increment.

**Tested how:** Inspected the local SkyCraft crack-export approach and Minecraft 26.3 public method signatures; wrote original implementation using those APIs. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No games launched or visually inspected, and no game-folder writes.

**Result:** Rust release and Fabric builds succeeded. Packages are dist/step5b/Passthrough-Minecraft-0.6.2.zip and Passthrough-Skyrim-0.6.2.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. An initial rg wildcard-path read produced Windows path syntax error; explicit paths and directory globs were used afterward. Implementation refreshes cracks on publication retries, sends zero crack vertices after damage clears, and retains the existing expiry/world checks.

**Still broken / not tested:** Runtime HLSL compilation, crack progression/cancellation/completion, depth occlusion, resource reload and regressions are **not tested**. Only full opaque MODEL cubes are supported; other shapes and capacity overflow are counted/skipped. Textures are resampled to 16x16, colored crack packs become grayscale, and a block atlas already 4096 pixels high cannot fit the strip and disables scene export. Mesh publication remains roughly 10 Hz, so fast damage stages can be skipped. Missing/invalid crack resources fail scene export with a logged error. Dropped items, camera orbit, world bleed and placement reach remain as previously recorded. Both game adapters must be installed together for protocol 7.

**Next:** Close both games and run `.\tools\install-build.ps1`; expect three INSTALLED lines (no rebuild needed). Start offline first-person worlds, run `/setblock ~2 ~1 ~ minecraft:stone` and `/gamemode survival` in Minecraft, and mine the stone bare-handed. Expect cracks to advance in Skyrim; stopping early should clear cracks, completing should remove the block, and a Skyrim wall should hide both block and cracks. Check ordinary block textures, HUD and movement/look. Run `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines. Matching snapshots should have crack_vertices=36 for one damaged block and 0 after clearing, with SCENE_CRACK stages in the client and SCENE_CRACK_DRAW in Skyrim. In-game outcome is not yet known.

## 2026-10-08 Compare 0.6.1 logs and correct the cutoff assessment

**Changed:** Updated README and DESIGN with the numerical comparison and the user's corrected eight-block observation. Recorded close placement reach, perceived scale, camera orbit and world bleed; the last two are deferred as requested. No code, builds, deployment or game configuration changes.

**Why:** The user supplied logs/scene-report-20261008-081526 and clarified that the original three-block estimate was mistaken.

**Tested how:** Parsed all three report files and compared every SCENE_RECV with its SCENE_SEND by kind/sender/world/snapshot/atlas, checking counts, dimensions, payload sizes, checksum and origins. Summarized bounds and projection/depth diagnostics. Read position/mesh mapping code and ANCHOR/RELEASED events in logs/step2/skyrim-39800-163086750.log. No games were visually inspected.

**Result:** All 1,444 received records match (1,443 meshes and one atlas), with 1,443 draws, 317 usable depth probes and only one startup depth wait. No SCENE_ERROR, SCENE_DISABLED or SCENE_BUILD_DROPPED appeared in the report. All 1,443 bounds describe a 16x16x16 cube. Maximum depth-formula disagreement at 500 units is 0.00000114; maximum projection-W error is 0.00046 units. Both movement and mesh rendering use 70 Skyrim units/block. An initial whole-run scale comparison showed a 518.2307-unit offset change; reading the full log revealed release/re-anchor events, so a single unchanged anchor cannot be assumed. Between offset changes, consecutive mesh-origin deltas agree with 70 units/block within 0.0004 Skyrim units. This does not verify every movement sample or establish the cause of re-anchoring. Two rg wildcard-path reads failed with Windows path syntax; corrected directory/glob and direct file reads supplied the needed data.

**Still broken / not tested:** Close placement reach and perceived scale are unresolved. Camera orbit and world bleed are user-reported and deferred; their causes are **not tested**. Correct near/far arithmetic does not prove every pixel has correct occlusion. Breaking cracks and dropped-item rendering remain unimplemented. The earlier three-block-cutoff diagnosis is superseded by the user's corrected observation, not fixed through code.

**Next:** No rebuild or reinstall needed. For any later reproduction, run `.\tools\collect-scene-logs.ps1` immediately afterward and before restarting Minecraft; expect three COLLECTED lines. Keep the full bridge logs too, because the scene-only report omits alignment lifecycle events.

## 2026-10-08 Investigate short range and add 0.6.1 diagnostics

**Changed:** Wrote the investigation plan in DESIGN before code. Added SCENE_BOUNDS in Minecraft's client log (export limits, player position and aimed block), SCENE_PROJECTION in Skyrim (depth at known distances versus the shader's near/far formula), and an additional centre-screen depth sample with its sampling timestamp. Preserved the original 16 samples used to infer the depth convention. Added tools/collect-scene-logs.ps1 to stream rendering-only lines into an ignored project report folder. Updated README and package versions to 0.6.1; protocol remains 6.

**Why:** Investigate the user's roughly three-block disappearance without guessing at an occlusion fix or removing wall occlusion.

**Tested how:** Read the latest recorded bridge logs (minecraft-29756-152557406.log and skyrim-35748-152539546.log), local SkyCraft reference, CommonLib camera layouts, and Minecraft tessellation signatures. Built Rust release and Fabric with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests or game inspection. Gradle's test task has no test sources.

**Result:** The recorded run reaches mesh snapshot 2204, continues drawing nonempty meshes, and has only one SCENE_WAIT at startup, with no SCENE_DISABLED/ERROR entries found in Skyrim. Logged projection W is approximately 500 for a point 500 units ahead; near/far are 15/353840. These observations do not establish why pixels disappear. Both adapters compile. A diagnostic sample initially shadowed the new centre value; a compiler warning exposed this and the duplicate was removed before final packaging. The usual linker import-library message and Java deprecated-API warning remain.

**Still broken / not tested:** The cutoff is **unresolved**. The added runtime shader/diagnostics, report collector, installation and gameplay are **not tested**. Breaking cracks and dropped items remain unimplemented. The collector selects the newest bridge log for each game; collect immediately after a shared run, before restarting Minecraft. No game installation was changed.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines). Start offline first-person worlds, place a stone block with `/setblock ~2 ~1 ~ minecraft:stone`, keep looking at it while backing away until it disappears, then approach again. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three COLLECTED lines. Supply those three report files so exported coverage, aimed block, camera projection and centre depth can be compared. This diagnostic build may still show the cutoff.

## 2026-10-08 Record 0.6.0 gameplay results and short render range

**Changed:** Updated README and DESIGN with the user's 0.6.0 results. No code changes.

**Why:** Record passed behavior and the unexpected rendering cutoff separately from missing features.

**Tested how:** User gameplay report: spawned and broke a block, moved around it and a Skyrim wall, checked wall occlusion and older features. No agent-run tests or visual inspection.

**Result:** Block appearance/removal, stable world placement, Skyrim-wall occlusion, and prior features passed the user's checks. The user reports blocks disappear at roughly three blocks away. Breaking animation and block drops are not visible.

**Still broken / not tested:** Short render range is unresolved; no cause is established. Intended export-range coverage and numerical log comparison are **not tested**. Breaking crack overlays and dropped-item rendering are unimplemented; absence of a visible drop does not establish whether Minecraft created an item entity.

**Next:** Inspect this run's SCENE_CAPTURE/SEND/RECV/DEPTH/DRAW/WAIT logs to distinguish missing geometry from depth rejection or stale snapshots. The existing README step-5b Select-String commands should show corresponding snapshot identifiers, nonzero vertex counts and depth state while reproducing the cutoff. No rebuild is needed for this documentation update.

## 2026-10-08 Fix installer blocking VS Code Java processes

**Changed:** Replaced the blanket java/javaw process block in tools/install-build.ps1 with Minecraft launch-class/instance-path detection and the existing Skyrim check. Added an exclusive read/write-open preflight for all existing destination files before backup/copy. It does not write file contents during that check. Process-query failures warn and fall back to file checks. Command lines are inspected in memory and never printed. Updated README and DESIGN.

**Why:** The user reported installation refused with only VS Code's Java extension processes running. Blocking every Java process was incorrect.

**Tested how:** Read the supplied error/screenshot and installer source. No script execution, installation, builds or automated tests were run for this fix.

**Result:** Corrected the source so unrelated Java processes do not trigger the game-process guard. Existing backup, checksum and restoration behavior remains.

**Still broken / not tested:** Updated process detection, file-access checks and deployment are **not tested**. Process details may be unavailable or nonstandard launchers may be unidentified; both games must remain closed, and file-access checks cannot prevent a game starting afterward.

**Next:** Leave VS Code open, close both games, and run `.\tools\install-build.ps1`. Expect three INSTALLED lines and "Install complete: 3 files copied and verified." No rebuild is needed.

## 2026-10-08 Step 5b first world-mesh build (0.6.0)

**Changed:** Updated DESIGN before code after inspecting SkyCraft's actual mesh-rendering path. Added SceneCapture, atlas/sprite accessors, native scene mailboxes/FFI and scene_draw.rs. Minecraft tessellates nearby opaque/cutout block models and sends runtime atlas pixels; Skyrim maps triangles through the movement Anchor, projects through its rendered camera, probes its scene depth and clips hidden pixels. A private depth buffer resolves block-on-block overlap; the existing hands/HUD pass follows. Added SCENE_CAPTURE/SEND/RECV/UPLOAD/DEPTH/DRAW/WAIT/ERROR diagnostics. Bumped both adapters to 0.6.0 / protocol 6. Added build-step5b.ps1 with matching version checks; earlier step builders delegate to it and the installer defaults to dist/step5b. Updated README and ABI with exact commands, numerical checks, manual wall-occlusion procedure and limitations.

**Why:** The user confirmed the 5a arms/items/interface appear inside Skyrim and requested 5b. SkyCraft's current implementation sends world meshes and textures to Skyrim so its own camera/depth can position and hide them correctly; this build follows that approach rather than superimposing a flat world image.

**Tested how:** Compiled Rust release and Fabric, then packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Inspected cached Minecraft 26.3 tessellation signatures and private atlas fields, pinned CommonLib renderer/camera layouts, SkyCraft's fallback depth sampling, and Microsoft's D3D11 depth/Map documentation. Read the source changes and ran git diff --check. No automated tests were added or run; Gradle's test task was NO-SOURCE. No games were started or visually inspected.

**Result:** Rust and Fabric builds succeeded; dist/step5b contains Passthrough-Minecraft-0.6.0.zip and Passthrough-Skyrim-0.6.0.zip. Java reported use of a deprecated Minecraft API; Rust emitted the informational MSVC import-library linker warning. Initial WMI process classification was denied by the sandbox; the approved retry returned classification flags but did not establish that Minecraft was closed. The installer was then invoked and refused before any file writes because it detected two Java processes. No game installation changed. Two patch-context mismatches were corrected before those patches applied.

**Still broken / not tested:** New HLSL runtime compilation, mesh/atlas transfer, rendered camera alignment, depth inference/occlusion, resource reloads, pause/reconnection/resizing, installation and performance are **not tested**. This first 5b build handles a 16-cubed nearby region, up to 262,140 vertices, atlas up to 4096x4096, first sprite animation frames only, and simplified Minecraft brightness. Translucent faces are counted/skipped; fluids, dropped items, block-entity models, mobs and the third-person player are not exported. Skyrim lighting/shadow/post-processing integration is not implemented. Uniform/unavailable depth suppresses drawing instead of claiming valid occlusion; depth convention is inferred from samples. The installer remains blocked while Java is running.

**Next:** Close the games/Java applications, then run `.\tools\install-build.ps1` (no rebuild needed). Expect three INSTALLED lines. Start offline first-person practice worlds and use `/setblock ~2 ~1 ~ minecraft:stone` in Minecraft. Expect the nearby block to stay fixed in Skyrim and disappear behind a Skyrim wall; hands/HUD should remain visible. Run README's step-5b log commands and compare SCENE_SEND/RECV identities/counts/checksums, SCENE_DEPTH usable=true and SCENE_DRAW. Send both bridge logs plus Minecraft latest.log if drawing waits or fails. Further world-object classes remain later rendering work.

## 2026-10-07 Confirm step 5a overlay appears in Skyrim

**Changed:** Updated README, DESIGN and ABI status with the user's explicit confirmation that first-person arms/punching, held items, HUD and inventory also appear inside Skyrim's window. No code changes.

**Why:** Resolve the destination-window ambiguity in the previous gameplay report.

**Tested how:** User gameplay confirmation. No agent-run tests or visual inspection.

**Result:** Basic hand/item/interface composition over Skyrim is confirmed by the user.

**Still broken / not tested:** Numerical image-log comparison, resize/reconnection/error paths and broad graphics compatibility are **not tested**. Invisible Minecraft terrain still collides; third-person and world/depth rendering remain unsupported.

**Next:** No rebuild needed. To compare transport numerically, run the existing README step-5a log commands; expect matching OVERLAY_SEND/RECV frame identifiers and checksums plus OVERLAY_DRAW. Step 5b remains pending.

## 2026-10-07 Record first step 5a gameplay observations

**Changed:** Updated README and DESIGN with the user's report of first-person arms/punching, held items, hotbar, health, hunger and inventory rendering, a black Minecraft world, remaining Minecraft terrain collisions, and absent third-person rendering. No code changes.

**Why:** Keep observed behavior and current rendering limits explicit. Suppressing level drawing does not remove Minecraft's normal collisions. Superflat reduces obstacles but still supplies a colliding floor.

**Tested how:** User gameplay report only. Asked whether the visible elements appeared over Skyrim or only in Minecraft. No games, scripts or automated tests were run by the agent.

**Result:** Recorded the observations without claiming verified Skyrim composition or image transport. Third-person models/world drawing remain outside 5a.

**Still broken / not tested:** Destination-window confirmation and image-log comparison pending. Superflat workaround is **not tested**. World/depth rendering remains unimplemented; invisible Minecraft terrain can block movement.

**Next:** Confirm the destination window. For numerical evidence, run README's step-5a Select-String commands after closing both games; expect matching OVERLAY_SEND/RECV frame identifiers and checksums plus OVERLAY_DRAW. No rebuild or installation is needed for these documentation changes.

## 2026-10-07 Install matching files after package builds

**Changed:** Added tools/install-build.ps1 for the two game folders named in AGENTS.md. It checks package completeness and matching DLL hashes, requires Skyrim/Java to be closed, backs up previous mod files in ignored dist/install-backups, copies the Minecraft JAR plus both DLLs, verifies copied hashes, removes only older passthrough JARs, and attempts restoration on failure. Connected it to successful step-5 package builds by default; added -PackageOnly through older build wrappers and -WhatIf on the installer. Updated DESIGN before implementation and README with commands and expected output.

**Why:** The user requested a PowerShell script to place each new build in the mod folders and avoid repeated manual copies or mixed JAR/DLL versions.

**Tested how:** Not tested. No deployment, build or automated test was run for this script change. Read the existing scripts and installation paths while implementing.

**Result:** Script and build integration are written. No game folder was modified in this turn.

**Still broken / not tested:** Installation, checksum checks and failure restoration are **not tested**. Direct Skyrim installation can be overridden by a mod manager; use -PackageOnly and the manager in that case. The closed-game guard conservatively blocks all Java processes.

**Next:** Close both games and run `.\tools\install-build.ps1` from the project folder to install the existing build. Expect three INSTALLED lines and "Install complete: 3 files copied and verified." Future `.\tools\build-step5.ps1 -Offline` runs install after packaging; add -PackageOnly to skip that step.

## 2026-10-07 Step 5a: Minecraft hands and HUD overlay

**Changed:** Recorded the user's successful camera test. Wrote the 5a/5b split in DESIGN before implementation. Added Minecraft transparent level suppression and three asynchronous image-readback buffers, new native overlay exports and a bounded RGBA mailbox, protocol 5 readiness/generation checks, and a Skyrim D3D11 Present hook that uploads and blends hands/HUD while restoring graphics state. Added OVERLAY capture/send/receive/draw/error logs with pixel checksums, alpha counts and CPU timings. Versioned both adapters 0.5.0; added tools/build-step5.ps1 and made the step-4 builder delegate. Updated README and ABI with installation, limits and exact manual/log commands. Game pixel data stays in runtime memory. No game installation writes or Git commits were made for this change.

**Why:** The user confirmed Minecraft now turns Skyrim's camera and asked to proceed with step 5. Its first build step is hands/interface; world objects and depth follow after this part is validated.

**Tested how:** Built Rust release and Fabric against the cached Minecraft 26.3 APIs, then ran `.\tools\build-step5.ps1 -Offline` to package both. Read SkyCraft's capture/composition implementation, CommonLib's published renderer pointer/offset, cached Minecraft method signatures, and Microsoft's D3D11 context-state documentation. No automated tests were added or run; Gradle's test task reported NO-SOURCE. No games were launched or visually inspected.

**Result:** Final Rust release build and Fabric build succeeded; created dist/step5/Passthrough-Skyrim-0.5.0.zip and Passthrough-Minecraft-0.5.0.zip. Rust emitted only the existing informational MSVC import-library linker warning. Initial dependency fetch failed under sandbox network restrictions; approved fetch succeeded. First Rust build caught an incorrect string-replacement argument; that was fixed. The following build caught a separate u32/typed-flag mismatch in device creation flags; that was fixed and subsequent builds passed. API inspection also caught the texture-format enum name: changed the guard to the actual RGBA8_UNORM enum. The initial TextureFormat class lookup failed because this API calls it GpuFormat. The local MODLOG-template.md is absent; used its tracked template format without restoring the user's deleted file.

**Still broken / not tested:** Live shader compilation, Minecraft mixin application, image capture/transport/composition, orientation/transparency, resizing, reconnection, pause handling, performance and graphics-mod compatibility are **not tested**. Builds do not prove visible rendering. Step 5b world/depth drawing is not implemented. Paused menus, destination-aware crosshair inversion, exact color matching and GPU device replacement are not handled. Input remains in Minecraft's window. No pixel capture files were produced.

**Next:** Install both 0.5.0 packages, replacing Minecraft's JAR and DLL together. Run both games offline at 1280x720 with Minecraft focused, first person and HUD visible. Follow README's "Check step 5a hands and HUD" commands. Expect matching OVERLAY_SEND/RECV identity, dimensions, checksum and alpha counts, plus Skyrim OVERLAY_DRAW submissions; manually confirm the hand/hotbar appear over Skyrim. Validate this before implementing step 5b.

## 2026-10-07 Prepare initial GitHub publication

**Changed:** Configured origin as https://github.com/StevenLum/SkyCraftPassthrough.git for the user's requested commit and push. Prepared the current 0.4.0 project source, documentation and build scripts as the initial repository snapshot on the existing master branch. Removed extra trailing blank lines in tools/build-step4.ps1 flagged by the staged whitespace check.

**Why:** The user asked to push the accumulated project changes to their GitHub repository.

**Tested how:** Read AGENTS and the whitelist, enumerated all 42 publishable files, checked for common credential patterns without printing potential matches, and fetched the remote branch list. No builds or gameplay tests were rerun for publication.

**Result:** No existing local commits or remote branches; no credential-pattern matches. The whitelist excludes game data, reference checkouts, downloaded tools, logs and compiled packages. The sandbox initially blocked GitHub access; the approved fetch succeeded. Git author identity was resolved from the user's configured name and GitHub noreply address after the temporary safe-directory configuration hid those settings. Created initial commit b8119b46ac93ff67910be9d23bc508db43821785. The first push was rejected for authentication; retrying with the normal user Git configuration included succeeded. GitHub's master ref was read back and matched that commit exactly; local master now tracks origin/master.

**Still broken / not tested:** Camera behavior after the corrected Minecraft DLL installation remains **not tested**. Source publication does not verify gameplay, and compiled packages are not included in Git.

**Next:** Publish this outcome record, then continue the pending in-game camera verification. To check synchronization, compare `git rev-parse HEAD` with `git ls-remote origin refs/heads/master`; expect the same commit ID.

## 2026-10-07 Replace outdated Minecraft bridge DLL with approval

**Changed:** With explicit tool approval under AGENTS.md rule 4, replaced `C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\mods\passthrough_link.dll` with the prepared 0.4.0 DLL. Updated README status. No other game files changed.

**Why:** The installed 0.3.0 DLL could not supply the function required by the 0.4.0 JAR.

**Tested how:** Copied the packaged DLL and read the installed file's SHA256.

**Result:** Installed hash now matches the 0.4.0 package exactly: 75D5F9066EEDE11C3F203AACF25C5A70FC5D40E06A0497903E7E64F2A5C3F1DF. Copy command completed successfully.

**Still broken / not tested:** Minecraft restart and live camera following after replacement are **not tested**. No games were launched by the agent.

**Next:** Restart the Prism instance, enter the offline world and expect a new Minecraft bridge log containing NATIVE_READY protocol=4 and SEND lines. Run the README position comparison with -RequireLook after testing mouse look with both games running.

## 2026-10-07 Diagnose missing Minecraft logs after 0.4.0 upgrade

**Changed:** Added upgrade troubleshooting to README. No implementation or game installation changes.

**Why:** The user reported 0.4.0 did not work and Minecraft created no bridge log.

**Tested how:** Read Minecraft's latest.log, the two latest Skyrim bridge logs, and inspected the installed Minecraft DLL's SHA256 and exports. Installation reads initially failed in the sandbox and succeeded with approval. No automated tests were run.
**Result:** Minecraft loaded passthrough_practice 0.4.0, then threw NoSuchElementException in NativeLink.java:26 while resolving pt_mc_frame4. The installed DLL hash is 6563BD8D6FA3AF8EF22B79870873B949EAF94751ED07576CB0A3A5EC377FEA9E, exactly matching the 0.3.0 package; it exports pt_mc_frame but not pt_mc_frame4. The correct 0.4.0 packaged DLL hash is 75D5F9066EEDE11C3F203AACF25C5A70FC5D40E06A0497903E7E64F2A5C3F1DF. Both Skyrim runs loaded successfully, installed the hook and reached NATIVE_READY protocol=4, waiting for Minecraft. This establishes a mixed Minecraft JAR/DLL installation as the startup failure.


**Still broken / not tested:** Corrected Minecraft installation and live mouse-look behavior are **not tested**. No code fix is required for this diagnosed mismatch.

**Next:** Close Minecraft and replace its mods/passthrough_link.dll with dist/step4/minecraft/mods/passthrough_link.dll, with game-folder write authorization required by AGENTS.md rule 4. Restart and expect Passthrough Fabric plugin loaded in latest.log, a new project Minecraft bridge log with NATIVE_READY protocol=4, and SEND lines. Then repeat the README look comparison with -RequireLook.

## 2026-10-07 Step 4: Minecraft mouse-look transfer

**Changed:** Recorded the user's successful jumping/landing test. Updated DESIGN before implementation. Added yaw/pitch to rendered position samples, protocol 4 and `pt_mc_frame4`, and `look.rs` to convert Minecraft angles and drive Skyrim's player angles for its normal first-person camera. Added ordinary third-person switching/restoration and special-camera guards. Extended SEND/RECV logs with source angles and added LOOK_RECV target/read-back and camera-state quaternion diagnostics. Added `-RequireLook` to the position comparison script, using streaming file reads for large logs. Updated README/ABI, versioned packages 0.4.0, and added `tools/build-step4.ps1`; older build commands delegate to the current package builder.

**Why:** The user confirmed Minecraft's mouse turns its own camera but Skyrim's view does not follow, and requested Minecraft-to-Skyrim mouse translation. Send the resulting view angles so Minecraft retains its normal mouse sensitivity and controls.

**Tested how:** Inspected SkyCraft's angle conversion/player-angle writes and the public CommonLib camera interfaces; checked Minecraft 26.3 view-angle method signatures. Compiled and packaged using `.\tools\build-step4.ps1 -Offline`. No automated tests were added or run. Existing fixture code was kept source-compatible with the extended sample/API.

**Result:** Rust and Fabric compilation and packaging succeeded. Builds contain finite/range checks, same-frame yaw/pitch, normalized heading and matching log frame identities. These are compiled changes, not a confirmed live camera result. The user reports jumping and landing worked in the previous build, adding to previously confirmed wall/NPC blocking. The linker still prints its informational import-library creation warning.

**Still broken / not tested:** New camera response, axis direction, camera-state transitions/restoration, timing and `-RequireLook` are **not tested**. Camera quaternion logs precede the next native camera update; actor read-back is not proof of the rendered camera. Third-person Minecraft mirroring, exact eye position, FOV/bob matching and Skyrim-window input forwarding remain unimplemented. No game installation files changed.

**Next:** Install both 0.4.0 packages together, keep Minecraft focused in first-person and Skyrim running in the background, then look left/right, up/down and through a full turn. Expect Skyrim's first-person view to follow. Stop both games, select the latest `$mcLog`/`$skyLog` as in README, and run `.\tools\compare-position-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName -RequireLook`; expect position PASS and nonzero look frames matching within 0.00001 radians. Supply logs if the camera does not follow.

## 2026-10-07 Position and world comparisons; manual wall and NPC blocking

**Changed:** Updated README and DESIGN with the new comparisons, the user's manual collision result, and the clarified missing camera synchronization. No code or game files changed.

**Why:** Verify the user's supplied live logs and distinguish working Minecraft mouse look from unimplemented Skyrim camera alignment.

**Tested how:** Ran `.\tools\compare-position-logs.ps1 -MinecraftLog 'D:\Documents\ChatGPT\Game modding\logs\step2\minecraft-32128-107636093.log' -SkyrimLog 'D:\Documents\ChatGPT\Game modding\logs\step2\skyrim-15308-107625921.log'` and `.\tools\compare-world-logs.ps1` with the same arguments. Both scripts completed with exit code 0. The user tested movement and reported stopping at a Skyrim wall and NPC; the agent did not visually inspect either game.

**Result:** Position PASS: 5986 received frames matched all Minecraft coordinates and partial ticks exactly; 11437 frames sent; maximum Skyrim target/read-back difference 0 units. World PASS: 543 complete received snapshots, 1354585 collision-box records and 139 NPC records matched exactly (record counts across snapshots, not unique objects). The world comparison took several minutes for the approximately 335 MB logs. User clarified the camera turns inside Minecraft but not Skyrim: Minecraft mouse input works; view synchronization is absent.

**Still broken / not tested:** Look-direction transfer and Skyrim camera alignment are unimplemented, as is Skyrim-window input forwarding. Jumping, landing, slopes and general collision coverage remain **not tested**. Transport equality does not establish complete geometric accuracy.

**Next:** The comparison commands above should continue to print PASS on these logs. Next proposed implementation: transmit Minecraft look direction and align Skyrim's camera, then handle Skyrim-window input forwarding separately. No camera or input code was added in this comparison task.

## 2026-10-07 Live step 3 transport and control clarification

**Changed:** Updated README and DESIGN with the user's live result and clarified that movement currently uses Minecraft's controls. No code or game installation changes.

**Why:** The user reported matching world logs but could not move using Skyrim's controls. Minecraft owns position; Skyrim-window input forwarding is not implemented.

**Tested how:** Used the user's reported comparison result for the named Minecraft/Skyrim logs. Read the start of `skyrim-24140-106825984.log` and Minecraft's normal latest.log collision-query entries. The initial Minecraft read was denied by the sandbox; an approved elevated read succeeded. No automated tests were run.

**Result:** User comparison: 117 complete received snapshots, 149415 boxes, 117 NPC records matched exactly. Skyrim recorded PLUGIN_LOADED, HOOK_INSTALLED and Connected. Minecraft logged 28 client queries (12 with nonzero matches) and 28 server queries (1 with nonzero matches). The missing Address Library problem is resolved for this run. These counters confirm transport and query injection, not correct physical response; queries can belong to entities other than the player.

**Still broken / not tested:** Skyrim keyboard/mouse forwarding is unimplemented. Accurate movement, wall blocking, landing and NPC collision are **not tested**. Skyrim input being overwritten while Minecraft supplies positions is expected under the current design.

**Next:** Focus Minecraft, leave its pause menu closed, walk and jump near the exported surfaces, with Skyrim configured to update in the background. Use the README position/world comparisons after stopping both games; expect matching coordinates and snapshots, then verify movement behavior. Input forwarding can be a separate requested step.

## 2026-10-07 Step 3: sampled Skyrim geometry and NPC positions

**Changed:** Updated `docs/DESIGN.md` before implementing the requested step. Added `world.rs` (bounded snapshot format) and `world_export.rs` (Havok ray sampling and nearby loaded actors), extended the Rust/Java native bridge, and added `SkyCollision.java` and the common `BlockCollisionsMixin.java`. Added protocol-3 shared memory, immutable collision snapshots shared by the Minecraft client and integrated server, world/origin/age guards, per-snapshot geometry/NPC logs, and collision-query logging. Added `tools/build-step3.ps1` and `tools/compare-world-logs.ps1`. Versioned both packages 0.3.0; the previous build command now delegates to the current one to avoid mixing old Java and new DLLs. Updated README and ABI documentation. No game installation writes, commits, or staging.

**Why:** Keep Minecraft authoritative for movement while its unchanged physics queries include nearby Skyrim surfaces and NPC body boxes; log the sent and received data for comparison.

**Tested how:** Inspected SkyCraft's documented ray-sampling approach, public CommonLib headers and implementation at the existing pinned checkout, and Minecraft 26.3 collision iterator signatures/bytecode in the ignored build cache. Compiled the Rust release DLL and Fabric JAR and packaged with `.\tools\build-step3.ps1 -Offline`. No new tests were added or run; the previous 14 tests and Java smoke check were not rerun for step 3.

**Result:** Native and Java compilation and packaging succeeded. The exporter samples 1,536 grid rays per completed scan (plus continuation rays, up to eight hits each), with a target budget of 2 ms / 384 casts per update checked between rays. Complete snapshots carry up to 8,192 surface boxes and 128 NPC records. SEND/RECV logs include matching world, epoch, snapshot, indices/IDs and coordinates; Minecraft's normal log reports client/server COLLISION_QUERY use once per second per side. These are implementation details, not observed gameplay results. The linker still prints its informational import-library creation warning.

**Still broken / not tested:** Step 3 runtime behavior, Havok/native interfaces, mixin application, collision accuracy, performance, and the new log-comparison script are **not tested**. Successful Skyrim loading after installing its missing Address Library has not been reported. Ray sampling is approximate (half-block spacing, four-block radius); it can miss thin objects and lags moving surfaces. NPCs are restricted to loaded high-process actors in the player's cell within eight blocks per axis, with fixed 0.6-by-1.8-block approximate collision boxes. No exact shape decoding, NPC interactions/combat, camera/input work, or emergency floor on missing geometry. Incomplete/slow scans are discarded, stale snapshots expire, and overflow reports an error instead of truncating data.

**Next:** Install both 0.3.0 packages together using the README procedure and confirm Skyrim's required Address Library is installed. Run both games offline, approach a floor, wall and NPC, stop both games, and run `.\tools\compare-world-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName` after selecting the latest logs as documented. Expect complete matching snapshots with nonzero boxes/NPCs and COLLISION_QUERY for both client and server. Recheck position logs as well. Game-folder installation by the agent still requires authorization under AGENTS.md rule 4.

## 2026-10-07 Identify missing Skyrim Address Library


**Changed:** Added the full Address Library path and required runtime to the file-read error in `passthrough/src/skse.rs`. Updated README with the actual load failure and dependency download link. Rebuilt the step 2 packages.

**Why:** The user's Skyrim log reported only Windows error 2, without identifying the missing file.

**Tested how:** Read `logs/step2/skyrim-11832-84680734.log`, inspected the loader source, and listed the named Skyrim install's `Data/SKSE/Plugins` directory. Ran `.\tools\build-step2.ps1 -Offline` to rebuild. No new automated tests were run.

**Result:** The log contains LOAD_BEGIN followed by LOAD_FAILED. The plugin directory contains only `passthrough_link.dll`; the required `versionlib-1-7-104-0.bin` is absent. The release build and packaging succeeded. The user reports Minecraft position logging.

**Still broken / not tested:** Skyrim remains blocked by the missing dependency. The improved message and successful loading/movement in the real game are **not tested**. No game files were changed by the agent.

**Next:** Install the Address Library for runtime 1.7.104.0, then run `& 'E:\SteamLibrary\steamapps\common\Skyrim Special Edition\skse64_loader.exe'` (or the mod manager's SKSE entry). Expect `plugin loaded runtime=1.7.104 event=PLUGIN_LOADED` in the new Skyrim log, then compare position logs after loading both single-player worlds.

## 2026-10-07 Step 2: rendered position sender and Skyrim puppet adapter

**Changed:** Updated `docs/DESIGN.md` for the user's requested step 2. Added the Fabric render callback and Java/native bridge in `minecraft/`; extended the Rust connection to protocol 2 with a latest-frame position record; added a Rust SKSE loader and player-update adapter in `passthrough/src/skse.rs`. Added per-frame SEND/RECV logs, origin alignment, coordinate conversion, actor/controller movement, temporary gravity suspension and restoration, and stale/inactive-world guards. Added build/package, Java/native smoke-test, and log-comparison scripts, plus `docs/ABI.md`. Updated README and the source whitelist. Built the two packages under ignored `dist/step2`.

**Why:** Minecraft must determine position using the partial-tick rendered value, and matching frame identities must let the user compare what was sent, received, and applied.

**Tested how:** `tools/test-link.ps1`: 8 unit tests, 5 connection process tests, and 1 position lifecycle test. `tools/test-position-native.ps1`: Java 25 loaded the real DLL, sent synthetic render samples, and compared them with a separate Windows receiver. Built the Fabric JAR against 26.3/Fabric 0.19.5, inspected the render-hook target signatures and Java 25 mixin compatibility, and inspected the DLL exports. Built both packages with `tools/build-step2.ps1 -Offline`. Fed the log checker a deliberately altered coordinate to verify rejection.

**Result:** All 14 Rust checks passed. The Java/native test sent 180 synthetic frames; 100 consumed frames matched coordinates and partial ticks exactly. Example fixture frame 2: partial=0.3333333333333333, mc_x=12.033333333333333, mc_y=64.04166666666667, mc_z=-5.066666666666666 on both sides. Simulated puppet movement produced (170, -60, 335) from anchor (100, -200, 300), then restored gravity from 0 to 9.8. Both native release and Fabric builds passed; the DLL exports SKSEPlugin_Load, SKSEPlugin_Version, pt_mc_init and pt_mc_frame. The checker rejected the deliberately changed mc_x=999 value. The first Gradle attempt picked the pre-existing JAVA_HOME pointing at Java 15; the script now verifies and selects installed Java 25, and the retry passed. Source review caught SKSE's pointer-encoded post-load success flag and corrected its handling before game testing. Linker output includes an informational import-library creation warning.

**Still broken / not tested:** Actual Fabric/SKSE loading, live Skyrim movement/capsule behavior, background execution, and compatibility with other mods are **not tested**. The standalone and simulated checks are not game logs. The approved read of Minecraft's installation now succeeds; its manifest confirms 26.3/Fabric 0.19.5, resolving the earlier access issue. Skyrim's executable confirms 1.7.104 and SKSE is present, but the direct install lacks Data/SKSE/Plugins and the required Address Library there. No game installation files were changed. Camera/input/collision export, rendering composition, combat and save synchronization remain unimplemented.

**Next:** With authorization for game-folder writes (AGENTS.md rule 4) and the matching Address Library installed, install the prepared packages, run both games in offline single-player, and compare real SEND/RECV logs using the README procedure. Stay on this step until the in-game result is verified.

## 2026-10-07 Step 1: standalone connection prototype

**Changed:** Wrote the requested three-part plan in `docs/DESIGN.md`. Implemented only step 1: the reusable Rust connection and two-role probe in `passthrough/`, process tests, and `tools/cargo.ps1` / `tools/test-link.ps1`. Added `README.md` with tested behavior, limitations, and exact commands. Added the missing whitelist `.gitignore`; downloaded tools, logs, build output, and `reference/` remain ignored. Added `rust-toolchain.toml` and Cargo build files. Game installations and the SkyCraft reference source were not modified.

**Why:** Prove the shared-memory connection independently before adding SKSE/Fabric adapters or controlling either player.

**Tested how:** Ran `.\tools\test-link.ps1` on Windows x64 using Rust stable 1.99.0 and MSVC. This ran 2 protocol unit tests and 5 process tests covering both startup orders, heartbeat progress, version rejection, timeout and fresh-session recovery, duplicate-role rejection, orderly shutdown, and isolated sessions. Read the generated logs and confirmed Git excludes reference files, downloaded tools, and build output.

**Result:** All 7 tests passed. In the recovery run, `skyrim.log` recorded `Connected` at 100 ms, `TimedOut` at 3511 ms with `peer_age_ms=2078`, and a fresh `Connected` at 3614 ms. The duplicate logged `duplicate role Minecraft: process 25320 still owns it`. Initial Rust download was blocked by the network sandbox and succeeded after approval. The first build failed because `link.exe` was not discoverable and Windows SDK libraries were absent; the build wrapper locates the installed linker, and project-local Microsoft SDK libraries completed the next build. Rust and SDK downloads stay in ignored `.tools`, with no permanent PATH change. Git's normal ownership check still rejects this repository; inspection used a temporary project-local trusted-directory config, leaving the user's Git config unchanged. Git also emitted a filesystem-monitor daemon warning during inspection.

**Still broken / not tested:** These are standalone probes, not loaded game mods. Actual SKSE/Fabric integration and all gameplay are not implemented and **not tested**. A read of the named Minecraft installation folder was denied, so its contents were not inspected. The abandoned-lock defensive path and other PCs are **not tested**. Manual gameplay was not run.

**Next:** When requested, implement step 2: thin SKSE and Fabric adapters calling the same Rust connection, and verify their handshake in offline single-player logs.

## 2026-10-06 SkyCraft architecture review

**Changed:** Cloned SkyCraft into `reference/SkyCraft` and read its `README.md` and `docs/DESIGN.md`. No mod implementation code was changed.

**Why:** Study the requested reference before planning this project's architecture.

**Tested how:** Read the two requested documents and checked the local clone's top-level layout. No tests were run.

**Result:** SkyCraft keeps each game's native logic and connects them with a Fabric mod, an SKSE plugin, a shared-memory protocol, and shared GPU textures. Minecraft owns player physics and inventory; Skyrim owns the world, NPCs, quests, and saves. Skyrim world collision is translated into Minecraft collision shapes; Minecraft player state and hits are sent back to Skyrim; Minecraft-rendered layers are composited over Skyrim. The design proceeds in phases, starting with a handshake and player movement before rendering, combat, blocks, and persistence.

**Still broken / not tested:** This project's own architecture is not yet planned or implemented. No in-game behavior or compatibility has been tested.

**Next:** Use the reference to plan the first practice milestone in this project's `docs/DESIGN.md` before writing code.

## 2026-10-06 Documentation baseline

**Changed:** Initialized the project guidance and the plain-language design baseline (`AGENTS.md`, `docs/DESIGN.md`), and recorded this entry using the new log format (`MODLOG.md`).

**Why:** Keep project rules, design notes, and change history in the files requested by the user.

**Tested how:** Read the documentation and checked the workspace file list.

**Result:** Documentation is in place. No mod implementation files were present in the workspace at the time of review.

**Still broken / not tested:** The mod itself is not implemented; no game run or automated tests were performed.

**Next:** Before any substantial code change, write the plan in `docs/DESIGN.md` and implement it one step at a time.









