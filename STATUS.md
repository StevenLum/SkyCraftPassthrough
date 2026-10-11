# Handover note: 2026-10-11 (0.10.56 accepted; chair/healing reopened), Codex

## Right now
- State: user says "All working" for .56 left-click tests. Accepted, about to
  commit current accepted source/records on dev under standing instruction.
- User additionally requests ONE more attempt each at chair and healing after
  prior two-attempt stop; permission written to AGENTS.md for both agents.
- No new repair code yet. Diagnosis first, separate steps/versions. User installs.
- Current versions/packages .56, protocol33; baseline77 Rust tests pass.
- Prior uncommitted work since .19 belongs to this project and the recorded
  steps, including still-failed chair/healing work. Snapshot records these failures;
  acceptance does not imply those features work. Last commit fab672b on dev.

## Report / acceptance
- Collected logs/scene-report-20261011-225755 and ran checker. 17 mouse dispatches:
  inventory14/favourites1 item_use, other2 pointer; passive layers skipped1 or2.
  Zero ERROR/disabled. User accepts all requested take/equip/use/category checks.
  Container acceptance is user-reported; no container menu in this log session.
- Chair still has a terrain_blocked exit in this session; no new acceptance.
- Healing: one clamp request2 returned2 health120; zero pre-clamp restore callbacks,
  zero healing forwarded/applied. Damage and native potion effect path under review.

## Next
1. Commit accepted .56 snapshot on dev; no push/master changes.
2. Diagnose chair stabilization/obstruction using logs and native controller state;
   write new plan/header before repair code. Healing investigation is read-only
   in parallel; no concurrent source edits. Build chair step before healing step.
3. Preserve accepted door/bed/held-E/movement/click behavior. Deferred tests and
   door-facing remain deferred. Actual repairs NOT TESTED until user gameplay.

# Handover note: 2026-10-11 (0.10.55 left-click repair built, gameplay pending), Codex

## Right now
- Step: left-click use only, 0.10.55. Attempt 1 of 2 for this separate problem.
- State: built and packaged, waiting for user gameplay. Nothing half-done.
- Files changed: menu_input.rs, skse.rs, switches.rs, Switches.java; comments in
  SkyrimMenus.java; four version files/Cargo.lock; checker and project records.
- No installation, staging, commit or push. Only user installs.
- Chair/healing remain stopped after two attempts, as documented in next section.

## Versions and checks
- Source and both packages: 0.10.55, protocol 33, dist/step5b. No packet changes.
- Package-only offline build: 76 Rust tests (70 unit, 5 connection, 1 position),
  113 Minecraft rule checks and 136 hook checks (23 files), all passed.
- ZIP DLL hashes match release; packaged Fabric manifest is 0.10.55.
- All 22 Rust/Java switch indices agree; new item_click=off is bit 21.
- UTF-8 checked for 114 source/record files; git diff --check passes. Diagnostics
  paths verified ignored. No game assets entered the tracked source.
- Last commit fab672b on dev; preserve all uncommitted work since 0.10.19.
  Do not commit until user says ALL testing passed.

## Findings and change
- User: left click cannot use items; E works. Earlier report:
  logs/scene-report-20261011-221546, 20 Click deliveries and 3 Accept deliveries.
- Native ClickHandler only queues GFx pointer down/up and marks the event handled;
  generic user-event dispatch is then skipped. PC control map links mouse0 to
  Right Attack/Block, remapped to RightEquip in Item Menus. Native favourites
  message7 handler explicitly recognizes Right Attack/Block (UserEvents+0x48).
- Action10 now sends RightEquip with InventoryMenu on top, Right Attack/Block
  with FavoritesMenu on top; other menus keep Click. No additional confirm/click,
  no new wire command, no PlayerControls dispatch. E and other commands unchanged.
- Top selection mirrors native GetTopMostMenu(15), reverse stack first depth<15;
  native getter itself avoided because it retains output reference. Optional
  Inventory/Favourites vtable IDs 215494/215307; missing support falls back to Click.
- Pointer motion from the sample is dispatched before click. SCENE_MENU_CLICK
  records menu, route, native event, flags/context and native cursor coordinates.
- item_click=off restores generic Click after restarting both games. Switch order
  is appended. Report checker tallies route/menu; separate fixture validated it.
- Tests exercise the synthetic native input sink, one press/release, pointer order,
  popup/depth precedence, off/missing support, busy controls and unchanged commands.
  Actual native item consumption/equipping and Flash hover ordering NOT TESTED.
- Read-only native diagnostics are ignored logs/native-*-01055.txt; capstone/lz4
  diagnostic libraries live in ignored .tools/menu-diagnostics. No assets tracked.

## Acceptance kept / remaining failures
- Door idle teleport, owned-bed grounding, held entering E, creative/flat jumps,
  E/WASD stand requests, seated focus/menu/save, close movement and bow checks kept.
- Chair obstruction exits and Minecraft potion healing still fail. Workbench,
  native-hand visibility and old J/K/M/N/O remain not tested/deferred.
- Face away from exit door remains future work. No new rules/permissions in chat.

## Next
1. User closes both games; runs .\tools\install-build.ps1. Expect 3 INSTALLED lines,
   including Fabric 0.10.55. Start both games normally.
2. In I inventory, hover/highlight an item already usable with E, then left click.
   Expect one use/equip, and one-count decrease for a consumable. Holding left click
   for a second should not consume extra. Try another item to check target selection;
   confirm a category click still navigates without unexpectedly consuming an item.
3. In G favourites, left click an already-favourited consumable/weapon. Expect one
   use/equip. E remains usable. Judge native item count/equip mark, not MC healing.
4. One dialogue pointer choice is the limited pointer-route regression, if available;
   don't redo the long shop/chair sequence or deferred map/travel checks.
5. After report: authorized collector then checker; use SCENE_MENU_CLICK routes
   inventory/favourites=item_use, other=pointer. Delivery alone isn't acceptance.
# Handover note: 2026-10-11 (0.10.54 report; chair/healing stopped after two attempts), Codex

## Right now
- State: mixed gameplay result; no new code or build. Nothing half-done.
- Stopped: chair exit and potion healing after two unsuccessful approaches, under
  AGENTS.md Honesty. Do not try another guessed adjustment or hook.
- Files changed this turn: STATUS.md, README.md, MODLOG.md, docs/DESIGN.md,
  docs/ABI.md (report/evidence only). No game installation, staging or commit.
- Waiting on user: nothing. This is the failure handover, not a new retest request.

## Versions and setup
- Source/packages 0.10.54, protocol 33. User tested this repair batch; game version
  binary hash not independently verified this turn. Both switches files absent;
  all features enabled. Minecraft 26.3; Skyrim 1.7.104; offline single-player.
- Minecraft owns normal movement; Skyrim owns furniture animation/exit physics
  while Minecraft is held, followed by a coordinate acknowledgement.
- Baseline .\tools\cargo.ps1 test --offline: 74 tests pass (68 unit, 6 integration).
  Last package build: 113 Java rules and 136 hooks pass; not rerun this docs-only turn.
- Last commit fab672b, dev; all work since 0.10.19 remains uncommitted/unstaged.

## What the user last said / accepted
- 1 failed: lower chair with table ahead, and high stool with NPC in way, still
  fall through ground on exit. Interference is suspected, not proven.
- 2 passed: leaving no longer teleports later. Future request: face away from
  exit door; deferred, not implemented.
- 3 passed on bed: owned-bed grounding check accepted. Workbench not tested.
- 4 failed: potions still do not heal Minecraft; left click cannot use anymore,
  E does. Holding the entering E leaves the character seated: accepted.
- Keep previous creative/flat jumps, E/WASD stand request, focus/menu sitting,
  seated-save recovery, close-combat movement and obstructed/empty bow acceptance.
  Native hand visibility not separately confirmed. J/K/M/N/O remain deferred.

## Current problems and evidence
- Authorized collector report: logs/scene-report-20261011-221546; checker run.
  Protocol 33 both sides; zero ERROR/disabled; 17 acknowledged recoveries.
- Six chair exits, six terrain_blocked results. First: native standing at
  time_ms=264406140, blocked at 264406343; raw target
  [-3131406.221650914,128.04691669940948,-4151278.046381487], 2895 boxes.
  Native gravity remains restored and repeated scans run until the native actor
  moves to a clear position; accepted token 1 only at 264412640 (about 6.5s later).
- Minecraft first blocked-exit position stays [-3131407.3271056246,
  128.12500023841858,-4151279.2397789634]; hold changes seated -> no_snapshot.
  No recorded Minecraft fall in that interval. This narrows that occurrence toward
  native actor/camera motion; does NOT prove why the user saw falling.
- recover/clear_feet only checks sampled terrain; NPC boxes are separate. Refusal
  prevents teleport, but does not stabilize native exit physics/camera. Exact
  intersecting table/floor boxes and native controller/camera heights not logged.
- Owned-bed landing: six waits and six dispatches, 15-94ms; ownership outcome
  accepted by user. Door tokens 6/8 lift y40.013725 to40.236932 and acknowledge;
  user reports no subsequent teleport.
- Healing hook installed once (ActorValueOwner+0xB8, slot6), zero positive restore
  requests, zero captures, zero Minecraft applications. Native health120 while
  Minecraft13.665022. Cannot establish whether full health skips the potion effect,
  another native path is used, or the interception is wrong from these logs.
- Left clicks: 20 action10 mouse requests dispatched/returned game_owned. E:
  3 action5 requests dispatched/returned. action10 = Click/mouse button0;
  action5 = Accept/keyboard Enter. Returned means callback completed, not item use.
  Inventory contexts22/24 and favourites26 show repeated clicks before successful
  E reported by user. This routing predates the .51-.54 repairs; regression cause
  not established. Preserve true pointer-click semantics from .17 until diagnosed.

## Already tried (do not repeat as a new attempt)
- Chair: .50 intentional native exit then alignment failed lower chair;
  .51 fresh terrain scan + <=0.5 upward correction still fails obstructed exits.
- Healing: .30 positive CheckClampDamageModifier capture failed; .53 pre-clamp
  ModActorValue capture and live-menu lease also failed (no native requests).
  Synthetic fixtures pass but do not reproduce the actual native potion path.

## Ideas not tried / next diagnostic work
1. Log native feet/controller/camera height and exact rejected terrain plus NPC
   overlaps during the exit hold; distinguish visual/native falling from MC falling.
   Relevant: skse.rs Puppet::seating/recover, recovery.rs clear_feet, look.rs,
   world_export.rs, SkyCollision.java, BridgeRecovery.java.
2. Trace the real consumed potion/effect and native restoration call path, including
   already-full native health. Do not guess another vtable slot. Relevant: damage.rs,
   menu_input.rs dispatch/menu_lease, SkyrimMenus.java click, BridgeDamage.java.
3. For click failure, establish pointer target versus highlighted item and native
   inventory/favourites handler result; input dispatch alone is insufficient.
4. Future door-facing request is in DESIGN Backlog. No new gameplay tests requested
   until a justified next step exists. Agents package only; user installs.

## Chat rules recorded
- Nothing new; existing AGENTS.md permissions and two-attempt stop rule apply.
# Handover note: 2026-10-11 (repairs: 0.10.46 and 0.10.47 built, 0.10.48 not started), Claude Code

This first section is the handover note (rules: AGENTS.md, "Handover between
agents"; shape: templates/HANDOVER-note.md). Everything below it is history.
Claude Code stopped here because its usage ran out; Codex continues.

## Right now
- Step: repairs after the first play of 0.10.45. Plan: newest section of
  docs/DESIGN.md ("Repairs after the first play of 0.10.45"). Three steps.
- State: step 1 (0.10.46) and step 2 (0.10.47) are coded, tested and built.
  Step 3 (0.10.48) is **not started: no line of it is in any file.** The
  project builds as it is.
- Doing or about to do: nothing is half-done.
- Files being changed: none
- Attempt: 1 of 2 for each of the two problems (freeze at a jump; door that
  did nothing)
- Waiting on the user for: nothing. Nothing from 0.10.46 on has been played.

## Versions
- In the source files: 0.10.47, protocol 31 (no packet change in these steps)
- Built packages in dist/step5b: 0.10.47 (and 0.10.46)
- Installed in the games: 0.10.45
- Last commit: fab672b on `dev`. Uncommitted work: everything from 0.10.19 on
  and its records. **Do not commit: the user said "Don't commit" on
  2026-10-11.** AGENTS.md rule 3 still holds for later.
- Checks at the last build: 50 Rust tests (44 unit, 5 connection, 1
  position), 99 Minecraft-side rule checks, 136 hook checks; all passed.

## What the user last said
"the usage limit is near. Write up the summary for codex to continue". Before
that, the test report: "B failed. There are freezes, freezes dont seem to
happen on a set interval." and "I could not leave the building, I tried this
after all the tests above." The whole report by letter is in the note below.

## What was found (details: the plan in docs/DESIGN.md, MODLOG "Test 0.10.45")
- Freezes: 7 holds `outside_bounds`, all at the start of a jump; the cubes
  were aimed 1.75 blocks down for a fall that had ended. Caused by 0.10.41.
  Fixed in 0.10.46 (`lead_up` in `passthrough/src/world_export.rs`).
- Door: in Honningbrew Meadery E was pressed on a bar stool; Skyrim's
  character sat down and stayed seated while the bridge moved it; after that
  E on doors did nothing. Fits the logs, not proven. 0.10.47 makes E and R
  leave seats alone and logs the sit state and what each press met.

## Next
1. Step 3, 0.10.48, as planned: stand a seated character up.
   - `passthrough/src/seat.rs`: a pure timing rule (suggested: struct `Watch`
     with `look(state, now_ms)`): while the character is moved and its sit
     state is not 0 for 4000 ms on end, ask for a stand-up; again every 2000
     ms; 5 tries, then one "gave up"; state 0, or a gap of more than 500 ms
     between two looks (menu, pause, load), starts the count again. Test it:
     standing; seated; a workbench (2.5 s sitting down, a long menu gap, 1.5 s
     standing up) must never trigger.
   - The call: `Actor::StopInteractingQuick(bool)`, CommonLib
     RELOCATION_ID(37752, 38697), so `resolve(38697)` in `SKSEPlugin_Load`,
     as `unsafe extern "C" fn(actor, true)`. Keep it optional (`.ok()`), in
     `Addresses` (the test near the end of `skse.rs` builds an `Addresses`
     literal and needs the new field).
   - Where: `Puppet::frame` in `skse.rs`, right after the `SCENE_SEAT` change
     line added in 0.10.47 and before `set_position`. Write a log line before
     the call (so a crash inside it can be seen) and one after with the state.
   - New switch `stand_up`, number 15, at the end of `NAMES` in
     `switches.rs` (and its test) and in `Switches.java`.
   - One line for it in `tools\check-test-report.ps1`, section "Seats".
   - Version in four places, MODLOG entry,
     `.\tools\build-step5b.ps1 -Offline -PackageOnly`.
   - It is experimental: the routine has never been called by this mod.
2. Records still owed for all three steps: a section at the top of README.md
   (what changed, switches `seat_guard` and `stand_up`, test list); a section
   at the top of docs/ABI.md (sit state bits 14 to 17 of the word at +0xC8;
   furniture record: type 0x28, bench type +0xE0, marker flags +0xE4, bit 31
   bed; lock record: level +0x00, key +0x08, flags +0x10 bit 0; IDs 20223 and
   38697; new log lines SCENE_SEAT, SCENE_INTERACT_TARGET, `seat=true`); the
   Backlog in docs/DESIGN.md (sitting, riding, mining and wood chopping from
   Minecraft; the indoor scan that hardly ever turns calm; 0.10.41's limit
   corrected).
3. Test list for the user (put it in README.md): install with
   `.\tools\install-build.ps1`. (a) Sprint-jump 200 blocks outdoors: no
   freeze; the log should have no `outside_bounds` on the client. (b) Load
   the last save (inside the meadery, probably seated) and press E on the
   front door: expect to leave; if not, load the autosave from entering the
   meadery. (c) Look at a chair and press E: the Minecraft inventory opens;
   R does nothing. (d) A workbench and a bed still work. Then
   `.\tools\collect-scene-logs.ps1` and `.\tools\check-test-report.ps1`.
4. Do not ask the user for the tests they put off (C, continuous damage,The handover for Codex is written: it is the first section of STATUS.md. Two of the three repair steps are built (0.10.46 and 0.10.47); the third (0.10.48) is not started, and nothing is half-done. Nothing from 0.10.46 on has been played, and nothing is committed.

What I found in your 0.10.45 test
Freezes (B): My change in 0.10.41 caused them. Minecraft held you seven times for 50 to 100 ms, each time at the start of a jump. The collision cube (the box of Skyrim shapes sent to Minecraft) had been aimed 1.75 blocks downward while you were coming down from the previous jump, so your head left the top of it on the next jump.
Could not leave the building: You were in Honningbrew Meadery. At 18:37:46 you pressed E while the crosshair was on the bar stool, and Skyrim’s character sat down. The bridge then carried the seated character around, and a seated Skyrim character does not open doors. This fits every line of the logs but is not proven, because the sitting state was not being logged.
First door outside that did nothing: That is the boilery door, which needs a key. Skyrim was behaving normally there.
What is built
0.10.46: The cube no longer aims far up or down for anything a jump can do. A new test simulates Minecraft’s jump and fails against the 0.10.41 behaviour.
0.10.47: Skyrim chairs, stools and benches are no longer used by E or R; E on one opens the Minecraft inventory. The log now records what every E press met (door, lock, whether Skyrim’s character is seated). New switch seat_guard=off restores the old behaviour.
Checks before the last build: 50 Rust tests, 99 Minecraft-side rule checks, 136 hook checks, all passing.
What is left for Codex
0.10.48: Stand Skyrim’s character up automatically if it is ever found seated. Until this exists, your last quicksave (made while seated) will probably still be stuck.
Records: The README test list, docs/ABI.md and the Backlog still need their sections for these three steps. MODLOG.md and the plan at the top of docs/DESIGN.md are up to date.
If you want to test before Codex finishes

Close both games and install:

.\tools\install-build.ps1

Expect three INSTALLED lines with Fabric 0.10.47. Then:

Outdoors, sprint-jump about 200 blocks. Expected: no short freezes.
In Skyrim, load the autosave from when you entered the meadery, not the quicksave. Walk to the front door and press E. Expected: you leave.
Look at a chair and press E. Expected: the Minecraft inventory opens and Skyrim’s character does not sit.
   health potion, sensitivity, height and depth).

## Known problems and findings not yet fixed
- Until step 3 exists, a save made while seated (the user's quicksave of
  18:38:46) stays seated; the autosave from entering the meadery is clean.
- Sitting as the cause of the dead doors is not proven. If a door fails
  again, `SCENE_INTERACT_TARGET` now shows the lock and the sit state.
- The furniture offsets (+0xE0, +0xE4) are from CommonLib, not seen in this
  game version; `seat_guard=off` puts the old behaviour back.
- E on a horse still mounts it; the bridge cannot ride.
- A bow shot has no visible arrow (the user noted it).
- logs\step2 holds old raw logs; only the user removes them.

## Given in chat this session and already written into AGENTS.md
- Nothing new.

# Handover note: 2026-10-11 (0.10.45 played: result by letter), Claude Code

Closed. The step that follows from it is the note above this one.

## Right now
- Step: third batch, 0.10.37 to 0.10.45, played once by the user.
- State: rejected in part (two failures); the rest passed or was not tried.
- Report: `logs\scene-report-20261011-184225`. Causes of both failures are in
  the newest plan in docs/DESIGN.md.

## Versions
- Source, packages and games at the time: 0.10.45, protocol 31. Last commit
  fab672b on `dev`; everything from 0.10.19 uncommitted ("Don't commit").

## What the user said
"A passed. B failed. There are freezes, freezes dont seem to happen on a set
interval. C did not attempt. D passed. E passed. F conditional pass, enemy took
different damage on short and full draw - I can't see the arrow fly. I did not
shoot at an enemy behind a wall or shoot at the sky. G did not test - I don't
have shouts yet, but I think I hear the skyrim sound effect when the shout is
on cooldown/no shout equipped. H-L - WASD in menus work. Did not test
continuous damage, health potion or sensitivity. M - Runs better. I could not
leave the building, I tried this after all the tests above."

## Result by letter, checked against the logs
- Confirmed: A; D (129 calm scans, 573 re-sends; Skyrim's cost 757
  microseconds a frame, was 1093); E (items, magic, favourites, map); F1 and
  F2 (7 shots, 6 hits at 177 to 321 units, damage following the draw); H (41
  direction keys in menus, 1 repeat); M.
- Failed: B (7 holds of 50 to 100 ms, all at the start of a jump); leaving
  Honningbrew Meadery (E on its doors did nothing).
- G: the key reached Skyrim's player controls 8 times and the user heard
  Skyrim's "nothing equipped" sound; a real shout is not tested.
- Not tested: C, F3, F4, I, J, K. L was not mentioned. The user tests these
  when they choose; do not ask again.

# Handover note: 2026-10-11 (0.10.36 partly tested; nothing in progress), Claude Code

This first section is the handover note (rules: AGENTS.md, "Handover between
agents"; shape: templates/HANDOVER-note.md). Everything below it is history.

## Right now
- Step: none in progress. Two batches, 0.10.19 to 0.10.36, are built, installed
  and played once.
- State: partly accepted; the rest waits for a later test by the user
- Doing or about to do: nothing. No code is being changed.
- Files being changed: none
- Attempt: not applicable
- Waiting on the user for: the remaining tests "later" (their word), or the
  next step.

## Versions
- In the source files: 0.10.36, protocol 31
- Built packages in dist/step5b: 0.10.36
- Installed in the games: 0.10.36
- Last commit: fab672b on `dev`. Uncommitted work: everything from 0.10.19 to
  0.10.36 and its records. **Do not commit it yet: the user said "Don't
  commit" on 2026-10-11**, because part of it is untested. The standing rule
  (AGENTS.md rule 3: commit when the user says all the testing passed) still
  holds for later.
- Checks: 39 Rust tests, 90 Minecraft-side rule checks, 124 hook checks; all
  pass.

## What the user last said
"Question 1: Don't commit. I will not test anything again the 'Not confirmed'
until later. Question 2: Not tested. Question 3: Not tested."
(Question 2 was whether W or S was pressed in a Skyrim menu; question 3
whether a health potion was drunk.)

## Next
1. Wait. Do not ask the user to test the open letters again; they will do it
   when they choose.
2. When they report: `.\tools\collect-scene-logs.ps1`, then
   `.\tools\check-test-report.ps1`. Evidence to look for: E SCENE_MENU_SEND
   source=move_key or source=repeat; H SCENE_DAMAGE_POOL lines with
   pause_ticks above 0 and deliveries with hits above 1; J a
   SCENE_DAMAGE_CLAMP line with a positive "requested", then SCENE_HEAL_CAPTURE
   and SCENE_HEAL_APPLY; F3 a factor other than 1.0; K2 and K3
   buildable=false.
3. If the user asks for more work instead: the first cost numbers point to
   Skyrim's collision ray casting, about 1.1 ms of every frame (Backlog in
   docs/DESIGN.md), and the Backlog lists ranged attacks and swimming.

## Known problems and findings not yet fixed
- Confirmed by play and logs (report 20261011-155446): A, B, C, D, F1, F2, G,
  I, K1, L, M.
- Not tested: E1 and E2 (W, A, S, D and held directions), H (continuous
  damage), J (Skyrim healing). Not exercised: F3 (sensitivity), K2 and K3
  (height and depth), D4 (wait key).
- No hit has yet arrived inside Minecraft's hit pause in play, so the pool
  rule corrected in 0.10.27 is still unobserved; H is the test for it.
- Building is limited to Minecraft's own height range (Backlog).
- logs\step2 holds about 34 GB of old raw logs; only the user removes them
  (`.\tools\prune-logs.ps1 -Delete`).

## Given in chat this session and already written into AGENTS.md
- Nothing new. "Don't commit" is a decision about this work, recorded above,
  not a change to the rules.

# Handover note: 2026-10-11 (batch 0.10.19 to 0.10.27), Claude Code

This first section is the handover note (rules: AGENTS.md, "Handover between
agents"; shape: templates/HANDOVER-note.md). Everything below it is history.

## Right now
- Step: batch of eight steps plus one review correction, 0.10.19 to 0.10.27.
  Plan and result are the two newest sections of docs/DESIGN.md.
- State: waiting for user test
- Doing or about to do: nothing. Everything is built; nothing has been played.
- Files being changed: none
- Attempt: not applicable
- Waiting on the user for: the combined test at the top of README.md
  ("Test 0.10.27", letters A to I). The user installs with
  `.\tools\install-build.ps1`; the agent never does.

## Versions
- In the source files: 0.10.27, protocol 30
- Built packages in dist/step5b: 0.10.27 (0.10.19 to 0.10.26 are there too)
- Installed in the games, as far as known: 0.10.18
- Last commit: fab672b on `dev`. Uncommitted work: the whole batch (source,
  documents, new files SkyrimShortcuts.java and tools/prune-logs.ps1). Commit
  it when the user says all the testing passed, not before.
- Rust tests: 33 pass (27 unit, 5 connection, 1 position)

## What the user last said
"Plan ahead on making new features or optimisations. Continue working on all of
these and give me another test run once complete"

## Next
1. Wait for the user's test result. They report by letter; collect the logs
   with `.\tools\collect-scene-logs.ps1` and check them (allowed, AGENTS.md).
2. What to look for per letter: A SCENE_CROSSHAIR kind=6 on enemies and
   SCENE_INTERACT_SEND source=inventory_key; B SCENE_MENU_SEND/RECV/RESULT
   action=11..14; C SCENE_DAMAGE_POOL and SCENE_DAMAGE_APPLY hits=;
   D SCENE_DAMAGE_SHIELD on/cancelled/off; E SCENE_ATTACK_SEND sharpness= and
   effects=; F SCENE_CURSOR_SEND span_w=; G SCENE_RECOVERY buildable=;
   I the size of the newest files in logs\step2 and no BOX_ lines in them.
3. A letter that fails: fix that step only (each is one version, one MODLOG
   entry). All pass: record it, commit on `dev`, do not push.

## Known problems and findings not yet fixed
- Nothing in this batch is played. Three things rest on unverified behaviour of
  the games: Skyrim's menu-opening handler taking a forwarded key (B), the
  game's hostility test (A5), a paused Minecraft still drawing frames (D).
- Building is limited to Minecraft's own height range; a taller world needs
  the user's decision (Backlog in docs/DESIGN.md).
- The Minecraft side has no automated tests. 0.10.21 was built against the
  wrong counter for that reason and corrected in 0.10.27 by reading.
- logs\step2 holds about 34 GB of old raw logs. Only the user removes them:
  `.\tools\prune-logs.ps1` lists, `-Delete` removes.

## Given in chat this session and already written into AGENTS.md
- Nothing new from the user. The agent added two entries to "Things that have
  already cost time": build with `-Offline -PackageOnly` (the plain build also
  installs into the games), and print Minecraft's code with `javap -c` instead
  of searching it.

# Handover note: 2026-10-11, Claude Code

This first section is the handover note (rules: AGENTS.md, "Handover between
agents"; shape: templates/HANDOVER-note.md). Everything below it is history.

## Right now
- Step: none in progress. The last step, 0.10.18 (real left click; E opens
  Skyrim containers), is accepted.
- State: accepted
- Doing or about to do: nothing. The handover rules were the last thing written.
- Files being changed: none
- Attempt: not applicable
- Waiting on the user for: which step to do next

## Versions
- In the source files: 0.10.18, protocol 29
- Built packages in dist/step5b: 0.10.18
- Installed in the games, as far as known: 0.10.18 (the user tested it)
- Last commit: on branch `dev`. 28bf33d holds everything through 0.10.18;
  859e429 adds the handover rules and this note; one more commit after it
  corrects a wrong statement about line endings. `master` is still the 0.4.0
  state published on GitHub. Nothing has been pushed. Uncommitted work: none
  (check with `git status --short`).
- Rust tests: 28 pass (22 unit, 5 connection, 1 position)

## What the user last said
"All tests passed. But I eventually want pressing E in skyrim to take
precedent. Only open the minecraft inventory when E won't do anything in
skyrim." Also: make documented handover rules so either agent can continue the
other's work at any state, and "Commit every time I say all the testing passes
from now on."

## Next
1. Wait for the user to choose. Candidates, in the order last suggested:
   Skyrim's E taking precedence everywhere (the user wants this "eventually");
   carrying refused small hits forward so continuous Skyrim damage counts;
   protecting Skyrim's character while Minecraft is not active; the height
   window. Details for each are in the Backlog section of docs/DESIGN.md.
2. Whichever is chosen: write its plan at the top of docs/DESIGN.md first, and
   update this note before touching code.

## Known problems and findings not yet fixed
- Rapid small Skyrim hits are mostly lost to Minecraft's half-second hit
  cooldown (measured: 7 of 99 taken).
- While Minecraft is paused or not the active window, Skyrim's character takes
  damage directly and queued hits are dropped.
- Minecraft's height covers Skyrim heights of about -11,600 to 15,000 with the
  current saved origin; places outside are refused with a chat message.
- A left click on Skyrim's map (fast-travel marker) has not been tried.
- The Minecraft side (Java) has no automated tests; only the Rust side does.
- Two different Skyrim places whose record IDs share the same low bits would
  share a Minecraft region (not detected, not yet seen).

## Given in chat this session and already written into AGENTS.md
- Commit each time the user says all the testing passed, on `dev`, never push
  unless asked (both agents).
- Running the log collector and reading its reports is allowed for both agents.
- Run the Rust tests before every build (the build script does it).
- The handover rules themselves.

# 0.10.18 accepted (2026-10-11); nothing in progress

User: "All tests passed." Report 20261011-101205: protocol 29 both sides, zero
ERROR/DISABLED lines; SCENE_CROSSHAIR kinds 2 x13, 3 x11, 4 x4, 5 x9, 1 x2;
SCENE_INTERACT_SEND inventory_key x10, interact_key x8; 17 dispatched, 1
dropped (cooldown); menu commands 75 sent = 75 received = 75 returned (action
10 from left click x25, action 5 from E x5); recoveries 3/3. Map click not
reported. Installed build 0.10.18.

User wants later: Skyrim's E takes precedence everywhere (backlog in DESIGN).
User also asked for documented handover rules and for a commit each time they
say all testing passes; both follow this entry.

# Current: 0.10.18 (real left click; E for containers), awaiting gameplay

Agent: Claude Code. User accepted 0.10.16, confirmed R in a container takes
everything, chose "make left click a real Skyrim click" and asked to add "E
opens the Skyrim container". Plan at the top of docs/DESIGN.md.

0.10.17 (Java only): SkyrimMenus.click() sends command 10 (true click) instead
of 5. 0.10.18: Skyrim publishes a crosshair target kind (attack.rs classify()/
kind(), world.rs target_kind at TARGET_AT=397,440; packet 398,024 bytes, magic
PTWORL13, protocol 29, link29); Minecraft SkyrimAttack keeps it with the
handle; SkyrimInteraction.beforeKeybinds (handleKeybinds HEAD) drains the
inventory key when a Skyrim menu is open (sends confirm) or when the kind is
container/body (queues the activation request). A press from E can never become
"take all" (guard added on review). 28 tests pass (22 unit incl. packet offset
and classification tests, 5 connection, 1 position). Packages 0.10.18 in
dist/step5b. No install or commit. Gameplay **not tested**. Attempt 1.

Unverified: base-object pointer at reference+0x40 and form types on arbitrary
references; life state on NPC bodies; whether Skyrim's crosshair target is set
at the distances the user expects; true click on map markers.
User also asked how the height limit was known: it is arithmetic from the saved
origin (y 100 = Skyrim z -141.6, 70 units per block, 384 blocks), plus Skyrim
sea level and peak heights quoted from memory; not yet observed to fail.

# 0.10.16 accepted by the user; verified from report 20261011-094530

User: A, C, D, E passed; B-11 middle mouse acts like left mouse in Skyrim
(B 5-10 not reported individually). Logs: zero ERROR/DISABLED/fallback lines.
Verified numbers: 6 leftover meshes skipped then 6410 draws (depth usable
indoors); 9/9 recoveries incl. one fast travel; 115/115 menu commands
dispatched once (wheel 25, right click 14, action 9 x5, action 10 x27);
pointer gain remembered from the fourth menu; 102 attacks, 90 dispatched, 11
range drops, loss == requested within 1e-5, scale 1.75, criticals 10.5/2.625;
59 wear events and the sword broke; armour 14 cut a 13.58 hit to 8.43; 2
shield blocks; enemy death -> kill, reload, recovery.

Findings not fixed (in DESIGN backlog): (1) Minecraft's hurt cooldown discards
most rapid small Skyrim hits (99 small hits, 7 taken, 0.69 of 12.76 applied);
(2) while Minecraft sent no active frames for about 19 s, 113 hits passed to Skyrim natively
(52.6 health), and 22 queued hits were dropped at the reset; (3) R as take-all
unconfirmed by the user.

Next candidates: make left click a true Skyrim click (proven by the middle
button); carry refused small hits forward; protection while Minecraft is
paused; height window. Nothing in progress. Installed build 0.10.16.

# Current: 0.10.16 built overnight, awaiting the user's test session

Agent: Claude Code, working alone on 2026-10-11 at the user's request ("make the
fix, continue with more gameplay features, give me a complete test list, do not
ask for testing while I am away"). Nothing here has been played. Install
0.10.16 (dist/step5b); the combined test list is at the top of README.md.

User report on 0.10.9: doors and interiors work and play continues, but blocks
were invisible indoors and, afterwards, outdoors (still solid). Report
20261011-012218: single cause, `SCENE_DISABLED message=invalid scene origin` at
the moment the interior origin became ready; a leftover outdoor mesh could not
be mapped through the interior origin and the world pass disabled itself.

Steps (each its own version, MODLOG entry and DESIGN plan):
- 0.10.10 fix: scene.rs mesh_origin() skips a mesh more than 64 blocks from the
  player (SCENE_WAIT reason=mesh_from_another_place); test with logged numbers.
- 0.10.11 menu controls: wheel = Up/Down, right click = Cancel once per press,
  R in a menu = command 9 "XButton", optional middle-click key = command 10
  true "Click"; verdict()/event() pure rules with tests.
- 0.10.12 pointer gain remembered for the session.
- 0.10.13 critical hits x1.5 by Minecraft's rule.
- 0.10.14 weapon wear via DataComponents.WEAPON and hurtAndBreak.
- 0.10.15 incoming hits as MOB_ATTACK from the nearest Skyrim character:
  armour and shields now apply (generic bypassed both; earlier notes saying
  armour applied were wrong), with knockback as a side effect; generic fallback.
- 0.10.16 review correction: held-button latches lapse if the per-frame poll
  stops.
26 Rust tests pass (20 unit, 5 connection, 1 position), run by every build.
Protocol 28, no wire layout change since 0.10.6. No install, no commit.

Least certain: "XButton" as take-all, the true click, depth-based hiding of
blocks indoors, shield direction in a crowd. Known limits unchanged: Minecraft
height window, no enchantment damage, Skyrim-only death does not reset
Minecraft health. If a step misbehaves, revert that step alone (newest first)
rather than patching several at once. Standing permission: run
collect-scene-logs.ps1 and read its reports.

# Current: 0.10.9 doors/interiors (derived origins), awaiting gameplay

Agent: Claude Code. User accepted 0.10.8 death/respawn, then asked for doors
and interiors ("will also help with fast travel"). Plan at the top of
docs/DESIGN.md. Idea: the saved PBOR origin stays the origin of its own place;
any other place (interior cell, or another worldspace) gets an origin derived
from its form ID by a fixed formula (interiors: 2048-block grid from low 24
bits of the cell ID, y 128; worldspaces: 8192-block grid east of it from low 16
bits, y 136; flipped row if near home). Nothing new is saved, so Skyrim reloads
cannot reassign regions. Travel then uses the existing recovery handshake.

Code: saved_origin.rs for_location/square + restore (too-far now Err 5, was 2);
save_identity.rs origin/recovery_target/menu_authorized; Java BridgeRecovery
rejection message, BridgePairing reason-5 text. No wire/record change, protocol
28. Tests: 20 pass (14 unit incl. 5 new place tests and a simulated door in the
skse.rs test, 5 connection, 1 position), run by the build. Packages 0.10.9 in
dist/step5b. No install or commit. Gameplay **not tested**. Attempt 1 of 2.

Unverified: door activation while puppeted, loading screen/cell change timing,
interior Havok sampling, multi-million-block Minecraft teleport and ground
detection there, return trip, fast travel. Known limit: height window (home
maps Skyrim z about -11,600..15,000). Standing log permission applies.

# 0.10.8 death/respawn accepted (2026-10-11)

User confirms reconnect after death; report 20261011-005940 verifies it: death
age 19 ms, health 100 -> -200, life 0 -> 1 (next frame) -> 2 (+2.5 s) -> 0
after reload, reload at +6.3 s, SCENE_DEATH clear, recovery token accepted in
31 ms, origin_ready, Minecraft ready 7115 ms after death, zero ERROR lines.
Attempt 2 succeeded; no handoff needed. Life-state offset (Actor+0xC8 bits
21..24) is now confirmed but lags the hit by one frame. F5/F9 verified in the
previous report (2 saves, 6 loads). Installed build 0.10.8, protocol 28.

Open, in rough priority: Skyrim-only death does not reset Minecraft health;
death from real combat/fall not yet exercised; 0.10.5 damage scale unverified
in combat; items in the DESIGN backlog. Nothing in progress. Standing
permission: Claude Code may run collect-scene-logs.ps1 and read its reports.

# Current: 0.10.8 death reconnect repair, awaiting gameplay (attempt 2 of 2)

User tested 0.10.7: F5/F9 worked; `/kill` killed and respawned Minecraft with
inventory kept and killed the Skyrim character, but the bridge did not
reconnect. User granted standing permission for Claude Code to run
collect-scene-logs.ps1 and read its reports, and asked for the Rust unit tests
to be fixed and run before every build.

Cause (report 20261011-005252): world.rs decode rejected hold reason > 6; the
new reason 7 reply failed the sender's own encode check ("event=ERROR
message=invalid world snapshot" at the kill frame), frame() returned Err and
the Skyrim adapter set failed=true. Kill itself worked: health 100 -> -200,
reload 6.3 s later. Life state read 0 right after the call (unconfirmed offset).

0.10.8: HOLD_DEAD/MAX_HOLD_REASON constants (7 accepted); hold on health <= 0
plus 3 s grace, life only logged (SCENE_DEATH_LIFE); skse.rs test fixture
repaired and extended with a simulated death; world.rs hold-reason test;
build-step5b.ps1 runs `cargo test --offline` first; collector keeps ERROR/LINK/
RELEASED/ANCHOR/load/hook lines and Java sender-disabled traces; AGENTS.md has a
"Test before build" rule. 15 tests pass (9 unit, 5 connection, 1 position).
Packages 0.10.8 in dist/step5b. No install or commit. Reconnect **not tested**.

If the reconnect fails again: do not try a third variation; write the
template-based handoff. Still untested: 0.10.5 damage values in combat.

# Current: 0.10.7 built (death 0.10.6 + quick keys 0.10.7), awaiting gameplay

Agent: Claude Code. User chose "Skyrim-style: reload the last save" for death,
then asked for F5 quicksave / F9 quickload before testing. Plans are at the top
of docs/DESIGN.md. Both built with build-step5b.ps1 -Offline -PackageOnly;
0.10.7 packages in dist/step5b contain everything. No install, commit, tests
or log reads. All gameplay below is **not tested**; 0.10.5 damage also untested.

0.10.6 death: ServerPlayer.die observer -> death number + age in frames for
10 s -> Skyrim death.rs: release puppet, DoDamage(37335) on the player with
null source, hold (reason 7) while dying/dead or 3 s after the attempt; hook
passes the lethal hit (reason=bridge_death). Bridge worlds force immediate
respawn and keep inventory. Reload then uses the existing recovery handshake.
Protocol 28, record 456 bytes, pt_mc_frame14. Attempt 1 of 2 (attempt 2 would
be Actor::KillImpl virtual 0x10E, then handoff).

0.10.7 keys: SkyrimSaves keys F5/F9 -> menu command 7/8 with context 0 ->
menu_input.rs gameplay gate -> ButtonEvent "Quicksave"/"Quickload" through
MenuControls. Perspective-toggle clicks are drained in bridge worlds when the
keys coincide. No layout change.

Unverified risks: DoDamage on the player with no attacker (could fail or
crash), life-state offset 0xC8, QuickSaveLoadHandler accepting forwarded
events, timing of Skyrim's background reload, recovery after death reload.
User was told to make a named Skyrim save first. Log permission is per report.

# 0.10.5 untested by user choice; next feature: death and respawn (planning)

User declined to test 0.10.5 ("sounds like it should feel right") and asked to
continue with the next feature. Damage x1.75 stays **not tested**; verify
scale=1.75 and health loss from the first later combat log.

Next feature chosen from the earlier recommendation: death and respawn. Code
reading so far (nothing run): when the Minecraft player dies the client sends
inactive samples, Skyrim releases the puppet, and Skyrim's character stays put
under native control, so native hits reduce Skyrim health (reason not_ready).
After Minecraft respawns at world spawn, the existing save-position recovery
handshake (origin reason 5 -> token -> integrated-server teleport) should move
the Minecraft player back to Skyrim's position. That path was built for save
loads and has **never been tested** for death. The Minecraft overlay is not
captured or drawn while dead, so the death screen only shows in Minecraft's own
window. Waiting on the user's choice of what dying should do before planning.

# Current increment 0.10.5: outgoing damage balance, awaiting gameplay

User asked for the deferred balance pass. SkyrimAttack now multiplies Minecraft
attack damage by DAMAGE_SCALE=1.75 (was 10): fist 1.75, wooden sword 7, up to
netherite 14, chosen to line up with Skyrim swords iron 7..daedric 14 (base
values from memory, not game files). Recharge, guards, native code, protocol 27
and incoming damage unchanged. Log field `scale` added to SCENE_ATTACK_SEND.
Build passed with build-step5b.ps1 -Offline -PackageOnly; 0.10.5 packages in
dist/step5b. No install, commit or tests. Gameplay **not tested**.

Backlog: the user asked for the other deferred items to be listed. They are now
grouped in the Backlog section near the top of docs/DESIGN.md (the user said
"MODLOG"; the section was created in DESIGN, so it was kept there and the user
was told). Log permission still per report: ask before reading the next one.

# 0.10.4 dialogue pointer accepted (2026-10-11)

User: "The mouse works inside an NPC conversation." Feature accepted. User
deferred pointer speed scaling to the backlog (new section at the top of
docs/DESIGN.md, which also holds the first-movement doubling defect; no
decision on fixing that yet). User asked what gameplay feature comes next;
nothing new is started. Installed build stays 0.10.4. Punch after menu untested.

# 0.10.4 pointer verified from report 20261011-001910

User granted reading this report. Synthetic MouseMoveEvent ("Cursor", id 0xA)
is accepted: read-back changes synchronously, exactly 2 px per count on both
axes, same sign, screen 2560x1440. Contexts 1 and 3 had pointer movement (move
counters 380 and 51) and 8 mouse confirms; contexts 5 and 7 had only keys.
24 MENU_SEND = 24 RECV dispatch = 24 RESULT returned, no duplicate sequence.
User: keys work in NPC dialogue; block place/break fine after menus.

Open: (1) initial gain guess 1 vs real 2 doubles the first small moves in each
menu until an 8+ count move; candidate fix is to keep the learned gain between
menus. (2) Pointer in NPC dialogue itself not yet shown by logs or user report.
(3) Punch after menu not tried. (4) Speed with an 854x480 Minecraft window is
about 3 Skyrim px per mouse unit; ask user if it feels right. Documentation
only; installed build stays 0.10.4. Log permission so far covers this report.

# 0.10.4 pointer accepted by user (2026-10-11)

User: "That seems to work." Skyrim Tab menu opened from Skyrim's window, then
Minecraft focused: pointer followed the mouse, left click moved through the
menu, Backspace exited, bridge reconnected, movement resumed. Attempt 1
succeeded for this path, so the two-failed-attempts stop does not apply.
Not verified by logs: Claude Code still has no permission to collect or read
them. NPC dialogue hover/click, pointer speed numbers, and regressions to mouse
look, block breaking and punching were not separately reported. Documentation
only; installed build stays 0.10.4. Open follow-ups if wanted: mouse wheel,
right click, a true Skyrim click event for containers/barter.
Encoding: the old dash byte in this file ("bytes 240-255" line) was found
already turned into a replacement character on 2026-10-11, cause not
established; it is now a proper UTF-8 dash. README.md still has its one byte.

# Current increment 0.10.4: dialogue mouse pointer, awaiting gameplay

Agent this increment: Claude Code **Steven here-- I'm using Opus 5.5 High** (handover from Codex). User request: "make the
skyrim mouse work when talking to an npc". Plan is at the top of docs/DESIGN.md.
Minecraft captures mouse motion at MouseHandler.turnPlayer while Skyrim menu
status is fresh and sends a running total (window fractions) plus menu context.
Skyrim's UI task turns the change into a MouseMoveEvent ("Cursor", id 0xA) via
MenuControls::ProcessEvent, reads MenuCursor back and learns pixels per count.
Left click queues menu action 5 (confirm) and skips vanilla attack in menus.
Protocol 27, record 440 bytes, pt_mc_frame13; world/scene unchanged.

Build passed with build-step5b.ps1 -Offline -PackageOnly; packages 0.10.4 in
dist/step5b. No install, no commit. Gameplay **not tested**. Attempt 1 of 2.
Unverified: synthetic event acceptance, user-event name/idCode, pointer speed,
vertical direction, hover highlight. If read-back never changes, attempt 2 is
sending the hover to the menu movie directly; a second failure means handoff.

Found, not fixed: `cargo test --lib` does not compile because the skse.rs test
fixture lacks Addresses fields hud_vtable/interaction (older than this change).
So the new record round-trip test was **not run**. Flagged as a separate task.

Permission note: the user has not yet told Claude Code it may run
collect-scene-logs.ps1 or read game logs. Ask before doing either.

Encoding note for either agent: README.md and STATUS.md each still contain one
Windows-1252 dash byte (0x96, in "bytes 240-255"). Claude Code's edit tool turns
that byte into a replacement character, so add to those files by prepending
bytes, not by an in-place edit.

## 2026-10-10 Recharge verified (0.10.3)

Report scene-report-20261010-225957 final combat sequence22:58:55-22:59:05:
19 sends,19 receipts,19 results, sequences1..19 against targetFF000D9E.
Wooden sword:687ms interval full40;454ms charge0.76 damage26.483198;
250ms charge0.44 damage14.195199 (twice). Fist:156/141ms charge0.7 damage5.92;
longer intervals full10. All requested amounts match measured native health loss
within0.00005 health points (printed float rounding). Sequence spans9781ms.
This confirms tested recharge scaling for wooden sword and fist. Damage balance
still deferred: user reports full-strength damage too high. Axe, block-priority
regression and HUD indicator remain **not tested** here; HUD is not wired to this
bridge timer. No rebuild/reinstall or repeat test needed.

# Current increment0.10.3: attack recharge, awaiting gameplay

User says current damage is far too high: future balance task recorded in DESIGN
and README; full damage scale remains10x.0.10.2 sword40/fist10 verified.
Implemented item ATTACK_SPEED compute and elapsed client tick recharge; first
request per player/world full, item switches share timer, stale snapshot clear
preserves timer. Only emitted bridge requests consume it. Multiplier0.2+0.8*q^2.
Client/native100ms spam guard; protocol26/frame12 unchanged payload. Logs include
speed/elapsed/charge/multiplier/full damage/request. Build passed, no install/commit.
Gameplay **not tested**. Vanilla HUD indicator does not reflect bridge recharge.
Manual: rapid sword pair then two-second wait/hit, optional axe and block break.
Read logs with rg --no-ignore. Earlier weapon-calculation attempt2 succeeded;
no two-failure block applies to the accepted sword computation.

## 2026-10-10 Held-item damage verified (0.10.2)

Report scene-report-20261010-224713 confirms wooden sword sequence1: base1,
computed attribute4, request40, target0009F361 health22 ->-18. Empty-hand sequence3:
base/computed1, request10, targetFF00091A health5 ->-5. Both native losses match
the request exactly. User reports wolf killed by sword and mudcrab by punch;
actor species are user observations, not identified from these log fields.
Sequence2 was an empty-hand request against the earlier handle with no dispatch
or result; its rejection reason is not logged. Do not count it as applied damage.
The second weapon-calculation attempt succeeds for tested sword/empty hand.
Other weapons, effects, durability, recharge and block-priority regression remain
**not tested** here. No new build or repeat gameplay test needed.

# Version 0.10.2: held-item damage repair, attempt 2

Build passed; gameplay **not tested**.0.10.1 returned client attribute1 for both
sword and fist. This build uses player's base attack value and the held item's
ATTRIBUTE_MODIFIERS component through Minecraft's MAINHAND compute method.
No player/inventory mutation. Existing10x scale,500ms cooldown/range and native
transport guards remain. Temporary player effects, enchantment bonuses, recharge,
criticals and durability are outside this base-item increment.

Close both games, run `.\tools\install-build.ps1`; expect three INSTALLED lines
and Fabric0.10.2. Hit a hostile enemy empty-handed, then equip an unenchanted wooden
sword and wait one second before hitting (use a second enemy if needed). Sword
should deal more damage than fist. Tell me Done for direct log collection.
Logs now include base/client attribute/component count/computed attribute/request.
If this second runtime attempt fails, stop for a documented handoff.

## 2026-10-10 Held-item damage test failed (0.10.1)

Report scene-report-20261010-223504: sequence1 wooden_sword logs attribute1.0,
requested10, native target0009F361 health22 ->12. Sequence2 empty hand (air)
logs attribute1.0, requested10, native health12 ->2. Both deal10: transport
matches, but current client attribute read does not distinguish the equipped
sword in this test. Exact underlying cause not yet investigated. First runtime
attempt failed; no fix made. Next investigate equipped-item damage calculation
before another build. No repeat test or reinstall needed for this documentation.

# Current increment: 0.10.1 held-item base damage

Implemented ATTACK_DAMAGE attribute times10 at click time, immutable request
float, native finite/positive/<=1000 guard. Existing range, expiry, target and
500ms cooldown remain. No durability/enchantment/critical/weapon-speed handling.
Protocol25, connection424 offset412 f32, pt_mc_frame12; world unchanged.
Built twice successfully, final pass after correcting hardcoded Fabric manifest.
Gameplay **not tested**. User installs and compares fist vs unenchanted wooden
sword, then normal block breaking. Do not infer full Minecraft combat simulation.
Remember rg --no-ignore when searching logs directories. No install/commit.

## 2026-10-10 Punch logs verified; previous search conclusion corrected

Report scene-report-20261010-222744 contains three matching attack SEND, dispatch
and RESULT sequences (1..3), target000EBC4D: health22 ->12 ->2 ->-8, exactly10
per punch. Five incoming captures also match five accepted Minecraft damage
applications. Fabric startup reports0.10.0 and native startup protocol24.
The previous report221938 also contains three complete punch traces with the
same health progression. Earlier claims that attack logs were missing were
incorrect: rg skipped the gitignored logs directory. Use rg --no-ignore for
log-directory searches. The collector and attack logging were working.
Negative target health alone does not verify kill attribution/death processing.
Range rejection, empty aim and block priority remain **not tested** here.
Documentation only; no rebuild, reinstall or repeat fight required.

# 0.10.0 punch accepted by user (2026-10-10)

User reports being able to punch and damage the enemy. Collected report
scene-report-20261010-221938 contains no SCENE_ATTACK events; searching raw
logs/step2 for SEND/RESULT also found none. Do not infer numeric damage or dispatch
counts from this report. Exact damage, out-of-range/empty aim, block priority and
regressions remain **not tested**. Documentation only; no new build or install.
Next: choose the next gameplay increment when requested. If investigating attack
logging, establish running adapter versions and source session first.

# Current increment: 0.10.0 outgoing basic punch

User approved normal Minecraft attack input against Skyrim actors, preserving
block breaking. Added startAttack observer, target-handle mailbox and native
DoDamage37335, player source, fixed10 damage, cooldown500ms/range210/samecell.
Requires fresh admitted bridge and same crosshair handle. Original MC attack
continues; real MC blocks/entities have precedence. Post-attack menu/load guard.
Build passed twice; runtime **not tested**. No install/commit or test suite run.
Protocol24/connection424/PTWORL12/world398016/pt_mc_frame11.

Install-build.ps1 with games closed; test close hostile NPC, distant/empty aim,
normal block breaking, incoming damage and R. Collect logs directly afterward.
Need SEND/RECV/RESULT sequence plus native target-health decrease. No claim yet
of native combat reactions, kill attribution, weapon stats or animations.
User clarified previous final native hit was after closing Minecraft and Skyrim
while chased; ignore as expected standalone damage, not a new bug to fix.

# 0.9.9 live damage interception accepted

User confirms success; report20261010-220115: epoch3 sequences1..3 captured,
applied and acknowledged once each. Minecraft20 ->18.6422 ->17.737 ->16.831799;
Skyrim stays100 on those hits. Minecraft later17.831799. Second hook attempt
succeeded for this path, so the two-failed-attempt stop condition was not reached.
No further hook variation needed. Documentation only; current build0.9.9.

Limitation observed: fourth hit not_ready at177713671 passes through and native
health falls93.211 after Minecraft coverage disappears/save events. Do not claim
protection outside ready bridge. Next address not-ready/death lifecycle policy;
other damage sources/death/full regeneration not yet verified. No new install.

# Incoming damage: attempt 2 awaiting runtime result (0.9.9)

Attempt1 (0.9.8) failed. Report20261010-215421 shows ready=true/max100 and two
HandleHealthDamage probes (-6.789), but zero ModActorValue capture/passthrough.
Native damage bypasses that intercepted virtual entry in the tested path.
Attempt2 replaces hook with documented CheckClampDamageModifier primary slot0x127,
float(this,ActorValue,float), prior called once, result zero only after capture.
CLAMP logs input/result/currenthealth; existing queue/scaling/MC path unchanged.
Runtime **not tested**. If this also fails, STOP and complete the template-based
handoff, not a third hook variation. Capstone unavailable; no disassembly done.

User test: install-build.ps1 with games closed, load paired saves, one nonlethal
hit then retreat. Collect logs directly. Need clamp/capture/apply/ack and preserved
native health. Full death and scripted setters still unsupported. No install or
commit performed. Earlier statements that ModActorValue intercepts hits are false
for the observed enemy path and superseded by this evidence.

# Current increment: 0.9.8 incoming damage bridge built

User asked to implement forwarding and native suppression. Implemented mutation
hook on actual PlayerCharacter ActorValueOwner+0xB8 slot6, modifier2 health24,
queue32/ack/readiness, maximum-health fractions, server hurtServer(generic).
Native health loss suppressed only after enqueue while ready; otherwise original
runs. No Skyrim regen forwarding. Full death/respawn and scripted setters not
covered. Runtime behaviour **not tested**; this is the first forwarding attempt.

Protocol23/connection400/PTWORL11 world398016/pt_mc_frame10. Build passed twice;
review fixed a shadowed collision-count variable before final packaging. Final
MC326169 / Skyrim220335 bytes. No installs/commits/tests run. Prior approval usage
limit resolved on continuation. Local CommonLib and javap API evidence recorded.

Next user test: install-build.ps1 with games closed, load paired saves, verify
movement/collision, receive two nonlethal hits >=1s apart, retreat and heal with
Minecraft food. Collect logs directly. Need matching CAPTURE/APPLY/ACK IDs,
no duplicate APPLY, native health retained and Minecraft hearts affected.
Native fallback outside readiness and dropped pending events on reset/expiry are
logged limitations, not complete health/death authority in all circumstances.

# 0.9.7 probe completed: HandleHealthDamage is after health mutation

Report 20261010-120922: two hits, supplied -6.789 and -4.526, attacker EBC4D.
Health on entry already 93.211 / 89.694016 and unchanged after original callback.
Both measured_loss=0. Do NOT derive event damage from before-after here, or assume
skipping HandleHealthDamage suppresses health loss. Different callback threads.
587 received health snapshots match; regeneration reaches 100; MC health stays20.
No native ERROR/LOAD_FAILED events found. User test complete, no new build.
Next investigate earlier health mutation for Minecraft-owned authority; all
forwarding/death/suppression remains unimplemented. Documentation only this turn.

# Current: 0.9.7 individual damage callback probe

User chose Minecraft health, food/healing and eventual death ownership. Choice is
resolved; implementation staged in DESIGN. Built native HandleHealthDamage probe
(slot 0x104) with sequence, supplied amount, before/after current health, attacker,
revision/thread logs. Original called exactly once; no damage suppression or
Minecraft heart writes yet. Probe **not tested** in game; build passed.

Next: install via `.\tools\install-build.ps1`, verify save load, receive two
ordinary Skyrim hits, collect logs directly. Determine callback semantics before
forwarding damage with acknowledgements and enabling health/death authority.
Protocol 22 unchanged. No commits or game-install writes.

# Pending gameplay choice: health/healing authority

User requested continuation after accepted read-only health test. Asked whether
Minecraft controls hearts/food/healing or Skyrim controls health/healing/death.
Common duplication/load-baseline/scaling rules written in DESIGN; no health writes
or new build yet. Choice must be resolved before dependent implementation.
Current build remains 0.9.6. No new test needed for documentation-only update.

# Read-only health observation accepted (0.9.6)

Report 20261010-120304 confirms both user-reported enemy hits and regeneration:
100 -> 93.22922 -> 100; 100 -> 93.28407 -> 100. All 1043 native receive snapshots
match send values. Minecraft health remains 20 in all client records. No native
ERROR/origin-block events found. No new build or code changes this turn.
Next: design health/damage authority and death/load baseline handling before
writing health across games. Attacks and cross-game damage remain unimplemented.

# 0.9.6 stability and baseline health verified in one run

Report 20261010-115741: 329 health exports over 69719 ms, 327 received and all
match exactly. Skyrim current health 100, Minecraft health 20. Later save events;
no logged bridge errors or recurrence of load crash in this run. Documentation
only; installed build remains 0.9.6. Next user test is Skyrim damage then healing
outdoors with paired bridge active. Collect logs directly afterward. Damage/regen
is not tested yet, and health writes/attacks remain unimplemented.

# Current: 0.9.6 health-read crash correction

0.9.5 crashes on save load; user never reached damage/regen testing. Collected
report 20261010-115402. Found concrete wrong ActorValueOwner offset: 0xB0 was
used, but Actor.h versioned accessor requires 0xB8 since Skyrim 1.6.629, including
1.7.104. Corrected call and docs; 0.9.6 Rust/Fabric packages built successfully.
No crash dump, so exact faulting instruction is not independently established.
Runtime correction **not tested**. This is the first evidence-based repair.

Next user test: `.\tools\install-build.ps1` with games closed; load same save
through SKSE with paired Minecraft active, wait 30 seconds, walk briefly. Do not
ask for damage/regen until load stability confirmed. Collect logs directly.
Protocol 22/PTWORL10 unchanged; no commits or install writes.

# Current increment: 0.9.5 read-only health feed

Dialogue and recovery accepted; next planned direction is combat/damage.
Implemented its first diagnostic step only: Skyrim current player health at
world header 68, native SEND/RECV logs and client log with Minecraft health.
Protocol 22/PTWORL10, packet size unchanged. Read-only; no shared health or attacks.
Rust/Fabric build passed; gameplay **not tested**. No installs/commits/tests run.

User procedure: `.\tools\install-build.ps1` with games closed, load paired saves,
get hit by an existing hostile Skyrim enemy outdoors and allow healing. Collect
logs directly afterward; compare keyed SCENE_HEALTH_SEND/RECV and CLIENT values.
Health/death ownership must be designed before writing health across games.

# 0.9.4 accepted: dialogue and outdoor return

User confirms Up/Down once per selection, Enter selects, Backspace returns.
Report 20261010-114651: 14 native dispatches without duplicate sequences; menu
contexts stable across callback threads. Building entry blocks reason 4 at
interior 000133C8/worldspace 0; exiting through Skyrim controls restores the
same saved origin and user confirms reconnection. Interior play unsupported.
Three scoped recovery repairs have user acceptance. Documentation only this turn;
no new build, install, commit or additional gameplay feature. Other menu types,
rebindings and held-key suppression remain not tested. Historical notes follow.

# Current increment: 0.9.4 menu state repair

Built successfully, game behaviour **not tested**. Shared mutex now owns native
menu context and deduplication across SKSE UI callback threads. Thread-local state
contains only mapping handle; nonblocking lock prevents reentrant deadlock. Native
menu event route unchanged. Thread IDs added to state/receive/result diagnostics.
Install `.\tools\install-build.ps1`; test NPC R, arrows, Enter, Backspace and
movement resume. Collect logs directly. Do not claim menu selection works yet.

Recovery report 20261010-114219 confirms token 1 accepted with error_squared=0,
47 ms after request, matching both Minecraft positions. User says appears working.
Protocol 21/PTWORLD9 retained; no install writes or commits. If menu dispatch is
now stable but selection still fails, investigate dialogue routing next. Stop and
write a handoff if two real attempts at the remaining issue fail.

# Current increment: 0.9.3 save-position recovery

Outdoor fix accepted by user and report 20261010-113517 (9731/9712 within 3C).
Step 2 implemented, built and packaged; game recovery **not tested**. Install via
`.\tools\install-build.ps1`, then test a previously mismatched paired load and
quicksave/walk/quickload with a marker block. Collect logs directly afterward.

Recovery holds native control while Minecraft's normal integrated-server
teleport moves to the saved-origin inverse of the loaded Skyrim position. A
server/client readiness acknowledgement unlocks native admission. Token expiry,
load/session/world/location checks and duplicate prevention included. Protocol 21,
PTWORLD9 397488, pt_mc_frame9; all adapters need update. No origin remapping or
historical inventory restoration. Native recovery timeout is 30 seconds.

Both builds passed; final packages MC 311896 / Skyrim 212826 bytes. No tests run
or game installs. Menu arrow navigation and thread-local context/deduplication
remain known failures for step 3. No work on that feature in this increment.

# Current increment: 0.9.2 outdoor cell continuity

Recovery plan written in docs/DESIGN.md. Only step 1 implemented and packaged;
build passed, game behaviour **not tested**. Install with
`.\tools\install-build.ps1`, then cross the previously failing outdoor boundary
and return to a marker block. Collect logs directly after user testing.

Report 20261010-112905: reason 4 at outdoor 9732 -> 9731 within worldspace 3C.
Origin now permits same-worldspace exterior cells and logs outdoor_cell_accepted;
interiors retain exact-cell checks. Mapping and 140-unit proximity guard unchanged.
Protocol remains 20. No installs/commits. Known outstanding failures: initial load
reason 5 despite matching lineage; menu context token oscillation and duplicate
sequence 10 dispatch (thread-local state despite multiple UI callback threads).
Next steps: explicit Minecraft-position recovery handshake, then global menu
state/deduplication repair. Do not claim 0.9.1 dialogue navigation works.

# Current handoff: 0.9.1 menu navigation

Implemented and packaged; in-game behaviour **not tested**. Rust/Fabric build
passed using `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No install or commit.
Protocol 20, 384-byte connection, pt_mc_frame8 plus pt_mc_menu. Both native copies
and Fabric jar must be installed together with `.\tools\install-build.ps1`.

Arrows/Enter/Backspace are remappable Minecraft keys for open Skyrim menus.
SKSE UI tasks run during paused menus; validate fresh paired samples and saved
location, deduplicate expiring requests against a menu context token, dispatch
ButtonEvent pairs through MenuControls. Minecraft movement holds while menu
status is fresh. No mouse/text, repeat, or inventory synchronization.

Next: user tests NPC choice navigation/confirm/back, movement hold/resume, held
keys, rebinding, Minecraft-screen suppression, and a pausing container. Collect
logs directly with `.\tools\collect-scene-logs.ps1`; compare SCENE_MENU_SEND/RECV
and STATE/RESULT events. Returned dispatch is not proof of a visible action.
Post-build inspection initially hit approval-service usage limit, now resumed.
Prior accepted work and historical handoff notes follow.

# STATUS

## Goal
Make Minecraft gameplay operate inside Skyrim, with Minecraft owning movement,
inventory and placed blocks. Current increment: Skyrim location identity feed.

## Current increment (0.9.0): Skyrim interaction
User chose gameplay before travel/rollback. R (remappable Minecraft key) invokes
Skyrim's normal crosshair activation with one short-lived sequence per press.
Pairing/origin checks precede activation; expiry/deduplication/cooldown/resume gates
prevent delayed/repeated requests. Native menus suspend bridge control, but menu
input routing is next and currently requires Skyrim's own window. Build passed
on second attempt (first lacked GLFW import; fixed to InputConstants.KEY_R).
User confirms NPC conversation opens; report 20261009-214457 shows sequence 1
dispatched once, returned, then menu blocked/unblocked. Three later input presses
were discarded as inactive_or_menu. Other object types, rebinding, empty aim and
held-key behaviour remain **not tested**; see README for the procedure.
Protocol 19, pt_mc_frame7; all adapters must be installed. Next gameplay steps are
menu input routing then combat/damage; location travel/rollback remain deferred.
README encoding-write failure recovered from Git snapshot plus known acceptance
update; no Git history or user code was reset.

## Accepted 0.8 release scope (0.8.9)
User approved ending 0.8 with restart alignment at the same practice location.
0.9 owns save revisions/rollback, doors/interiors, construction per location and
coordinate remapping. PBOR origin serialization implemented, paired to Minecraft
folder UUID and exact Skyrim cell/worldspace, with form resolution and 140-unit
resume proximity guard. Protocol 18, 288-byte connection, PTWORLD8. Rust/Fabric
builds passed. User reports block/inventory persistence after restart. Report
20261009-212809 plus Skyrim sessions 19424/17024 confirm identical saved/restored
world ID, location and origin vectors. Scoped 0.8 restart milestone is complete.
An earlier setup attempt hit reason=5 after Minecraft coordinates changed greatly;
user reloaded a legacy save before establishing the final successful origin. Cause
of that coordinate change is not established. No checkpoint matching/world selection.

## Previous repair (0.8.8)
0.8.7 failed matched-save startup (user report and 204741 logs: eligible=false,
no admission events). Fixed transient null-overworld rejection caching and early
inactive pairing reads. Also corrected ID overlap with renderer status by using
bytes 240–255, protocol 17. Build passed. User confirms normal movement/rendering;
report 20261009-205054 confirms eligible=true, matched and native allowed=true.
Matched-save repair passed. User and report 20261009-205719 confirm missing-ID
blocking, mismatch blocking and recovery to the original paired lineage. Native
admission changed true -> false (missing) -> false (mismatch) -> true (restored).
Next: persistent origin/restored-coordinate design; lineage is not save revision.

## Previous increment (0.8.7)
Native lineage admission now precedes puppet movement/look/rendering. Expected ID
travels with position samples; rejected requests receive identity-only replies
without usable coverage. Initial pairing can still proceed. Protocol 16/PTWORLD7,
pt_mc_frame5. Runtime **not tested**. See README for matched/mismatch/recovery test.
0.8.6 pairing confirmed by user and report 20261009-204217 after reopening.
Save revision checks and persistent origin restoration remain unimplemented.

## Previous increment (0.8.6)
Diagnostic Minecraft pairing implemented. Explicit Pair confirmation creates
world-root passthrough-pairing.txt once; future IDs log matched/mismatch and warn
on mismatch. Protocol 15/PTWORLD6 appends 16 lineage bytes. Build passed, gameplay
**not tested**. Install both adapters, pair with identified Skyrim save and reopen
Minecraft to check matched without re-prompting. Collect logs directly afterwards.
No native admission gate, restored origin, save revision matching or automatic
save selection yet. Stop bridge play on mismatch; do not claim it is blocked.

## Previous increment (0.8.5)
Skyrim save-lineage identity implemented using SKSE serialization callbacks.
Versioned PBID record contains a Windows-generated 128-bit ID; SCENE_SAVE_ID logs
created/saved/loaded/reverted and failures. Build passed. Skyrim-only save/restart/
load/save test passed: the same ID was created/saved in process 27040 and
loaded/saved in process 14360. Minecraft pairing remains unimplemented.
Next: install, save to a new slot, restart/load that slot, save again and collect
logs directly. Save copies inherit ID; save revision and Minecraft pairing are
not yet implemented. No restart-origin restoration. Keep previous save slot.

## Previous increment (0.8.4)
In-memory pause-origin retention implemented and built. Same Minecraft session,
same Skyrim cell, no load/reset and <=1 Skyrim unit displacement are required
for reuse. SCENE_ALIGNMENT explains suspension/reuse/rejection. User confirms
the pause test behaves as expected. Collected report 20261008-212546 shows three
origin reuses, two rejections for Skyrim movement while suspended, and a final
suspension without a subsequent resume. One rejection follows a 5.55-unit vertical
change; its cause is unconfirmed. Not every interruption preserved the origin.
No disk persistence or save binding. See README for the exact manual procedure.

## Previous increment (0.8.3)
Cell/worldspace record IDs now cross in world snapshots (PTWORLD5, offsets
112/116). SCENE_LOCATION_SEND/RECV expose location transitions. Build passed;
runtime is **not tested**. No save binding or construction remapping yet. Install
with tools/install-build.ps1, collect reports in the same cell before/after a
restart, and compare IDs. IDs depend on plugin load order, not Skyrim save identity.
User requested deferring sprint after 0.8.2 report showed a speed increase.
The supplied 211142 report showed only an initial 548 ms coverage hold and 256
scans averaging 142.13 ms; broader travel tests remain pending.

## Setup
- Minecraft 26.3 / Fabric; Skyrim 1.7.104 / SKSE; Windows 11.
- Agent: Codex. Rust stable MSVC native bridge plus Java 25 Fabric adapter.
- Project: D:\Documents\ChatGPT\Game modding.
- Minecraft: C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3.
- Skyrim: E:\SteamLibrary\steamapps\common\Skyrim Special Edition.
- Template: templates/STATUS-handoff.md (user corrected its location).
- Current packaged version: 0.9.0, protocol 19; dist/step5b.

## What works (tested)
- User confirms 0.8.1 save resuming works.
- User reports the basic 0.8.0 empty bridge-world setup works well.
- User confirmed movement, jumping/landing, wall/NPC blocking and camera following.
- User confirmed basic blocks, drops, transparency and HUD layering.
- User confirmed normal Minecraft adjacency placement and some direct floor placement.
- 0.8.0 Rust and Fabric builds succeeded. This is not a gameplay test.

## What doesn't work yet
- 0.8.0 empty-world creation, runtime hooks and movement hold/resume are **not tested in game**.
- Long-distance coverage, unlimited altitude and stable Skyrim save/cell alignment are not implemented.
- Direct Skyrim placement in 0.7.1 was displaced toward the player.
- User accepts current block placement for this iteration; broader verification remains pending.
- 0.7.3 filled water/lava bucket targeting is implemented but not tested in game. Fluid containment remains absent.
- Sampled box edges can supply artificial faces; exact terrain normals remain future work.
- Four-block collision coverage, grid gaps, conservative occupancy and support rules limit placement.
- Glass/ice cracks and previously reported camera/world-bleed issues remain deferred.

## Which game owns the player
Minecraft owns position and mouse look; Skyrim follows.

## Current travel increment (0.8.2)
Implemented a bounded predictive scan centre using rendered sample displacement
and timestamps, with a horizon based on the previous scan duration. Horizontal
lead is capped at 1.5 blocks and vertical at .75. Radius four, sampling spacing,
ray count, per-update work, expiry and existing movement-hold rules remain.
Added SCENE_WORLD_SCAN and detailed SCENE_BRIDGE_MOTION reasons/hold durations.
No wire change, larger sampling volume, shape decoder or stable save mapping.

Rust/Fabric build passed; runtime travel improvement is not tested. No gameplay
logs read, automated tests added/run (Gradle NO-SOURCE), or game installs changed.
Normal shell execution failed before running due to sandbox setup refresh;
reviewed escalated project-only execution succeeded. No compile failures.

Next: install 0.8.2 (three INSTALLED lines), Resume bridge, walk/sprint, reverse,
try slopes/stairs/jumps/short drops, briefly pause/resume Skyrim, and collect scene
logs (three COLLECTED lines). Provide the report path. Compare lead/timing with
outside_bounds versus freshness holds; do not widen sampling without evidence.

## Latest save-resume increment (0.8.1)
User raised inventory persistence when using New Skyrim bridge. That button is
first-time setup, not a session-start requirement. 0.8.1 adds Resume bridge using
Minecraft save summaries and the normal loader, with explicit name/folder review
for the most recently played marked bridge. New bridge now explains that it starts
a separate inventory. Other saves remain selectable from the ordinary world list.
Vanilla remains the only inventory/block save system. Inventory diagnostics record
join_return and save_return on the server, with save/player identity, item IDs,
counts/equipment and XP. They do not prove disk durability or compare components.

Final Rust/Fabric builds passed. First compile failed because screen/setScreen
moved to Minecraft.gui in 26.3; inspected current signatures and corrected it.
Two normal shell attempts failed before executing due to sandbox setup refresh;
reviewed escalated execution restored project-only access. No gameplay logs read,
no automated tests run (Gradle NO-SOURCE), no game installs modified. Resume UI,
logging and the persistence round trip are not tested in game.

Next test: install 0.8.1, reopen existing bridge, note inventory, Save and Quit to
Title, Resume bridge, compare items/equipment/XP and collect scene logs while
Minecraft stays open. Next implementation priority after this is travel coverage;
full Skyrim-save pairing/rollback and stable world alignment remain later work.

## The current problem
Generated Minecraft terrain obstructed Skyrim travel. User requested the dedicated
empty-world plan and authorized step 1. 0.8.0 adds a New Skyrim bridge button to
Singleplayer, creates a separate marked all-air save, gates passthrough to that
save's overworld, and holds movement until fresh collision covers the player and
initial terrain support exists. Ordinary saves keep normal rendering/movement.
Initial terrain generation logs non-air counts; natural mobs and spawn scatter
are disabled only in the marked bridge save. No existing save migration.

World protocol 13/PTWORLD4 adds centre/radius in reserved header bytes. The actual
radius remains four blocks. Movement holds on absent coverage, without expanding
sampling. Active stationary position samples bootstrap Skyrim alignment/export.
This local feature is not unrestricted travel: coordinate-window remapping,
world/cell save binding and ahead-of-motion coverage remain later steps.

Build failure: removed displayClientMessage API caused the first Java compile to
fail. Inspected current APIs and switched to sendSystemMessage/sendOverlayMessage;
subsequent and final builds passed. No automated tests run (Gradle NO-SOURCE).
No gameplay logs read or game installs modified. No runtime behavior verified.

Next: close both games, run .\tools\install-build.ps1 (three INSTALLED lines).
Stand on Skyrim ground; Minecraft Singleplayer > New Skyrim bridge. Expect waiting
then ready, no generated terrain, and working Skyrim grounding/walls plus deliberately
placed Minecraft block collision. Test loss of coverage and ordinary-world gating.
Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines); user supplies report.
SCENE_BRIDGE_WORLD/CHUNK/COVERAGE/MOTION identify setup and hold/resume. Do not
read other logs automatically. See README for full procedure and limitations.

## Previous placement investigation

0.7.0 required the aiming ray to cross the cell above the actual terrain contact.
At shallow angles it could miss. 0.7.1 projected the ray onto the rounded grid
plane, moving the cell toward the player. Both attempts were inadequate; work
stopped and was documented. The user subsequently authorized continuing.

0.7.2 separates actual terrain contact, destination cell and packet point. Keep
cell coordinates anchored to contact and clamp the packet point into that cell.
Carry contact metadata via a bounded synchronized mailbox inside the offline
client/integrated-server process. Client validation queues the intent before
vanilla sends its predicted-use packet. Server consumes a matching intent once,
checks fresh origin/surface, reach, obstruction and occupancy, then lets vanilla
place the item. Capacity 16, expiry 500 ms, clear on bridge reset. No second
placement operation or native protocol change. No new gameplay logs read this turn.

## Evidence
The user supplied logs/scene-report-20261008-161512/{minecraft-scene.log,
skyrim-scene.log,minecraft-client-scene.log} in the prior investigation.
- 16:13:07 snapshot 514: eye (32.699560,70.137890,34.938651), contact
  (31.666421,68.062130,36.853716), face up, selected cell (32,69,35).
  Contact distance 3.007263; cell-centre distance only 0.872835; horizontal
  displacement from contact 1.589780. This record is a selected target, not a click.
- Snapshot 707 recorded successful client/server dirt placement into the same
  displaced destination. Agreement did not establish correct targeting.
- Server decisions: 13 accepted, 5 occupied_terrain, 1 no_fresh_surface.
- Throttled misses: 35 no_fresh_terrain_hit, 8 grid_plane_outside_ray,
  1 missing_or_changed_snapshot. These are not per-frame totals.
- 875 received scene metadata/checksum records matched sends; zero mismatches.
- 0.7.2 compiled and packaged with build-step5b.ps1 -Offline -PackageOnly.
  No automated tests added or run; Gradle test tasks NO-SOURCE.

## What we've already tried
- 0.7.0 ray-through-cell: placement possible but inadequate range.
- 0.7.1 ray/grid-plane projection: logs confirm destination moves toward player.
- 0.7.2 separate contact intent: implemented after renewed user authorization;
  build passed, runtime behavior not tested. Do not claim the bug fixed yet.
- This turn an apply_patch delete/add replacement was rejected before editing
  that file; used a direct source write. README replacement initially found no
  expected version heading; corrected with the headings actually present.

## Ideas not tried yet
- Export actual targeted terrain hits/normals to avoid sampled-box side faces.
- Broader terrain coverage after measuring capacity/timing.
- Buckets and terrain-aware fluid simulation after placement is validated.

## Files that matter
- docs/DESIGN.md: 0.7.2 plan before implementation.
- minecraft/src/main/java/dev/passthrough/SkyCollision.java: contact-anchored cell.
- minecraft/src/main/java/dev/passthrough/TerrainPlacement.java: intent lifecycle and validation.
- minecraft/src/main/java/dev/passthrough/mixin/TerrainPickMixin.java: client target hook.
- minecraft/src/main/java/dev/passthrough/mixin/TerrainPlaceMixin.java: normal item-use guard.
- passthrough/src/world_export.rs: movement tiles; normals not retained in wire data.
- README.md: exact 0.7.2 manual procedure.

Close both games and run .\tools\install-build.ps1 (three INSTALLED lines).
Use /gamemode creative and /give @s minecraft:stone 64; compare near/far floor
placement, walls and ordinary adjacency. Check survival consumption, stale targets
and server corrections. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines).
Only read logs explicitly supplied or requested by the user, not installation logs
automatically. Compare intent IDs, original surface, chosen cell and final state.




