## 2026-10-11 result: 0.10.56 accepted; chair/healing extra attempts authorized

User says "All working" for the focused left-click checks. Report225755 confirms
15 item_use/2 pointer dispatches and skipped passive render layers. The route fix
ran; actual take/equip/use/category acceptance comes from the user. Record
container pass as user-reported, not present as a container click in this log.
Current .56 source/package tests77/113/136 pass. Commit accepted snapshot on dev
under standing instruction before new repairs; failed chair/healing stay recorded.

User explicitly requests one further attempt at chair and healing despite two
prior failures. AGENTS records this scoped exception. Diagnose first, chair then
healing as separate steps/versions. No repair code yet. If the extra attempt fails,
stop and hand over; do not retry guesses. Preserve other accepted/deferred checks.

## 2026-10-11 result: input-menu click repair 0.10.56 built, gameplay pending

.55 failed in report224351: every one of52 clicks selected a rendering-only menu
(flags0x8840/context19), so the native item-use branch never ran. Conversation
and buying are user-accepted; take/equip/use remain failed until retested.
.56 skips passive custom rendering only when contextNone19 and no input owner
flags; other popups/menus stop the reverse/depth15 search. Container is explicitly
recognized too. Inventory/container get RightEquip, favourites Right Attack/Block;
barter/dialogue/unknown menus keep Click. Existing authorization, E and packets
stay as before. item_click=off retains rollback. Logs expose skipped layers/index/
identity; fixtures cover the actual renderer case and input barriers.

Built package-only/offline:77 Rust tests,113 Java rules,136 hooks pass. Source and
packages .56/protocol33. No install/commit. Actual use and category targeting are
NOT TESTED; README asks focused checks. This is attempt2 of2 for left clicks;
if it fails in game, stop and write the failure handover. Chair/healing stay stopped.

## 2026-10-11 left-click input-menu selection (candidate 0.10.56), plan before code

Attempt2 of2, limited to the failing item clicks. User accepts conversation and
buying; taking/equipping/using inside inventories still fail. Report224351 has
52 mouse dispatches but ALL route=pointer/menu=other, flags0x8840/context19.
The .55 rule found the top rendering menu, not the input menu. No evidence yet
that RightEquip or favourites use failed, because neither was actually sent.

1. Walk the same bounded UI stack in reverse/depth<15, skipping only passive
   custom-rendered menus with contextNone19 and no pause/cursor/menu-context/modal/
   update-cursor/item/application input ownership flags. Flags0x8840 is the actual
   failing case. Stop at other menus including popups; do not search through a
   dialogue, barter or modal into an underlying inventory. Missing IDs/off keeps
   generic Click. Log skipped count, selected stack index/vtable and flags/context.
2. Add ContainerMenu to the explicit optional identity list (AE215061); native
   message7 compares RightEquip+0x230 and calls Flash AttemptEquip, verified in
   installed runtime. Inventory/container use RightEquip, favourites Right Attack/
   Block. Other input menus keep generic Click. No double event, no new packet,
   no E changes. Keep item_click switch21 for rollback.
3. Add a meaningful regression with the observed0x8840 rendering menu above
   inventory, and test popup/modal/unknown-menu barriers, barter and dialogue,
   container routing, depth cutoff and skipped menus. Verify actual synthetic
   sink receives one press/release after cursor motion; preserve missing/off.
4. Version0.10.56 in all four places/Cargo.lock; Rust tests then offline package-only
   build/Java rules/hooks. Update STATUS/MODLOG/README/ABI/Backlog. User alone
   installs and checks take/equip/use and category targeting. Gameplay NOT TESTED.
   If attempt2 fails, stop and hand over, without a third guessed fix.

## 2026-10-11 result: left-click repair 0.10.55 built, gameplay pending

Left mouse now uses the native inventory/favourites item actions instead of
sending only a pointer click. Other menus retain pointer clicks. Pointer motion
is sent first; one press/release, existing E behavior and authorization checks.
The two known menu vtables and native top-menu depth/order identify the route,
so a popup above inventory gets its own pointer click. Missing support and new
item_click=off restore generic clicks. Native events come from installed control
mapping and verified handlers; this candidate still needs actual gameplay.

76 Rust tests, 113 Java rules, 136 hooks passed in the offline package-only build.
ZIP DLL/manifest and22 switch indices verified. Checker route/menu counters tested
separately. No install or commit; protocol33 unchanged. README requests only
inventory/favourites use, holding left click, category navigation and one dialogue
choice if available.
Judge native item count/equip mark; healing is not the pass condition. Chair and
healing repairs remain stopped; accepted/deferred gameplay preserved.

Limitations: native Flash hover ordering and item use are NOT TESTED. Stock left
mouse events are used; Skyrim remapped controls/custom menu vtables not tested.
Optional click binding shares the same contextual action10. No game assets or
native disassembly were written into tracked files.

## 2026-10-11 left-click item use (candidate0.10.55), plan before code

User asks to repair left-click use separately. Twenty mouse Click events reach
Skyrim but E/Accept is the only reported working use action. Diagnose the native
Click handler, real item/favourites input mapping and pointer dispatch order.
Read installed game data/binary only at runtime or in ignored .tools; do not
write game assets/decompiled code to tracked source.

Native diagnosis before implementation: generic ClickHandler accepts only Click
and queues GFx mouse down/up; the handled event skips the generic user-event
message route. PC control map links mouse0 to Right Attack/Block, mapped to
RightEquip in Item Menus. The native favourites message7 branch explicitly
recognizes Right Attack/Block (UserEvents+0x48). Hence candidate repair: action10
maps to RightEquip only with InventoryMenu on top, Right Attack/Block only with
FavoritesMenu on top; other menus keep Click. Top selection mirrors native
GetTopMostMenu depth15: walk menuStack backwards, first depth byte<15. Do not
call that native getter because it retains an output reference. Inventory/Fav
identities use optional verified vtable IDs215494/215307. Missing IDs or new
item_click=off switch21 restores generic Click. Move queued pointer motion before
click dispatch; never send Click and use together. Log route/native context,
flags and cursor coordinates. Tests must exercise actual synthetic button sink,
overlay precedence and missing/off support, not just string selection.

1. Establish what the actual native handler expects for left mouse in item menus.
   Compare against synthetic event construction; distinguish pointer selection,
   item equip/use and delayed UI processing. Read recorded cursor/click timing.
2. Fix only the demonstrated mismatch. Keep true pointer clicks in dialogue/map;
   prevent duplicate uses and stale releases across menu/pair changes. If the fix
   depends on unverified live behavior, append a rollback switch in both adapters.
3. Add regression examples for the new event/routing rule and native adapter
   behavior. Log chosen route and relevant state to judge actual gameplay later.
4. Bump all four version locations together; offline package-only build after
   Rust+Java rules+hook checks pass. Update records/Backlog and handover. Gameplay
   not tested until user result; user alone installs. No chair/healing changes.

## 2026-10-11 result: 0.10.54 report; chair/healing repair attempts stopped

User accepts door exit without a later teleport, owned-bed grounding, and held
entering E. Chair exit still fails around lower chair/table and higher stool/NPC;
exact obstruction not established. Potion healing still fails; left click cannot
use items while E can. Native model visibility not separately confirmed.

Report logs/scene-report-20261011-221546: six terrain-blocked chair exits,17
acknowledged recoveries, six grounding waits/dispatches,20 native mouse click
and3 confirm dispatches, zero healing requests/captures/applications despite hook
installation. First blocked chair interval holds Minecraft position; native
stand-up physics remains free. Need native/controller/camera height and exact
terrain/NPC overlap evidence before claiming the cause of apparent falling.

Chair .50/.51 and healing .30/.53 are two unsuccessful approaches. Stop repairs
and use first STATUS failure handover per AGENTS.md. No new code planned, no new
build or repeated gameplay test. Native effect/click path must be established;
synthetic callback tests cannot prove actual potion behavior. Version0.10.54,
protocol33 unchanged; baseline74 Rust tests pass. Face-away-from-door is future
work, not part of this diagnosis. Previous accepted/deferred checks preserved.

## 2026-10-11 result: 0.10.51 to 0.10.54 built, focused play pending

Terrain-safe arrivals, bounded bed/crafting landing, full-health/menu potion
capture and reversible native first-person arm visibility completed as separate
versioned steps. Final package-only build: 74 Rust tests, 113 Java rule checks,
136 hooks passed. ZIP hashes/Fabric version and 21 switch numbers verified.
Protocol stays 33; save origins and ordinary flight prediction stay unchanged.

User accepted A/E/F/H/I and E-to-stand in C. Original sequence can leave but its
outside teleport, low chair floor/camera, bed grounding and potion healing failed.
Repairs are NOT TESTED in gameplay. README now requests only four focused retests;
old deferred checks are optional. No install, stage, commit or push. STATUS holds
present state; plans below are historical execution detail.

## 2026-10-11 native first-person hands (0.10.54), plan before code

The user reports a Skyrim hand in the lower chair view. Existing look.rs forces
first-person mode but never hides Skyrim's first-person arm node, so that node
can render beneath Minecraft's transparent view. The exact low-chair camera
cause is not established; the unwanted native model itself can be suppressed.

Pinned CommonLib verifies TESObjectREFR::Get3D1(bool), virtual slot 0x6F, and flat
NiAVObject flags at 0xF4, bit 0 hidden (SetAppCulled). Retain the first-person node
using NiRefObject atomic refcount at 8 while recording its original hidden bit.
Hide only that root while bridge look owns view; reapply after native updates.
On release/model replacement, restore only its original hidden bit and release
our retained reference (DeleteThis slot 1 if last). Do not change other flags,
third-person skeleton, camera height or furniture animation. Add native_hands
switch 20 to restore old behavior. Tests cover repeated frames, visibility state,
model replacement/null, refcount balance and already-hidden model. Separate build,
no install or commit; actual rendering remains NOT TESTED.

## 2026-10-11 repairs after 0.10.50, plan before code

Observed results are recorded at the top of README/MODLOG and in STATUS.
Keep creative flight, E/WASD standing, pause/menu seating, loaded-seat recovery,
close combat movement and bow obstruction acceptance. Do not reopen deferred tests.

1. 0.10.51: recovery must wait for a fresh terrain scan around the destination.
   If native actor feet overlap a thin sampled floor, lift the Minecraft target
   only enough to clear that terrain, within half a block; refuse if no standing
   clearance exists in that bound. Existing token acknowledgement still owns the
   teleport and saved origins stay unchanged. Apply to doors, loads and chair exits.
   Retain movement hold while scanning. Add reported-coordinate floor, ceiling,
   wall and clear-air regression examples; fixture must wait when scans incomplete.
2. 0.10.52: beds/crafting furniture need Skyrim's physical grounded state. Defer
   their activation while restoring native gravity, hold Minecraft, wait for the
   published Actor::IsInMidair test to clear, then dispatch the request exactly
   once. Keep ordinary activation freshness and bounded timeout; log before/after.
   Resume through terrain-safe recovery if native feet moved. Owned bed should
   produce Skyrim's own ownership rejection; workbench remains a regression check.
3. 0.10.53: intercept the ActorValueOwner ModActorValue restore entry (verified
   local CommonLib slot 6, kDamage) before a full-health clamp can erase the request.
   Preserve the native call, suppress double capture through nested clamp, and
   authorize healing across paired Skyrim inventory/favourites menus independently
   of combat-ready expiry. Clear on load/pair/world replacement/disconnect. Tests
   cover full native health, menu pause, duplicate capture, regeneration rejection,
   and ordinary negative damage routing. Existing healing switch remains.
4. First-person hands: investigate native model visibility and view state; do not
   guess offsets. If supported, hide only native first-person model while bridge
   view owns rendering, retain lifetime safely and restore its original visibility
   on release. Separate version/plan update after evidence, not bundled above.

Each code step gets tests and an offline package-only build, plus MODLOG/README,
ABI and handover updates. Gameplay fixes remain not tested until the user's report.
No install, commit or staging.

## 2026-10-11 result: 0.10.49 and 0.10.50 built, waiting for focused play

Separate flight and seating steps completed without installation or commit.
Final package-only build: 66 Rust tests, 113 Java rule checks, 136 hook checks.
ZIP hashes/metadata, 18 switch numbers and native/Java frame15 argument types
verified. Quiet creative scans preserve slow pacing while standing. Records,
packet notes, checker and README A/B/C are current. The first STATUS section
contains present state; older repair plans/results are history.

New gameplay is **not tested**. Do not label the original door sequence passed
or ask to repeat accepted flat-ground sprint-jumps. Native seated-save recovery
worked once in the user's last logs; deliberate sitting is a distinct new mode.

## 2026-10-11: intentional chair sitting (0.10.50), plan before code

0.10.49 built: 59 Rust, 104 Java rules, 136 hook checks passed. Gameplay not tested.
The user wants E to sit and either WASD or E to stand; keep timed recovery for
old seated saves. Do not carry Skyrim's seated actor to Minecraft's moving feet.

Recognize ordinary sit-capable furniture (bench 0, marker bit 30, no bed bit 31).
Only allow chairs when the stand-up address is available and the new sitting
switch is on. Other non-menu furniture remains guarded (wood chopping/riding
are separate deferred modes); beds/workbenches keep their menu handling.

After a chair activation, hold Minecraft's position and restore Skyrim's saved
gravity. Skip all Skyrim position/velocity writes while entering, seated and
standing up; let Skyrim own its furniture animation/position. Continue view,
scene, health and fresh input status. Publish held status every Skyrim frame
using the formerly reserved u32 after target kind (0/1), protocol 33. No synthetic
collision area is used for movement: Minecraft checks held status first.

The intentional-seat state belongs to the paired sender/world/save. Retain it
through focus/pause/menu suspension, clear on load/death/pair change. Activation
must actually enter non-standing within four seconds or expire. No timed recovery
while deliberately seated. WASD or a new, fresh E request asks native stand-up,
with bounded retries; the activating E is never replayed as stand or activation.
Consume all activation requests while seated. When actually standing, use the
existing coordinate recovery handshake to align Minecraft with Skyrim's furniture
exit position before steering resumes. Keep the saved mapping unchanged.

Add pure state-transition tests for sit indefinitely, E/WASD, freshness, failed
activation, retry cap, sender/save replacement, resume and natural standing.
Adapter test checks zero position writes, restored gravity, native stand-up and
coordinate handshake. Java tests check E override with no crosshair while seated
and seat-hold precedence. Add sitting switch 17; update records/test instructions.
Package-only build. Animation/camera/menus/doors in this mode remain not tested;
test the user's actual enter/shopkeeper/menu/shout/sit/stand/leave sequence.

## 2026-10-11: creative flight coverage (0.10.49), plan before code

User accepted seamless sprint-jumps on relatively flat tested ground, but reported
interruptions in creative ascent and falling after disabling flight. Preserve the
ordinary jump rule. The original meadery sequence and beds/workbenches are not tested.
The native seated-save recovery worked once; intentional sitting is the next
separate feature: E sits, WASD or E stands, per the user's answer.

Flight report: logs/scene-report-20261011-194102. Nineteen downward and two mainly
sideways holds while rising, about 50 ms. Empty collision scans take 67 ms, then
another frame before starting the next; a scan can be used for about two scan
intervals. Snapshot 386 bottom 26.5 was above the requested 26.1975; snapshot 693
had zero lead despite steady horizontal movement. A rendered-frame velocity can
be a transient zero or a spike and the old accepted speed excludes terminal falls.

Keep the half-block sampling grid and the existing two-millisecond time budget.
For fast movement, allow up to 1536 casts per frame within that same budget. Use
at least 50 ms of position history for the velocity; invalid/discontinuous samples
reset it. Accept actual creative/fall speeds up to 64 sideways and 100 vertical.
Flight or falling faster than jumps scans a taller rectangular area: horizontal
radius 4, vertical radius 12 (24 at speeds above 40). The vertical lead in this mode
uses the longer area; the ordinary .75/1.75 jump rule stays unchanged. No geometry
is claimed outside the area actually scanned, and no collision check is bypassed.

Add input flags (WASD=1, permission to fly=2) at previously reserved sample word 396, and
vertical radius in an extended 144-byte world header. Protocol 32 and PTWORL14;
all sizes/offsets/parsers updated together. Native export frame15 adds controls.
New fast_scan switch appended at bit 16 restores the prior scan/aim behavior.
Flight permission rather than current flight keeps tall coverage through the
flight-off transition and the first accelerating fall.
Tests cover every face ray at half-block spacing, full-fall and ascending coverage
through two scan intervals, ordinary jump regression, flags and packet round trip.
Package-only build; actual creative movement and scan costs remain not tested.

## Repair build result, 2026-10-11: 0.10.48 ready, gameplay not tested

Codex completed the third step from Claude Code's plan. Package-only build
passed: 55 Rust tests, 99 Minecraft rule checks, 136 hook checks. Both 0.10.48
packages are in dist/step5b; no game installation or commit was made. README
has the focused A to D test, ABI notes cover the new offsets/call/log lines,
and the Backlog records the unsupported furniture/mount modes and indoor scan.

The four-second delay, two-second retries, five-call cap, resets and workbench
menu sequence have pure tests; a simulated adapter call checks true and the
standing-state change. The real StopInteractingQuick call, seat record offsets
and gameplay outcomes are not tested. The log checker and package metadata
were verified separately. Wait for the user's repair test before more changes.

## 0.10.48 execution details, 2026-10-11, Codex

Continuing step 3 of the repair plan immediately below. The timing rule is pure:
watch only frames in which the bridge is about to move the character. Standing,
a gap greater than 500 ms, a backwards time value, or explicitly releasing the
character resets it. Nonzero sit-state changes do not restart the four seconds.
Calls occur after four seconds and at least two seconds apart, five at most;
wait a final two seconds before logging one gave-up outcome. No further calls
until the watch resets. This gives the last call time to finish.

Resolve Actor::StopInteractingQuick optionally (CommonLib Actor.cpp,
RELOCATION_ID(37752, 38697), void return, actor and bool arguments). If absent,
log that recovery is disabled and continue without it. stand_up is switch 15.
Log before entering native code and after returning. After a call, recheck menus,
load/readiness and cell identity, then reacquire the movement controller before
using it: the native call may have changed the character's interaction state.

Tests cover standing, first-call/retry boundaries, five calls and one gave-up,
nonzero state changes, resets, and workbench sit/menu/stand timing. Extend the
simulated adapter test to check the bool argument and actual standing state
change without requiring a real game. Then run the package-only build and finish
the README, ABI notes and Backlog for 0.10.46 through 0.10.48. Gameplay remains
not tested, including whether recovering the seated quicksave fixes its doors.

## Repairs after the first play of 0.10.45, 2026-10-11: plan for 0.10.46 to 0.10.48

User, after playing 0.10.45: "B failed. There are freezes, freezes dont seem to
happen on a set interval." and "I could not leave the building, I tried this
after all the tests above." Written before the code, by Claude Code. Report:
`logs\scene-report-20261011-184225`. Three steps, one version each.

### What the logs say

**The freezes (letter B) come from the look-ahead added in 0.10.41.** Minecraft
held the player seven times while moving, 50 to 100 ms each time, every time
with the reason `outside_bounds`; the session before had none. All seven are
at the start of a jump. In Skyrim's log the four cubes involved (snapshots
445, 457, 486, 499) were all aimed with `lead` = -1.75 blocks on the height
axis: the scan started while the player was coming down at the end of a jump
(7 to 8 blocks a second), so 0.10.41 placed the cube 1.75 blocks lower, as if
the fall would go on. It ended on the ground, the player jumped again, and the
top of the cube was 0.2 block too low. The same happens the other way round: a
cube aimed 1.75 up at the start of a jump leaves 0.13 block under the feet at
the landing. With the limit used until 0.10.40 (0.75) neither can happen: the
cube then holds a whole jump with 0.3 block to spare.

Nothing else froze: Minecraft drew 120 frames and Skyrim 59 frames in every
second of that test, the six changes of Skyrim cell went through without a
hold (0.10.37 works, and the collision scan kept its number through them, so
outdoor cells do share one physics world), and the other holds of the session
belong to Skyrim menus and the start.

**The door: Skyrim's character had sat down on a bar stool.** The building is
Honningbrew Meadery (cell 00015207, read from Skyrim.esm in the install
folder, read-only, nothing copied).
- 18:37:46 (326.3 s in Skyrim's log): E with the crosshair on an object of
  kind 1 ("other"), 123 units from the stool at the bar, the only seat in
  reach. Skyrim's camera jumped 230 units away and 90 up for 1.5 s, which is
  what it does while its character sits down. From then on the camera stayed
  6 units higher than in the two minutes before (and than in report
  20261011-094530): Skyrim was showing its "seated" first-person view.
- The bridge kept moving the seated character to wherever Minecraft walked.
  Nothing in the bridge knows about sitting, and nothing made Skyrim's
  character stand up: in Skyrim that takes a movement key, and movement keys
  never reach Skyrim.
- 18:38:28 to 18:38:44: E on the inner door 000A43C7 (not locked, not a way
  out) nine times and on the front door 00015C2A once. Every press was handed
  to Skyrim's own "use what the crosshair is on" routine and came back, and
  nothing followed. E on Sabjorn 40 seconds earlier had opened his dialogue:
  talking works from a seat, doors do not.
- Going in had worked through the same pair of doors, 170 s earlier, standing.
- The first door outside that did nothing (18:35:36) is the boilery's, which
  needs a key: that one was Skyrim behaving normally.

Not proven: that sitting is what stopped the doors. The logs fit it at every
point and nothing else in them changed, but the sit state itself was not
logged and Skyrim's door code cannot be read. Step 2 logs it.

The quicksave made at 18:38:46 was taken in that state, so it may load seated.
The autosave made on entering the meadery (18:35:44) is from before the stool.

### Step 1, 0.10.46: the cube stays put for a jump

`aim` in `passthrough/src/world_export.rs`: the limit on the height axis is
0.75 block again up to 13 blocks a second, which is more than any jump, any
landing after a jump and any drop of up to 2 blocks reaches, and grows evenly
from there to 1.75 at 19 blocks a second (a fall of 4 blocks and more, fast
flight up or down). Sideways it stays 2.75.

Test for the rule (it would have caught 0.10.41): Minecraft's own jump
arithmetic is run in the test, jumping again on landing, on ground at several
heights; a cube aimed at any moment of that must still hold the player (0.5
below the feet, 2.3 above, plus the tick's movement) through the whole
following half second. And the other direction: a long fall and fast flight
still get the full 1.75.

No data format changes. No switch: this puts back what ran until 0.10.40 for
everything a player on foot does.

### Step 2, 0.10.47: a Skyrim seat is not something E uses, and the log says why a press did nothing

- A crosshair target whose base record is furniture (type 0x28), has no
  workbench (bench type 0 at +0xE0 of the record) and is not a bed (bit 31 of
  the marker flags at +0xE4) is a seat: a chair, stool, bench, throne, also
  the wood-chopping block and the like. Skyrim's side reports it to Minecraft
  as "nothing to use" (kind 0), so E opens the Minecraft inventory as it does
  on empty air, and it refuses to activate one when asked (the R key does not
  look at the kind). Workbenches open a menu and beds open the sleep menu;
  both stay usable. No new number in the packet, so no new protocol.
- For every press that is handed to Skyrim, one more log line:
  `SCENE_INTERACT_TARGET` with the target's form ID, base form ID and type,
  whether it is locked and its lock level (TESObjectREFR::GetLock, ID 20223,
  read only; "unknown" if the address is missing), and the sit state of
  Skyrim's character.
- `SCENE_SEAT` whenever that sit state changes while the bridge moves the
  character (bits 14 to 17 of the state word at +0xC8; 0 standing, 1 to 4
  sitting down, seated or on a horse, standing up, 5 to 8 the same for
  sleeping).
- Switch `seat_guard` (number 14). Tests: the seat rule; the sit-state bits.

### Step 3, 0.10.48: a seated character is stood up

While the bridge is moving Skyrim's character and its sit state has not been
"standing" for 4 seconds on end, call Skyrim's own "stop using this, now"
routine (Actor::StopInteractingQuick, ID 38697, with `true`), log the state
before and after, and try again every 2 seconds, five times at most. The 4
seconds leave time for a workbench's sit-down before its menu opens (an open
menu stops the bridge moving the character, which restarts the count).

This is what makes a save taken while seated playable again, and it also ends
a ride: E on a horse mounts it, and the bridge cannot ride.

Experimental: the routine has not been called by this mod before. Switch
`stand_up` (number 15). Test: the timing rule as a pure function.

### Not in these steps

- Sitting, riding, mining and wood chopping from Minecraft. They need the
  bridge to leave the character where Skyrim put it, like a menu does, and a
  key that ends it. Backlog.
- The arrow that is not seen (user, letter F). Backlog, as before.
- Indoors the collision scan hardly ever turns calm: in the meadery four of
  five scans ran at full rate although the player stood still and the number
  of boxes did not change (2,708 each time), so their checksum must differ by
  some small moving thing. Costs time, breaks nothing. Backlog.

### Order of work and checks

Step 1, step 2, step 3; each built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (Rust tests and the
Minecraft-side checks run first). Then a review pass and a test list at the
top of README.md. Version in four places per step; protocol stays 31.

## Third batch result, 2026-10-11: 0.10.37 to 0.10.45 built, not played

Everything in the plan below is built. Before the last build: 45 Rust tests,
99 Minecraft-side rule checks, 136 hook checks. Install 0.10.45; the test list
is at the top of README.md.

What changed from the plan while working:
- Added 0.10.41, a longer look-ahead for the collision cube at high speed
  (2.75 blocks sideways, 1.75 up or down; was 1.5 and 0.75). The plan's step 1
  fixed the pause the logs showed; this is the real "8 block limit" case
  (flying at 20 blocks a second, a long fall), and it became testable once
  0.10.40 had turned the centre calculation into a pure function. The later
  steps moved up one number each.
- The review found nothing to correct in the mod; the report checker was
  extended instead.

Decisions the agent made that the user may want to overrule (each has a
switch):
- The local server no longer checks the player's steps against Skyrim's
  shapes; only the client does (0.10.38, `server_trust`).
- Skyrim's shapes never force a crouch or a crawl, so under a low Skyrim
  ceiling the player stands in it (0.10.39, `pose_free`).
- A shot is a straight line from the crosshair with no flight time and no
  drop, and it can hit friendly characters (0.10.44, `ranged`).

Argued from code, not seen in a log: the cause of "blocked, very slow, cannot
jump". Unverified game behaviour this batch rests on: that outdoor cells of
one world share a physics world (0.10.37; harmless if not), Skyrim's
favourites handler and player controls taking a forwarded key (0.10.43,
0.10.45).

## Third batch plan, 2026-10-11: two movement fixes, an optimisation, three features (0.10.37 to 0.10.45)

User, after the first play of 0.10.36: "Continue with optimizations. Also when
I'm moving fast the 8 block limit seems to block me and there is a small pause
in movement before it resyncs again. Sometimes something also seems to block
me, I can move out of whatever it is blocking me but the movement is very slow
and I cannot jump. After that design more features and implement them. Give me
another comprehensive test run to test." Written before the code, by Claude
Code. One step, one version, one MODLOG entry; a switch for every step that
rests on game behaviour nobody has seen.

### What the logs and the games' code say about the two problems

**The pause when moving fast is not the 8-block limit.** In report
20261011-155446 every short hold while moving ("no_snapshot", about 0.1 s) sits
on the same second as a change of Skyrim cell. Skyrim's outdoor world is cut
into cells of 4,096 units, 58.5 blocks. On every change of cell the bridge let
go of Skyrim's character, sent Minecraft an empty collision packet, worked out
the same origin again and started a new collision scan; Minecraft held the
player until that scan was done. Walking along a border did it over and over
(cells 9638 and 9639 alternate five times in ten seconds). The collision cube
itself was left only once in the session, by 0.05 block, while falling.

**"Slow and cannot jump" was not reproduced in the logs,** so the cause is
argued from Minecraft's code (26.3, read with javap), not measured:
- Minecraft's local server checks every step the player reports. If the new
  position touches a collision shape that the old position did not touch, it
  sends the player back (ServerGamePacketListenerImpl.handleMovePlayer,
  isEntityCollidingWithAnythingNew; no log line, also in creative mode). In
  survival it also sends the player back when its own replay of the step ends
  somewhere else ("moved wrongly"). Skyrim's shapes are part of those checks.
  They change under the player: characters get a stand-in box that jumps to a
  new place seven times a second, doors swing. Each change can put a shape
  where the player just stepped, and the server undoes the step. Undone steps
  and undone jumps are what "very slow and cannot jump" would feel like.
- Minecraft also forces a crouch or a crawl when the standing player no longer
  fits. A stand-in box overlapping the upper body does that. Crawling is slow,
  and the box above stops a jump.
- Every character near the player gets a stand-in box, dead ones included. A
  body on the ground is an invisible pillar 1.8 blocks high.
- The bridge's own server-side "hold" drops the player's steps on the server
  while the client keeps moving, which sets up exactly such a correction.

### 0.10.37: no pause at Skyrim cell borders

Skyrim side. A change of cell keeps the character steered when three things
hold: the bridge was steering, the new cell belongs to the same outdoor world
as the old one, and the character is where Minecraft says it should be (within
140 units, the tolerance already used when a session starts). Otherwise it lets
go as before (doors, loads, fast travel). The collision scan restarts only when
Skyrim's physics world really changed, not on every cell. Two rules that asked
for "the same cell" now ask for "the same cell, or the same outdoor world":
which characters get a stand-in, and which enemy a hit may land on (one hit was
lost to that rule in the test session). The decision is a pure function with a
test; the simulated-player test gains a border crossing.

### 0.10.38: the local server trusts the client about Skyrim's shapes

Minecraft side. While the local server checks one of the player's reported
steps, Skyrim's shapes are left out, so the server's answer is always "that is
where you are". Minecraft's own blocks are still checked. The server no longer
holds or drops the player's steps; holding is done by the client alone, which
is the side that moves. Switch: `server_trust=off`.

### 0.10.39: Skyrim's shapes never force a crouch or crawl; bodies are not solid

Skyrim has no place a standing character must duck through, so a forced crouch
or crawl is always wrong here. While Minecraft asks "does the player fit
standing", Skyrim's shapes are left out (both sides). Sneaking with the key is
unchanged. Skyrim side: dead characters get no stand-in. New log line once a
second while Skyrim shapes overlap the standing player (how many terrain tiles,
how many characters, which pose), so the next report shows this case instead of
leaving it to argument. Switch: `pose_free=off`.

### 0.10.40: standing still costs a quarter (optimisation)

Measured: Skyrim's collision ray casting takes about 1.1 ms of every frame,
moving or not. New: when a finished scan has the same centre and the same
boxes (checksum) as the one before, the next scan is cast at a quarter of the
rate. While that slow scan runs, the last boxes are sent again every 125 ms
with fresh characters, health, hits and crosshair, so nothing Minecraft relies
on gets older than today. As soon as the player's half-block cell changes, or a
slow scan finds different boxes (a door moved), scanning returns to full rate
at once. Expected: about 0.3 ms per frame while standing, unchanged while
moving. A door that moves while the player stands still is seen after about
half a second instead of an eighth. Pure pacing function with a test. Switch:
`slow_scan=off`.

### 0.10.41: where Minecraft's half millisecond goes

The Minecraft cost line gains four parts (sending, receiving and collision,
hand and HUD capture, block capture), so the next report shows which part is
worth looking at. No behaviour change.

### 0.10.42: more Skyrim menus on the proven path

Tab, M and J worked in play. Same handler, same route: I opens Skyrim's items,
P its magic; G opens Skyrim's favourites (its own handler, not seen working
yet). Minecraft's "social interactions" action shares P and only prints a
notice in single-player; it is dropped in a bridge world while it shares the
key. New command numbers 15 to 17.

### 0.10.43: bows hit Skyrim enemies

Today an arrow flies through an empty Minecraft world and hits nothing. New:
when the player releases a bow with an arrow available, Minecraft sends a
"shot" with its strength. Skyrim finds what the shot hits by geometry it
already has: the living characters near the player, the line from the player's
eye along the view, and one ray against walls and terrain. The nearest
character within 0.45 block of that line, up to 60 blocks away and not behind
a wall, takes the damage through the same call a sword hit uses.
- Damage follows Minecraft's arrow: 6 at full draw, less in proportion when
  drawn less, a quarter more at full draw (Minecraft's critical arrow), half a
  point per Power level plus a half; times the 1.75 scale. Nothing below a
  tenth of a full draw.
- No arrow is seen flying and it does not drop: the hit lands where the
  crosshair points when the string is released. Minecraft uses up the arrow as
  usual.
- Not in this step: crossbows, tridents, snowballs, an arrow you can see.
Data: the attack record already carries a target handle and a damage amount;
a shot uses a reserved handle value. No layout change; an older bridge would
ignore the value as "no target". Aiming maths is a pure function with tests.
Switch: `ranged=off`.

### 0.10.44: Skyrim's shout or power on a key (experimental)

Key Z sends Skyrim's own "Shout" key press and release to Skyrim's player
controls, the same way menu keys go to its menu controls (PlayerControls,
Address Library ID 400864, from CommonLib). A tap is the shortest shout or the
equipped power. Nothing about this route has been seen working. Switch:
`shout=off`.

### 0.10.45: review

Re-read the whole batch; corrections, if any, as their own entry.

### Not in this batch

A taller Minecraft world for building at any height (needs the user's
decision), swimming, Skyrim loot in the Minecraft inventory, an optimisation of
the scan while moving (reusing the part of the cube already scanned).

## Second batch result, 2026-10-11: 0.10.28 to 0.10.36 built, not played

All eight steps of the plan below are built, plus one correction from the
re-read (0.10.36: Skyrim logs the switches at load). Together with the first
batch that is everything from 0.10.19 to 0.10.36 waiting for one play session.
Before the last build: 39 Rust tests, 90 Minecraft-side rule checks, 124 hook
checks. Install 0.10.36; the test list (letters A to M, with the switch that
turns each risky step off) is at the top of README.md.

Two things changed from the plan while working:
- The hook check (every hook into Minecraft verified against the Minecraft
  jar) was not planned. It was added in 0.10.33 after an experiment showed the
  build accepts a misspelt hook target, which would stop Minecraft at start.
- No optimisation of the collision packet path: measured at about 80
  microseconds a frame, it is not worth touching a proven path.

Decisions the agent made in this batch that the user may want to overrule:
- Skyrim's natural recovery is not forwarded as healing; potions, food and
  healing magic are (0.10.30).
- Pointer speed is tied to Minecraft's Mouse Sensitivity setting, which also
  sets the camera speed (0.10.32).
- In a bridge world the player cannot fall out of the world at all (0.10.33).

Unverified game behaviour this batch rests on: Skyrim calling its health
function for a potion drunk at full health (0.10.30).

## Second batch plan, 2026-10-11: 0.10.28 to 0.10.35

User, before testing the first batch: "Continue working. I haven't tested
anything. Plan ahead once more and implement those designs. Give me a
comprehensive testing list afterwards." Written before the code, by Claude
Code. Same rules as the first batch: one step, one version, one MODLOG entry.

What decides the choice of steps this time: sixteen steps will then be waiting
for one play session, and every round trip to the user is expensive. So the
batch starts with two steps that lower the cost of a failure (switches, and
checks for the Minecraft side), adds features only on paths that are proven or
that the logs already show, and replaces guessing about speed with measurement.

### 0.10.28: switches, so one step can be turned off without a rebuild

A text file the user can create, `switches.txt` in the project folder (never
committed; no file means everything is on). One `name=off` per line, `#` for
comments. Both games read it once at start through the bridge library, which
already knows the project folder (it writes its logs there), and log what is
off. Names: `e_precedence` (E goes back to containers and bodies only),
`hostility` (Skyrim is never asked whether a character is an enemy),
`menu_keys` (Tab, M, J, wait), `damage_pool` (hits delivered one by one as
before), `pause_shield`, `pointer_monitor` (pointer measured against the window
as before), and for this batch `healing`, `menu_wasd`, `void_guard`.
The parser is a pure function with a test. Minecraft asks the library for the
set through one new entry point; nothing is added to the packets.

Why: if one of the unverified steps misbehaves or crashes a game during the
test, the user can turn that one off and carry on with the rest of the list.

### 0.10.29: checks for the Minecraft side, run before every build

The Minecraft side has no automated tests, and 0.10.21 shipped a wrong rule
because of it. The rules that are plain arithmetic or yes/no decisions move
into one small file with no Minecraft code in it (BridgeRules.java): attack
damage with recharge, critical, Sharpness, Strength and Weakness; what E does
for each crosshair kind; when a pooled hit may be delivered; which arrival
heights are refused, allowed without building, or normal; what the pointer is
measured against. The mod calls these functions, and a check program
(minecraft\diagnostics\RulesCheck.java) runs worked examples against them.
New script tools\test-java-rules.ps1; tools\build-step5b.ps1 runs it after the
Rust tests and stops on a failure.

Limit, stated plainly: this checks the bridge's own arithmetic. It cannot check
an assumption about how Minecraft behaves, which is what was wrong in 0.10.21.

### 0.10.30: Skyrim healing heals your Minecraft hearts

Today Skyrim potions, food and healing spells do nothing useful: Minecraft owns
the health and never hears of them. The logs already show the path: the same
Skyrim function that reports damage as a negative health change reports
restored health as a positive one (report 20261011-094530, natural recovery at
0.008 per frame while Skyrim's health was below full).

- **Skyrim:** a positive health change while Minecraft is taking hits is
  queued like a hit, as "restore this fraction of full health". Skyrim's own
  slow natural recovery is filtered out, because Minecraft has its own
  (food): a single restore of at least 0.5% of full health is forwarded at
  once (potions, food); smaller ones are added up for a quarter second and
  forwarded only if they come faster than 2.5% of full health per second
  (healing spells do; natural recovery, about 0.7% per second, does not).
  Skyrim's own health is left exactly as the game computes it.
- **Minecraft:** heals the player by that fraction of maximum health, at once,
  outside the damage pool.

Data: the damage list in the world packet already carries "fraction" and "raw
amount" per event. A restore travels as an event with a negative raw amount.
No size change, but an older bridge rejects a negative amount, so the protocol
number goes to 31 (name link31). Checklist: lib.rs, windows.rs, the range
checks in world.rs and BridgeDamage.java, tests for the new value.

Unknown: whether Skyrim calls that function for a potion drunk at full Skyrim
health (which is the normal case here, since Skyrim's health is kept full).
If it does not, nothing is forwarded and the log shows no SCENE_HEAL line.

### 0.10.31: W, A, S, D and held directions in Skyrim menus

In Skyrim's own window W, A, S and D move through menus. Here only the arrow
keys and the wheel did. While a Skyrim menu is open, the Minecraft movement
keys now send the same four directions, and a direction that is held repeats
(after 0.4 s, then every 0.12 s) so long lists can be scrolled. A direction
only repeats if it was pressed while that menu was open, so walking into a
conversation with W held does not scroll it. Minecraft side only.

### 0.10.32: pointer speed follows Minecraft's mouse sensitivity

0.10.24 fixed the pointer at one Skyrim pixel per mouse unit and left no way
to change it. The travel is now multiplied by twice Minecraft's Mouse
Sensitivity setting (Options > Controls > Mouse Settings): 100% keeps
0.10.24's speed, 200% doubles it, 50% halves it, never below a quarter.
Minecraft side only.

### 0.10.33: no "fell out of the world" in the bridge world; any depth

0.10.25 stopped arrivals 56 blocks below Minecraft's floor because Minecraft
hurts anything more than 64 below it. In a bridge world that rule protects
nothing: the ground is Skyrim's, and the player is held in place whenever
Skyrim's ground is not known. So in a bridge world only, the player is exempt
from that damage (one small hook on the function that applies it, read from
the 26.3 code: LivingEntity.onBelowWorld), and arrival is allowed down to
2,000 blocks below the floor as it already is above the ceiling. Deep water,
deep caves and low-lying interiors then work like anywhere else. Building is
still limited to Minecraft's own range.

### 0.10.34: measure what the bridge costs per frame

Offline measurement for this plan: handing Minecraft the collision packet
costs about 80 microseconds per frame (1,500 boxes, release build), about 1%
of a frame, so that path is left alone. What is not known is the cost inside
the games. Both sides now time their own per-frame bridge work and log one line
every five seconds (frames, average and worst microseconds); Skyrim's
collision scan also logs the time its ray casts took. The next report then
answers "does the bridge slow the games" with numbers, and shows where an
optimisation would pay.

### 0.10.35: a checker for the test report

New script tools\check-test-report.ps1: reads the newest collected report and
prints, for each feature on the test list, what the logs show (counts and the
first line of evidence), and lists error lines. For whichever agent reads the
result, and readable by the user.

### After this batch (not planned in detail)

Candidates, most useful first as far as can be judged without play: ranged
attacks (bows) on Skyrim enemies; swimming in Skyrim water; a taller Minecraft
world so building works at every height (needs the user's decision); Skyrim
loot into the Minecraft inventory; Skyrim characters colliding with placed
blocks; fluids contained by Skyrim terrain.

## Batch result, 2026-10-11: 0.10.19 to 0.10.27 built, not played

All eight steps of the plan below are built, plus one correction found by
re-reading the batch afterwards (0.10.27: the damage pool of 0.10.21 watched
the wrong Minecraft counter and would have changed nothing). 33 Rust tests
pass. Install 0.10.27; the combined test list is at the top of README.md.

Decisions the agent made that the user may want to overrule, each revertible
on its own:
- E on an enemy opens the Minecraft inventory; E on a friendly character talks
  (0.10.19). R is unchanged.
- A paused Minecraft makes Skyrim's character unhurtable (0.10.22).
- Pooled Skyrim hits all count in full, at most two deliveries a second
  (0.10.21).
- Pointer speed is one Skyrim pixel per mouse unit, slower than before
  (0.10.24).
- Arrival is allowed outside the building range; building is not (0.10.25).

Unverified facts the batch rests on (the first play will show): Skyrim's
menu-opening handler accepts a forwarded key event (0.10.20); the game's
hostility test answers as expected (0.10.19); a paused Minecraft keeps drawing
frames (0.10.22).

## Batch plan, 2026-10-11: eight steps, 0.10.19 to 0.10.26

User, after accepting 0.10.18: "Plan ahead on making new features or
optimisations. Continue working on all of these and give me another test run
once complete." This plan was written before any of the code. Each step is its
own version with its own MODLOG entry, so one can be reverted alone. None can
be played before the next is written, so each uses paths already proven in
game where it can, and says where it cannot. One combined test list goes at the
top of README.md at the end. Written by Claude Code.

Order: the step that changes the protocol number comes first, then features,
then the optimisation.

### 0.10.19: Skyrim's E takes precedence

The user's rule: "Only open the minecraft inventory when E won't do anything in
skyrim." So E (the Minecraft inventory key) now activates whatever Skyrim's
crosshair is on: a character to talk to, a door, a container, a body, a plant,
an item, a lever. With nothing under the crosshair it opens the Minecraft
inventory as before. Inside a Skyrim menu it stays Skyrim's confirm.

The one case that needs new information: an enemy. Skyrim's own E does nothing
to an enemy in a fight, and in a fight the crosshair is usually on one, so E
must still open the Minecraft inventory there. Skyrim therefore reports one more
crosshair kind, 6 "living and hostile to the player", using the game's own test
(CommonLib Actor::IsHostileToActor, Address Library ID 37537; called with the
looked-up character and the player, about seven times a second, only when the
crosshair is on a living character). If that address is missing the kind stays 3
and E talks.

R is unchanged: it activates anything, and takes all inside a container.

Data: crosshair kind gains value 6. Layout and size are unchanged, but an older
copy of the bridge would reject 6 as invalid, so the protocol number goes to 30
(name link30) and the two cannot be mixed. Checklist: lib.rs, windows.rs,
world.rs maximum and test, attack.rs classification and test, NativeLink
comment, SkyrimAttack rule.

Logs: SCENE_CROSSHAIR shows kind=6 for an enemy; SCENE_INTERACT_SEND already
names the key.

Unknown: whether IsHostileToActor answers as expected for every kind of enemy
(animals, summoned creatures, a guard after a crime). Not tested.

### 0.10.20: keys that open Skyrim's own menus

Today the only way to open Skyrim's character menu, map or journal is to click
into Skyrim's window. New remappable keys in Minecraft (Controls > Key Binds >
Miscellaneous):
- Tab: Skyrim's character menu (skills, magic, items, map).
- M: Skyrim's map.
- J: Skyrim's journal (quests, and the system page with save and load).
- "Skyrim: wait": no key by default.
Inside any Skyrim menu, Tab goes back, as it does in Skyrim.

Same path as F5/F9, which is proven: one key event with Skyrim's own user event
name ("Tween Menu", "Quick Map", "Journal", "Wait", from CommonLib
UserEvents.h) sent to MenuControls, where Skyrim's MenuOpenHandler lives beside
the quicksave handler. Same guards as quicksave: fresh active frame, loaded
game, no Skyrim menu already open, the paired save, at most 250 ms old, one
second apart. New command numbers 11 to 14, context 0. No layout change.

Unknown: whether MenuOpenHandler acts on a forwarded event as the quicksave
handler does. The log shows dispatch only; the user sees the menu or not.

### 0.10.21: continuous Skyrim damage counts

Measured (report 20261011-094530): of 99 Skyrim hits under 2 damage, Minecraft
took 7, because Minecraft refuses a hit within half a second of the last one
unless it is larger. Fire, frost, poison and burning arrive as many tiny hits.

Change, Minecraft side only: forwarded hits are added to a pool. The pool is
delivered as one hit whenever Minecraft's half-second hit pause is over
(invulnerable time of 10 ticks or less), then emptied whatever Minecraft does
with it (armour, shield, creative mode). Nothing is delivered during the pause,
so nothing is refused. Effect: at most two Skyrim hits a second reach
Minecraft, each carrying everything received since the last, so the total
matches what Skyrim dealt. A single sword blow is delivered at once when no
pause is running, otherwise up to half a second late.

Difference to know: Minecraft's own rule lets a second mob hit inside the pause
count only for the part that exceeds the first. With the pool, hits from two
Skyrim enemies in the same half second both count in full. That is how Skyrim
itself treats them.

Logs: SCENE_DAMAGE_POOL per hit added; SCENE_DAMAGE_APPLY per delivery with the
number of hits and their sum.

### 0.10.22: protection while Minecraft is paused

Measured (same report): while Minecraft was paused or not the active window,
113 hits took 52.6 health from Skyrim's character directly, and 22 hits waiting
for delivery were thrown away.

Choice made (the user asked for all candidates to be worked on; this one is a
gameplay decision and can be reverted alone): a paused Minecraft protects the
Skyrim character, as pausing protects a Minecraft player. While the same
Minecraft session and world that was steering the character keeps sending
"not active" frames, Skyrim hits on the player are cancelled and logged, not
queued. Hits already waiting are kept instead of dropped, and still expire after
five seconds, so a short pause loses nothing and a long one delivers nothing
late. Protection ends half a second after the frames stop (Minecraft closed,
crashed, minimised without drawing, or in another world), and Skyrim's own
damage applies again.

Skyrim side only. No layout change: Skyrim reads the "not active" frame it used
to ignore. The decision (deliver, protect, pass through) becomes a pure
function with a test.

Logs: SCENE_DAMAGE_SHIELD state=on/off, and one line per cancelled hit.

### 0.10.23: Sharpness, Strength and Weakness

Outgoing damage today is the held item's attack damage only. Add, Minecraft
side only, using Minecraft's own numbers:
- Sharpness: plus 0.5 per level plus 0.5 (level 1 gives 1, level 5 gives 3).
- Strength effect: plus 3 per level. Weakness: minus 4 per level.
The sum is floored at zero, then the existing scale (x1.75), recharge and
critical rules apply. Smite, Bane of Arthropods and Impaling are left out:
Minecraft ties them to kinds of Minecraft creature. Logged in SCENE_ATTACK_SEND.

### 0.10.24: pointer speed no longer depends on the Minecraft window size

Backlog item. Pointer travel is "fraction of the Minecraft window", so the
three window sizes the user has played with (854, 682 and 372 wide) gave three
speeds: about 3, 3.8 and 6.9 Skyrim pixels per mouse unit. Change, Minecraft
side only: divide by the size of the monitor the window is on instead of the
window. With Skyrim at the monitor's resolution that is one Skyrim pixel per
mouse unit, the same as a desktop pointer, whatever the window size. The
pointer will feel slower than the user is used to; a full-screen Minecraft
keeps exactly its present speed. If the monitor cannot be read the window size
is used as before.

### 0.10.25: arriving at any height

The only place the height limit is enforced is the teleport after a door, a
load or fast travel: a target outside Minecraft's building range (y -64 to 319,
Skyrim heights about -11,600 to 15,000 at home) is refused. Walking was never
limited, because the ground comes from Skyrim's collision, not from Minecraft
blocks, and a Minecraft player can stand above the building limit.

Change, Minecraft side only: accept targets from 56 blocks below the world
floor (Minecraft only starts "fell out of the world" damage 64 below it) up to
2,000 blocks above the ceiling. At home that is Skyrim heights of about -15,500
to 155,000: sea level (-14,000) and every mountain. What does not change:
blocks can only be placed and kept inside Minecraft's own range, and Minecraft
says so itself when you try. Diving deeper than about Skyrim -16,000 would take
Minecraft void damage; not guarded in this step.

The larger answer (a taller Minecraft world so building works everywhere) is a
world-format change and is left in the Backlog for the user to decide.

### 0.10.26: the bridge's own log is far too heavy (optimisation)

Measured from logs/step2 on 2026-10-11: the folder holds 34 GB; one session
wrote 2.6 GB per game. In the last session 1,678,181 of 1,724,764 Skyrim lines
(97%) were BOX_SEND: every collision box of every snapshot as its own line,
about 7,000 lines a second, written on Skyrim's player-update thread and
mirrored as BOX_RECV on Minecraft's render thread. Each line is also written in
about seven separate system calls because the file is unbuffered.

Change:
- Each log call is written with one system call.
- A snapshot logs one WORLD line with the box and character counts and a
  checksum of each, the same on both sides, so "what Skyrim sent is what
  Minecraft received" can still be proven. The per-box and per-character lines
  are written only when the environment variable PASSTHROUGH_LOG_BOXES=1 is set
  before starting the games.
- New script tools\prune-logs.ps1 for the user to run: lists what it would
  delete, and with -Delete removes all but the newest sessions' raw logs.
  Nothing is deleted by the agent.
A test proves the default output has no per-box lines and that the checksum
changes when a box changes.

Expected effect: Skyrim and Minecraft main threads each stop making tens of
thousands of file writes a second; raw logs shrink by roughly 97%. Frame-rate
gain is not measured and cannot be from here.

### Not in this batch

- A taller Minecraft world for building at any height (Backlog; needs the
  user's decision and a backup of the bridge world).
- Moving items between the two inventories, shared time and weather, fluid
  containment, NPC collision with placed blocks.

## Skyrim quicksave and quickload keys (0.10.7)

User request while 0.10.6 was being built: F5 should do Skyrim's quicksave and
F9 Skyrim's quickload, pressed in Minecraft. Kept as its own step and version
so it can be reverted separately; both steps install together. This section was
written alongside the code, not before it (a small step, noted for honesty).

- **Minecraft:** two remappable keys under Controls > Key Binds >
  Miscellaneous, defaults F5 and F9. One command per fresh press, only while
  the bridge is sending active frames, Minecraft is focused with no screen
  open, and no Skyrim menu is reported. F5 is also Minecraft's "toggle
  perspective": in a bridge world, while the two share a key, the perspective
  toggle is dropped before Minecraft reads it (26.3 gives a key press to every
  action bound to it), so one press does not also change the camera. Rebind
  either action to get the perspective toggle back.
- **Skyrim:** the existing UI task sends one ordinary key event with Skyrim's
  own user event "Quicksave" or "Quickload" (CommonLib UserEvents.h) to
  MenuControls, where Skyrim's QuickSaveLoadHandler lives. Skyrim's own rules
  decide whether a save or load happens; no save or load function is called
  directly and no save file is touched by the bridge.

Data: reuses the menu command slot. New command numbers 7 (quicksave) and 8
(quickload) travel with context 0. Skyrim accepts them only with a fresh active
frame, a loaded game, no blocking Skyrim menu, the paired save lineage loaded,
at most 250 ms old, and at least one second after the previous one. Other
cases are logged with a reason and dropped. No wire layout change; protocol
stays 28.

After a quickload the existing chain applies: load reset, position mismatch,
recovery teleport of the Minecraft player. Minecraft's world does not roll back
and is not saved by F5; only Skyrim's quicksave is made.

Unknowns: whether Skyrim's handler acts on a forwarded "Quicksave"/"Quickload"
event outside its own input loop. Logs show dispatch and return only; the user
must see Skyrim's own "Quicksaving..." message or the load screen.

## Real left click and the E key for containers (0.10.17 and 0.10.18 plan)

User accepted 0.10.16, confirmed that R inside a container takes everything,
and reported that the experimental middle-button click "acts like left mouse in
Skyrim". They chose the next step (make left click a real Skyrim click) and
asked to add: pressing E opens the Skyrim container. Written by Claude Code.

### 0.10.17: left click becomes a real Skyrim click

In a Skyrim menu, left click now sends the true click (command 10) that the
middle button proved, instead of the confirm key. Skyrim then acts on whatever
is under the pointer, as it does in its own window: list entries, map markers,
buttons. Enter stays the confirm key for the highlighted entry. The optional
middle-button key keeps working and now does the same thing. Minecraft side
only, one line; no wire change. Difference to know: a left click with the
pointer over nothing no longer confirms the highlighted entry; use Enter or E.

### 0.10.18: E opens Skyrim containers

Goal: E, the key that opens the Minecraft inventory, should open the Skyrim
container you are looking at, and act inside it as E does in Skyrim.
- Looking at a Skyrim container or a dead body within reach: E activates it,
  exactly as R does, and the Minecraft inventory does not open.
- Inside any open Skyrim menu: E is Skyrim's confirm (take the highlighted
  item, pick the highlighted choice), instead of opening the Minecraft
  inventory on top of the Skyrim menu.
- Anything else, including looking at a living character or a door: E opens the
  Minecraft inventory as always. R remains the key that activates anything.
The rule follows the key bound to "Open/Close Inventory" in Minecraft, whatever
key that is.

Why not "E activates whatever Skyrim's crosshair is on": in a fight the
crosshair is usually on an enemy, and E must still open the Minecraft inventory
to eat or switch items. So Minecraft has to know what kind of thing the
crosshair is on, which it does not today (it only receives an opaque handle).

The two halves:
- **Skyrim** classifies its crosshair target each time it publishes a
  collision snapshot: nothing, container, door, living character, dead
  character, or other object. It looks the handle up with the same
  reference-counted lookup the attack code uses and reads the record types from
  CommonLib's documented fields (reference type at +0x1A, base object at +0x40,
  base type at +0x1A; container 0x1C, door 0x1D, character reference 0x3E; dead
  from the life state confirmed in 0.10.8 or health not above zero).
- **Minecraft** remembers that kind with the handle, for the same quarter
  second, and decides what E does before Minecraft itself reads the key.

Wire change: the world packet grows by 8 bytes, inserted just before the
damage section so every end-relative offset stays valid: u32 target kind and u32
reserved at 397,440. New size 398,024, magic PTWORL13, protocol 29, namespace
link29. The kind is validated against a maximum like the hold reason, with a
test, and the classification is a pure function with a test. All three files
update together.

Logs: Skyrim SCENE_CROSSHAIR when handle or kind changes; Minecraft
SCENE_INTERACT_SEND gains source=interact_key or inventory_key; menu confirms
from E are logged with source=inventory_key.

Not in this step: opening doors or talking with E, Skyrim's own "E to take"
prompt text, moving items into the Minecraft inventory.

Unknowns: whether the base-object and form-type reads are right for every
container, and whether bodies report as dead by these fields. The log line
shows the kind for whatever the user looks at.

## Work done while the user was away, 2026-10-11 (0.10.10 to 0.10.16)

User tested 0.10.9 (doors work; blocks invisible after a door), then asked for
the fix and further gameplay features without test requests until they return,
with one complete test list at the end. Each item below is its own step and
version so it can be reverted alone. None of them could be played before the
next was written, so each is built only on paths already proven in game, adds
unit tests where the logic is the bridge's own, and regressions are avoided by
leaving every accepted behaviour as it was. Written by Claude Code.

### 0.10.10 fix: a mesh from the previous place disabled block drawing

Report 20261011-012218: on stepping through a door the newest block mesh was
still the outdoor one. Mapped through the interior's origin it was too far to
express, the drawing code treated that as a fatal error and switched block
drawing off for the session ("SCENE_DISABLED message=invalid scene origin").
Doors, recovery, indoor collision and indoor placement all worked. Fix: a mesh
further than 64 blocks from the Minecraft player on any axis belongs to
another place and is skipped while waiting for the next one. A test replays
the logged numbers.

### 0.10.11 plan: more Skyrim menu controls from Minecraft

Goal: make containers, shops and Skyrim's other list menus usable without
switching windows. All while a Skyrim menu is open and Minecraft is focused:
- Mouse wheel up/down sends the existing Up/Down commands (it no longer changes
  the Minecraft hotbar slot during a Skyrim menu).
- Right click sends the existing Back command, once per press, and no longer
  uses the held Minecraft item during a Skyrim menu.
- The Interact key (R), which does nothing in menus today, sends Skyrim's own
  R-key menu action (user event "XButton": take all in a container, drop in the
  inventory). New command number 9.
- A new, optional key "Skyrim menu: click at pointer" (default middle mouse
  button) sends a true Skyrim mouse click (user event "Click", new command 10)
  at the pointer. Left click stays exactly as accepted (confirm). This key is
  an experiment to learn whether a forwarded click works where the clicked spot
  matters, such as the map; if it does, left click can switch to it later.
Wheel, right click and R reuse the dispatch path already accepted for arrows
and Backspace. The true click and the "XButton" name are unverified. No wire
layout change. The command numbering and accept/drop rules become a pure
function with a test covering every number.

### 0.10.12 plan: pointer speed remembered between menus

Backlog defect: each menu started from Skyrim's stored sensitivity (1) while
the measured value is 2, so the first small movements travelled double. Keep
the last measured pixels-per-count for the session and start each menu with
it. Pure helper functions with tests. Pointer speed scaling stays in the backlog.

### 0.10.13 plan: critical hits on Skyrim enemies

Minecraft's own rule: a full-strength hit while falling (not on the ground, not
climbing, not in water, not sprinting, not blinded, not riding) deals 1.5 times
damage. Apply the same test to bridge attacks, multiply the requested damage
by 1.5, play Minecraft's critical-hit sound, and log crit=true. Minecraft side
only; the damage scale (x1.75) and recharge rule are unchanged.

### 0.10.14 plan: weapons wear out

Minecraft's own rule: a weapon loses durability per hit as stated by its
"weapon" data (swords 1, axes 2), and breaks at zero. When a bridge attack is
sent, apply that wear to the held item on the integrated server through
Minecraft's normal hurtAndBreak, so Unbreaking, creative mode and the break
sound and animation behave as usual. Wear is applied when the attack is sent;
the bridge has no "hit landed" reply, so a swing Skyrim then drops (target
moved away) still costs durability. Bare hands and items without weapon data
are unaffected.

### 0.10.15 plan: Minecraft armour and shields against Skyrim hits

Finding while reading Minecraft's own data (tag files in the cached 26.3 jar):
the "generic" damage type used for Skyrim hits since 0.9.8 is listed under
"bypasses_armor", and through that under "bypasses_shield". So the earlier note
that Minecraft armour still applies was wrong: armour and shields have done
nothing against Skyrim hits. No test ever covered it (the accepted runs
matched the requested amounts exactly, which is what unarmoured play shows too).

Change, Minecraft side only: deliver each forwarded hit as Minecraft's ordinary
"mob attack" damage type, which armour, enchantments and shields all act on.
Give it a position to come from: the nearest Skyrim character in the current
collision snapshot. Minecraft's own shield rule then blocks hits you are facing
with a raised shield, wears the shield, and plays its sound. With no character
nearby the hit has no direction: armour still applies, a shield does not.

Side effect to know about: an ordinary mob attack also knocks the Minecraft
player back a little, away from the attacker, as any Minecraft hit does. The
"generic" type had no knockback. Skyrim's character follows the Minecraft
player, so being hit will now push you slightly.

Safety: if the damage type cannot be looked up for any reason, fall back to the
old "generic" hit and log it once, so damage forwarding itself cannot break.
The amount requested from Skyrim, the once-only delivery, the acknowledgement
and the death rule are unchanged. Logs add source, attacker position, armour
value and whether the player was blocking.

### 0.10.16: review correction

Re-reading the night's Minecraft-side code (it has no automated tests) found
one hazard: the held-button latches from 0.10.11 are released by per-frame code
inside the bridge sender, so a stopped sender could have left right click
swallowed. They now lapse after half a second without that per-frame code.
Nothing else was changed in this step.

### Result of the night's work

Built and unit-tested, none of it played: 0.10.10 fix, 0.10.11 menu controls,
0.10.12 pointer memory, 0.10.13 critical hits, 0.10.14 weapon wear, 0.10.15
armour and shields, 0.10.16 correction. Install 0.10.16; the combined test list
is at the top of README.md. Each step has its own MODLOG entry and can be
reverted alone, newest first.

## Doors and interiors: one Minecraft region per Skyrim place (0.10.9 plan)

User asked for doors and interiors, noting it also helps fast travel. Today the
bridge has one saved origin: the rule that turns a Skyrim position into a
Minecraft position, valid for one outdoor world (or one interior cell). Going
through a loading door puts Skyrim's player in another place with its own,
unrelated coordinates, so the bridge refuses it (hold reason 4) until the
player walks back out using Skyrim's own controls. Written by Claude Code.

A "place" here means: an outdoor world (Skyrim calls it a worldspace; the open
country is one, each walled city is another), or one interior cell (a building
or dungeon section behind a loading door).

Plan: give every place its own region of the one Minecraft bridge world.
- The place where the saved origin was made (the "home" place, here the open
  country) keeps that origin exactly as saved. Nothing already built moves.
- Every other place gets an origin worked out from its Skyrim record ID alone,
  by a fixed formula. Nothing new is stored in any save. This matters because
  Minecraft cannot roll back: if regions were handed out in order of first
  visit and stored in Skyrim's save, loading an older Skyrim save (which now
  also happens on every death) could hand an already-built-in region to a
  different building. A formula always gives the same building the same region.
- Blocks placed inside a building therefore stay with that building, and are
  not visible from outside it, just as Skyrim's own interiors are separate.

The formula (all in whole Minecraft blocks, far apart so places never touch):
- Interior cell: take the low 24 bits of the cell ID. Bits 0-11 pick a column
  and bits 12-23 a row on a grid of 2048-block squares centred on the world
  origin (column and row each minus 2048). Skyrim position (0,0,0) of that
  interior sits at the centre of its square at Minecraft height 128. Room for
  about 1000 blocks (70,000 Skyrim units) in every direction.
- Outdoor world other than home: bits 0-7 of the worldspace ID pick a column
  and bits 8-15 a row on a grid of 8192-block squares placed beside the interior
  grid (X from 4,718,592 upward, Z centred on zero). Skyrim (0,0,0) sits at the
  centre of its square at Minecraft height 136, which puts Skyrim's sea level
  (-14000) at the bottom of the Minecraft world.
- If a square would land within two outdoor squares (16,384 blocks) of the home
  origin in both directions, its row's top bit is flipped, moving it far away.
- The load-order byte of the ID is ignored, so reordering normal plugins does
  not move builds. Two places from different plugins with the same low bits
  would share a square; that is rare and accepted for now (logged, not
  detected). Light plugins keep their index inside the low bits, so their
  squares can move if the load order changes.

How travel then works, using only what already exists: after a door (or a
fast travel, or a load) Skyrim's player is somewhere whose Minecraft position
is far from where the Minecraft player stands. That is exactly the case the
accepted recovery handshake handles: Skyrim holds, asks Minecraft to teleport
to the matching position, and resumes once both Minecraft sides confirm.
Walking back out picks the home origin again and recovers the same way.

Code: saved_origin.rs gains "origin for this place" (home if it matches, else
the formula); save_identity.rs uses it in the three places that checked "same
location" (origin restore, recovery target, menu permission). No wire, save
record or protocol change (protocol stays 28). Minecraft: when a recovery
target lies outside the Minecraft world's height or area, say so in chat and
log it instead of holding silently; reword hold reason 5 to fit door travel.

Tests (run before the build): formula is stable, different places get
different squares, interiors and outdoor squares never overlap, home is kept
clear; the simulated player walks through a "door" (cell change), is held with
a recovery request for the formula position, and resumes after the simulated
Minecraft teleport.

Known limits, not solved here:
- Height. Minecraft has 384 blocks of height (about 26,900 Skyrim units).
  The home origin already cannot reach Skyrim's sea level or high mountains,
  and a fast travel to such a place will be refused with the new message. The
  "coordinate window" idea in the older plan below is still the answer and is
  a separate, larger piece of work.
- Fast travel needs Skyrim's map. The forwarded pointer and confirm key may
  not be enough to pick a map marker; a true click is still in the backlog.
- Collision sampling, NPCs, menus, attacks and death inside interiors use the
  same code as outdoors but have never run there.

Unknowns to settle from a real run: whether a door activated with R completes
while the bridge is steering the player, how the loading screen and cell
change appear to the bridge, whether Skyrim's collision world can be sampled
in an interior, whether Minecraft copes with a teleport of millions of blocks
and finds ground under the player, and the return trip. Attempt 1.

## Death: accepted (0.10.8)

User confirms that after a death the bridge reconnects and play resumes. Report
20261011-005940: death received 19 ms old, health 100 to -200, Skyrim reload
6.3 s later, hold cleared, recovery accepted 31 ms after the request, origin
ready; about 7.1 s from death to moving again; no error lines. The life-state
read is now confirmed: 0 when the lethal call returned, 1 (dying) on the next
frame, 2 (dead) 2.5 s later, 0 after the reload. It lags the hit by one frame,
so the hold stays on health; life state is usable for later features. Skyrim
keeps calling the player update while the player is dead.

## Death: first run result and repair (0.10.8)

Report 20261011-005252, first run of 0.10.7: the death number arrived 5 ms old,
the puppet was released, DoDamage(300, no attacker) took Skyrim health from 100
to -200, and Skyrim reloaded its last save 6.3 seconds later. So the kill itself
works. The bridge then never reconnected because of a bug in this change: the
world-packet check accepted hold reasons 0..6 only, the new "player dead" reply
used 7, the sender rejected its own reply ("invalid world snapshot"), and the
Skyrim adapter disabled itself as it does on any error. The old collector kept
only SCENE_ lines, so that error line was missing from the first report.

Repair, kept narrow:
- Hold reasons are named constants in world.rs with one maximum; reason 7 is
  accepted. A unit test encodes a hold reply for every reason.
- The simulated-player test in skse.rs is brought up to today's rules (paired
  lineage, cell, readable health) and now also plays a death: one kill, hold
  replies that must be accepted, a reload, and no second kill.
- Hold now depends on health at or below zero (plus the 3 second grace), not on
  the life-state bits: straight after the lethal hit they still read 0, so they
  are logged (SCENE_DEATH_LIFE) but not trusted yet.
- The collector also keeps ERROR, LINK, RELEASED, ANCHOR, load and hook lines
  and the first lines of a Minecraft "sender disabled" stack trace.
- Rust tests run before every build; build-step5b.ps1 stops if one fails.

This is the second build of the death feature. If the reconnect after a death
reload fails again, stop and write the handoff instead of a third variation.

## Death: Skyrim-style reload (0.10.6 plan, step 1)

User chose the rule on 2026-10-11: when the Minecraft player dies, the Skyrim
character dies too and Skyrim reloads its last save, as it normally does.
Minecraft cannot roll back, so placed blocks and the Minecraft inventory stay
as they are. Written by Claude Code.

What happens today (from code, not tested): Minecraft death makes the bridge
inactive; Skyrim's character stays alive under native control and can be hurt
directly; a Minecraft respawn is pulled back to Skyrim's position by the
save-position recovery step. Nothing kills the Skyrim character.

The two halves:
- **Minecraft** reports each death once and gets out of the way. A server-side
  hook on ServerPlayer.die (26.3 signature checked with javap) records a death
  number and time for marked bridge worlds. In bridge worlds two game rules are
  set at world load, like the existing spawn rules: immediate respawn on (no
  death screen, so no click in Minecraft's window is needed while Skyrim shows
  its own death) and keep inventory on (items and experience stay with the
  player; otherwise they would drop at the death spot while Skyrim reloads
  somewhere else). The respawned player waits at world spawn, held by the
  existing no-coverage movement hold.
- **Skyrim** carries out the death. On a fresh, unseen death number it first
  lets go of the puppet (restoring gravity and camera), then deals lethal
  damage to its own player with the same DoDamage function already verified on
  enemies (Address Library 37335), with no attacker and no difficulty
  adjustment. Skyrim's normal death sequence and automatic reload follow.
  Nothing in Skyrim's save or load code is called or changed.

After the reload the existing chain applies unchanged: the load resets the
bridge, saved-origin restore finds the position mismatch, and the recovery
handshake teleports the Minecraft player to the loaded Skyrim position.

Data crossing: Minecraft sends a death number (0 = none) and its age in
milliseconds with each active frame, for 10 seconds after the death. The native
sender converts the age to a time stamp. Skyrim acts once per sender/world/
number and only if the death is at most 10 seconds old; older ones are logged
as expired and ignored. A death number already handled is ignored after the
reload. No acknowledgement is needed.

Holding: while Skyrim's player is dying or dead (life state from CommonLib
ActorState, Actor+0xC0 on this runtime, state word at +0xC8, bits 21-24, with
health at or below zero as a cross-check), and for 3 seconds after a kill
attempt, Skyrim does not puppet, align, recover or export collision. It replies
with a new hold reason 7 so Minecraft shows "You died" and keeps holding. This
also stops the bridge from steering a corpse when Skyrim's player dies natively.
A living player with positive health is never held by this rule, so a wrong
life-state read cannot block normal play.

Damage hook: the incoming-damage clamp hook must not swallow the lethal hit.
The bridge is already released (not ready) at that point; an explicit flag
additionally makes the hook pass it through and log reason=bridge_death.

Failure handling: if the Skyrim player is still alive with positive health
after the grace period (god mode, a scripted protection), log state=survived
and let the normal flow continue, which returns the Minecraft player to
Skyrim's position alive. No retry and no second kill for the same death.

Wire change: connection record 440 -> 456 bytes: u64 death number at 440, u64
death time at 448. Protocol 28, namespace link28, pt_mc_frame14 (adds u64
death number, u32 age in ms). World packet layout unchanged; header word 124
gains value 7. All three files update together.

Logs: Minecraft SCENE_DEATH_SEND (number, cause, health, rules). Skyrim
SCENE_DEATH_RECV (number, age, decision), SCENE_DEATH_RESULT (returned, health
and life state before/after), SCENE_DEATH state=hold/clear/survived, and the
existing SCENE_RECOVERY lines after the reload.

Not in this step: Skyrim-native deaths do not kill or heal the Minecraft
player; no Minecraft death screen or message inside Skyrim's window; no item
drop; no rollback of Minecraft blocks or inventory; no protection for the
Skyrim character while the bridge is otherwise not ready. If the user turns
immediate respawn off and waits on the death screen longer than 10 seconds, the
death expires and the old "pulled back to Skyrim's position" behaviour applies.

Unknowns to settle from a real run: whether DoDamage on the player with no
attacker kills cleanly, whether life state changes inside the call, how long
Skyrim takes to reload in the background, and whether recovery completes after
that reload. Attempt 1. If the player does not die, the designed second attempt
is the documented Actor::KillImpl virtual; a second failure means a handoff.

## Outgoing damage balance (0.10.5 plan)

User reported that hits on Skyrim enemies feel far too strong and asked for the
deferred balance pass. Verified numbers so far: wooden sword full hit 40, fist
10, against a wolf with 22 health and a mudcrab with 5.

Change one number only: Skyrim damage = Minecraft attack damage x 1.75, instead
of x 10. Everything else stays: held-item damage calculation, recharge
multiplier 0.2+0.8*charge^2, range, target, expiry and spam guards, and Skyrim's
own difficulty setting applied on its side. Incoming damage (Skyrim hits on
Minecraft hearts) is not changed; it is already a share of maximum health and
was not part of the complaint.

Why 1.75: it lines Minecraft's sword tiers up with Skyrim's own one-handed
swords, so a Minecraft weapon hits about like the Skyrim weapon of the same
rank. These base values are quoted from memory of both games, not read from
game files: Minecraft swords wood 4, stone 5, iron 6, diamond 7, netherite 8;
Skyrim swords iron 7, steel 8, orcish 9, dwarven 10, elven 11, glass 12,
ebony 13, daedric 14.

| Minecraft full hit | Now (x10) | New (x1.75) | Nearest Skyrim sword |
| --- | --- | --- | --- |
| Fist (1) | 10 | 1.75 | weaker than Skyrim unarmed (4) |
| Wooden sword (4) | 40 | 7 | iron (7) |
| Stone sword (5) | 50 | 8.75 | steel 8 / orcish 9 |
| Iron sword (6) | 60 | 10.5 | dwarven 10 / elven 11 |
| Diamond sword (7) | 70 | 12.25 | glass (12) |
| Netherite sword (8) | 80 | 14 | daedric (14) |

Expected feel: a wolf (22) takes four full wooden-sword hits instead of one; a
mudcrab (5) takes one sword hit or three full punches. Rapid hits are weaker
still through the existing recharge rule. The fist becomes weak on purpose, as
it is in Minecraft; raising it separately is a later option if it feels useless.

Java-only change in SkyrimAttack (one named constant, logged as `scale`). No
wire change: protocol 27, record and FFI unchanged, native guards unchanged
(they already accept any finite damage above 0 up to 1000). Versions go to
0.10.5 so the installed files are identifiable; the DLL is rebuilt only because
the build requires matching versions. This is a balance choice, not a unit
conversion, and is expected to be tuned from play. Test: full hits with fist
and wooden sword on an enemy that survives; logs should show scale=1.75,
full_damage 1.75 and 7, and matching native health loss.

## Backlog (deferred by the user, not scheduled)

Repair follow-ups updated 2026-10-11 (0.10.54 played):
- Door exit without later teleport accepted; future update: face away from the
  exit door rather than somewhat toward it. Deferred, not implemented.
- Chair exits still fail at lower chair/table and high stool/NPC. .50 exit and
  .51 terrain correction unsuccessful. User explicitly reopened ONE more attempt
  2026-10-11 after .56 acceptance. Native height/camera and exact terrain/NPC cause
  unknown; diagnose first. Earlier failure handover remains in STATUS history.
- E/WASD stand request, held entering E, focus/menu seating and seated-save recovery
  accepted. Native hand visibility .54 not separately confirmed.
- Owned-bed grounding .52 accepted. Unowned-bed sleeping and workbench not tested.
- Potion healing .30/.53 unsuccessful; stopped after two attempts. Hook installs
  but no restore callbacks. Left-click item use failed while E works; contextual
  .55 selection failed:52/52 clicks chose a passive renderer (other/pointer),
  never item_use. .56 skips passive layers and adds container item route; all
  take/equip/use/category tests accepted by user. Potion healing reopened for ONE
  extra attempt 2026-10-11; native restore path diagnosis first. Stock mouse only;
  continuous damage remains deferred.
- Creative ascent/flight-off falling, flat-ground jumps, close enemy movement and
  bow wall/sky cases remain accepted. Do not repeat accepted checks.
- Indoor non-calm scans, visible flying arrow, riding, mining/chopping and other
  unsupported modes remain deferred; no request to reopen them.

Gathered 2026-10-11 from README, STATUS, MODLOG and this file. Items are
unfinished or untested things already recorded there, grouped by area. Nothing
here is planned in detail yet.

Combat and health:
- Death: a Minecraft death now kills the Skyrim character (0.10.6, untested).
  Still open: a Skyrim-only death does not reset Minecraft health, and no death
  message from Minecraft is shown in Skyrim's window.
- A Skyrim hit that lands while the bridge is not ready still lowers Skyrim
  health; protection while disconnected or recovering is undecided. A paused or
  unfocused Minecraft now protects the character (0.10.22; accepted
  2026-10-11: two pauses cancelled 5 hits in report 20261011-155446).
- Other incoming damage sources (falls, magic, scripted health changes) remain
  unverified. Potion healing failed in .30 and .53 gameplay: native restore hook
  installed but no positive requests. Stopped after two approaches; actual effect
  path at full native health unknown. Food/spells not tested; do not claim healing
  forwarding works from the synthetic fixture.
- Bows hit Skyrim enemies since 0.10.44 (draw/hits accepted in 0.10.45;
  wall/empty shots accepted in 0.10.50): a straight-line
  shot resolved in Skyrim by geometry. Open there: crossbows, tridents and
  thrown things; an arrow that is seen, flies and drops; creature sizes (every
  creature counts as a human-sized body).
- Skyrim's shout or power on a key since 0.10.45 (built, not played,
  experimental): a tap only. Open: holding for longer shouts; Skyrim's own
  spells in the hands.
- Not started, candidate for a later batch: swimming (Skyrim's water does not
  exist for Minecraft; the player walks on the sea floor).
- **Rapid small Skyrim hits are mostly lost (measured 2026-10-11).** Minecraft
  refuses a new hit for half a second after the last one unless it is larger.
  Skyrim's continuous damage (a flame or frost stream, burning, poison) arrives
  as dozens of tiny hits per second, each used up whether or not Minecraft took
  it. Report 20261011-094530: of 99 hits under 2 Skyrim damage, 7 were taken;
  12.76 half-hearts were requested and 0.69 applied. Ordinary weapon blows are
  fine (9 of 11 taken, the other 2 shield-blocked). Candidate fix: add up the
  refused amounts and deliver them with the next hit Minecraft will accept.
  Done as a pool delivered when Minecraft's hit pause is over (0.10.21,
  corrected in 0.10.27). Ordinary hits behave in report 20261011-155446 (21
  deliveries; every refusal was a raised shield). **Continuous damage itself is
  still not tested**, and no hit has yet arrived inside the hit pause in play.
- Critical hits (0.10.13), weapon wear (0.10.14) and armour/shields against
  Skyrim hits (0.10.15) are verified for a wooden sword, fists, 14 armour
  points and a shield. Axes and other weapon types are untested. Sharpness,
  Strength and Weakness are counted since 0.10.23 (accepted 2026-10-11; the
  logged numbers match the formula exactly; Weakness not tried). Still
  missing: Smite and the other creature-specific enchantments (left out on
  purpose), Fire Aspect, sweep attacks and knockback on enemies.
- Shield blocking and knockback take their direction from the nearest Skyrim
  character within about 8 blocks, which may not be the real attacker.
- Stand-in boxes for Skyrim's characters: dead ones have none since 0.10.39;
  every living creature still gets one human-sized box whatever its size, and
  the boxes move seven times a second. They no longer make the local server
  undo steps (0.10.38) or force a crawl (0.10.39); accepted for the close-combat test in the 0.10.50 report; the original
  problem they are meant to end ("blocked, very slow, cannot jump") was never
  captured in a log.
- Minecraft's attack recharge meter does not follow the bridge recharge timer.
- No Skyrim swing animation or knockback; kill attribution and crime reactions
  are unverified.
- Punching an enemy after leaving a Skyrim menu has not been tried on 0.10.4.
- One fist request in report 20261010-224713 was dropped with no logged reason.

Skyrim menus:
- **Menu pointer speed scaling (deferred 2026-10-11).** Pointer travel is
  currently "fraction of the Minecraft window" mapped to Skyrim's screen, so a
  small Minecraft window makes the pointer fast (854x480 against 2560x1440 gave
  about 3 Skyrim pixels per mouse unit). Add a scaling factor or a fixed
  pixels-per-unit mapping when the user asks. Done in 0.10.24 by measuring
  against the monitor instead of the window (accepted 2026-10-11: a 395-wide
  window measured against a 1920-wide monitor). The speed now follows
  Minecraft's Mouse Sensitivity setting (0.10.32; not exercised yet, the
  factor stayed 1.0 in the test session); a setting of its own, apart from the
  camera's, does not exist.
- First pointer movements travelling double: the measured speed is now kept
  between menus (0.10.12, untested). The first menu of a Skyrim session still
  starts from the stored value until one larger movement is measured.
- Wheel, right click, Skyrim's R action and the optional true click all reach
  Skyrim (0.10.11; 115 commands sent and dispatched once each in report
  20261011-094530). The user reports the middle-button true click "acts like
  left mouse in Skyrim", so a forwarded click works. Next candidate: make left
  click the true click, keeping Enter as confirm (done in 0.10.17, untested).
  The user confirmed on 2026-10-11 that R inside a container takes everything.
- E opens Skyrim containers and bodies and confirms inside Skyrim menus
  (0.10.18, accepted by the user and checked in report 20261011-101205).
- **Wanted by the user, not scheduled (2026-10-11): Skyrim's E takes
  precedence.** "Only open the Minecraft inventory when E won't do anything in
  Skyrim." So E should also talk to living characters, open doors and use
  other activatable objects, and fall through to the Minecraft inventory only
  when Skyrim's crosshair has nothing to activate. The crosshair kind is
  already published (0 nothing, 1 other, 2 container, 3 living, 4 body,
  5 door), so most of this is the Minecraft-side rule in
  SkyrimInteraction.beforeKeybinds. To decide when it is scheduled: whether a
  hostile living character counts as "E does something" (Skyrim's own E does
  nothing to an enemy in a fight, but Minecraft cannot yet tell hostile from
  friendly), and whether R then stays a separate key. Done in 0.10.19
  (accepted 2026-10-11; Skyrim reported "enemy" 31 times in the test
  session): an enemy is reported as its own crosshair kind and leaves E to
  the Minecraft inventory; R stays as it was. Both choices are the agent's and
  are the user's to overrule.
- Skyrim's items (I), magic (P) and favourites (G) menus since 0.10.43
  (accepted in the 0.10.45 report). An item cannot be made a favourite from Minecraft (Skyrim's F
  key in its item menu is not forwarded).
- Skyrim's character menu, map, journal and wait menu can be opened from
  Minecraft (Tab, M, J, and an unbound key; 0.10.20, accepted 2026-10-11 for
  Tab, M and J; the wait key is untried). Fast travel by a left click on the
  map works (first seen in report 20261011-155446).
  Skyrim's favourites menu and its other quick keys are not forwarded.
- No text entry; rebinding and non-dialogue menus are largely untested. Held
  directions repeat and W, A, S, D navigate since 0.10.31 (accepted in the 0.10.45 report; earlier
  W/S menu checks were initially deferred).
- Skyrim and Minecraft inventories are separate; no looting or trading into
  Minecraft's inventory.

Travel and saves:
- Interiors, doors and other outdoor worlds: each place now has its own
  Minecraft region (0.10.9, untested). Fast travel within reachable heights
  should follow from the same recovery step but is untested, and choosing a
  map marker from Minecraft may need the missing true click.
- Height: Minecraft's 384 blocks cover only about 26,900 Skyrim units. With the
  current saved origin, Skyrim heights below about -11,600 (including sea
  level) or above about 15,000 cannot be reached; the bridge now says so.
  Since 0.10.25 (built, not played) arriving is allowed from about -15,500 to
  155,000, because only the teleport was ever limited; **building** is still
  limited to Minecraft's own range. Open: (a) a taller Minecraft world so
  blocks can be placed anywhere. Earlier Minecraft versions allow a world from
  y -2032 to 2031 through a per-world data pack that replaces the overworld's
  height (from general knowledge, not checked against 26.3); that is a change
  to the bridge world's format, needs a backup and the user's decision, and
  has not been tried. (b) Below Minecraft y -128 (about Skyrim
  -16,000, deep sea floor) Minecraft's own void damage applies; not guarded.
  Guarded since 0.10.33 (built, not played): the player in a bridge world is
  exempt, and arrival is allowed 2,000 blocks below the floor.
- Long-distance and high-altitude travel need the coordinate-window remapping
  planned under "Next priority" below.
- Save revisions/rollback, automatic choice of the matching Minecraft world,
  and restoring historical inventory are not implemented.
- Collision coverage is a four-block sampled cube; thin objects can be missed
  and fast movement can cause holds. Since 0.10.37 a change of outdoor cell no
  longer lets go (that, not the cube, was the pause every 58 blocks), and since
  0.10.41 the cube looks up to 2.75 blocks ahead sideways. Cell-crossing play
  passed in Test 0.10.45; its vertical lead caused jump holds and is corrected
  in 0.10.46 as described above. At creative flying speed the cube can still
  be outrun for part of the time.

Building and world:
- Water and lava flow through Skyrim terrain (no fluid containment).
- Block size and reach calibration: user reported blocks look large and reach
  feels short at 70 Skyrim units per block.
- Placement leaves sub-block gaps and uses sampled box faces, not true surface
  normals. No terrain mining.
- Skyrim NPCs do not collide with placed blocks; no NPC stand-ins in Minecraft.
- Time and weather are not shared.

Rendering:
- No breaking cracks on glass or ice.
- Camera orbit and Minecraft-world bleed, reported at 0.6.1 and deferred.
- No third-person player model, block-entity models, mobs or animated textures;
  no Skyrim lighting on Minecraft geometry; no underwater effects.

Tooling:
- Rust tests were repaired in 0.10.8 and run before every build. They use fake
  objects; nothing tests the Java side automatically (Gradle has no tests).
  Since 0.10.29 the Java side's own arithmetic is checked before every build
  (BridgeRules with RulesCheck), and since 0.10.33 every hook into Minecraft is
  checked against the Minecraft jar (MixinTargetsCheck). Still unchecked by
  any machine: code that calls Minecraft, and anything about how either game
  behaves.
- Steps resting on unverified game behaviour can be switched off by the user
  in switches.txt (0.10.28). A new step of that kind should get a switch.
- Sprint state is not logged; Skyrim-window keyboard/mouse forwarding into
  Minecraft does not exist.
- Log volume: the native logs no longer write a line per collision box
  (0.10.26). Still heavy and left alone: the Minecraft client log gets six
  lines per block capture, about 55 a second, and both native logs get a line
  per rendered frame. logs\step2 held 34 GB on 2026-10-11; only the user
  removes it (tools\prune-logs.ps1 -Delete).
- Measured but not acted on (report 20261011-101205): Skyrim's collision scan
  takes up to 2 ms of every frame (about 2,700 ray casts per scan, a scan every
  0.15 s) whether or not the player moved. Scanning less often while standing
  still would give that time back; not planned, moving objects would need a
  separate answer. Since 0.10.34 both games log what the bridge costs per
  frame (SCENE_COST) and the scan logs its ray-casting time, so the first
  report after that says whether this is worth doing. Measured offline and
  left alone: handing Minecraft the collision packet costs about 80
  microseconds a frame. First in-game numbers (report 20261011-155446, Skyrim
  at 59 frames a second): the bridge takes about 1.1 ms of every Skyrim frame
  on average, worst frames about 2 ms, and nearly all of it is this ray
  casting (about 7.4 ms per scan over 7 frames). That is about 6.5% of a
  frame, all the time, including while standing still: the one optimisation
  the numbers point to. In Minecraft the after-render work averages about
  0.5 ms at 120 frames a second. Done for standing still in 0.10.40 (a calm
  scan at a quarter of the rate; built, not played, saving not yet measured).
  Still open for moving: reuse the part of the cube already scanned (a shift
  of half a block needs about a fifth of the rays); a larger change to a
  proven path, not started. Minecraft's cost line has four parts since 0.10.42.
- A mistake of method worth keeping: 0.10.21 was built against the wrong
  Minecraft counter because the bytecode was searched for expected words
  instead of printed and read (found and fixed in 0.10.27 before any play).
  Print the code.

## Mouse pointer in Skyrim dialogue (0.10.4, accepted)

Result: user confirms the pointer works in Skyrim's menus and inside NPC
conversation; report 20261011-001910 confirms the forwarded events numerically.
The plan below is kept as written.

Request: make Skyrim's mouse work while talking to an NPC. Today Minecraft has
keyboard focus, so Skyrim's window never receives mouse input; dialogue choices
only respond to the forwarded arrows/Enter/Backspace. Assumed meaning (confirm
with user): moving the mouse moves Skyrim's own menu pointer, hovering highlights
a dialogue choice, and left click confirms it. Written by Claude Code.

The two halves:
- **Minecraft:** while a fresh Skyrim menu status is active and no Minecraft
  screen is open, mouse motion that would normally turn the player is captured
  instead. Hook MouseHandler.turnPlayer at HEAD (26.3 signature checked with
  javap: private void turnPlayer(double), fields accumulatedDX/DY). The view
  stops turning during the Skyrim menu. A left click sends the existing
  "confirm" menu action (action 5, the same one Enter sends) and skips vanilla
  attack/mining for that click, so a click on a choice cannot break a block.
- **Skyrim:** the existing SKSE UI task applies the motion by building one
  ordinary MouseMoveEvent (CommonLib MouseMoveEvent.h; user event "Cursor") and
  passing it to MenuControls::ProcessEvent, exactly like the accepted key path.
  Skyrim's own cursor handler then moves the pointer and tells the menu about
  the hover. No pointer position is written directly.

Data crossing: Minecraft sends the menu context the motion belongs to and the
running total of motion since that menu opened, as fractions of the Minecraft
window (1.0 = one full window width or height). A running total is safe with
the latest-value slot: skipped frames lose nothing. Skyrim applies only the
change since the last total it saw. The first total for a context is a baseline
and moves nothing, so the pointer never jumps when a menu opens. Zero context
means "no pointer motion".

Skyrim converts the change to its own screen pixels (MenuCursor screen size),
then to mouse counts using a pixels-per-count factor per axis. Start from
MenuCursor's sensitivity value (or 1 if implausible). After each event, read the
pointer back and learn the real factor, including its sign, from moves of at
least 8 counts that end away from the screen edges. Motion Skyrim does not apply
(for example against a screen edge) is dropped, like a real mouse. Sanity-check
the MenuCursor numbers before use and skip when they are implausible.

Wire change: connection record grows 424 -> 440 bytes: u64 pointer context at
424, f32 total X at 432, f32 total Y at 436. Protocol 27, namespace link27,
pt_mc_frame13. World and scene packets unchanged. All three files update together.

Logs: Minecraft SCENE_CURSOR_SEND (context, totals, window size; at most four
per second). Skyrim SCENE_CURSOR state=baseline (pointer position, screen size,
sensitivity) and state=move (requested counts, pointer before/after, learned
factors; first five moves then at most four per second). Mouse confirms reuse
SCENE_MENU_SEND/RECV/RESULT with source=mouse on the Minecraft line.

Not in this step: mouse wheel, right click, a real Skyrim "Click" event (needed
for menus where the clicked spot matters, such as containers), and hiding or
drawing any Minecraft pointer. Right click still does its normal Minecraft
action during a Skyrim menu, as before.

Unknowns to settle from logs, not from the build: whether Skyrim's cursor
handler accepts a synthetic event (pointer read-back changes), the true
pixels-per-count factor and vertical direction, and whether hover highlights a
dialogue choice. This is attempt 1. If the pointer read-back never changes, do
not guess variations: attempt 2 would send the hover to the menu's movie
directly, and a second failure means a STATUS handoff.

## Attack recharge and deferred balance (0.10.3)

User reports damage feels far too high. Record a future balance pass for the10x
conversion (currently wooden sword40, fist10). Keep full-strength amounts for
this scoped feature; do not treat those values as final balance.

Implement bridge attack recharge: compute held-item ATTACK_SPEED from player base
plus MAINHAND component modifiers, just as repaired damage uses item components.
Use elapsed Minecraft client ticks since the last emitted bridge attack, with
charge=min(1,(elapsedTicks+0.5)*attackSpeed/20). First attack per player/world is
full. Damage multiplier=0.2+0.8*charge^2; capture it with the immutable request.
Use the same timer across item switches, so changing weapons does not grant a
fresh attack. Snapshot clearing must not reset this timer. A player/world change
starts a new timer. Only emitted bridge requests consume charge in this increment;
misses and real Minecraft block/entity attacks remain vanilla and outside this timer.

Lower the old fixed500ms client/native gate to100ms as a spam guard. Range, target,
expiry and dedup checks remain. Native still receives a final damage float. Bump
protocol26/namespace to prevent mixing old fixed-cooldown DLLs with this client.
Log speed, elapsed ticks, charge, multiplier, full-strength and requested damage.
This is a bridge recharge model; vanilla HUD meter is not yet synchronized to it.
No durability, effects, enchantments or criticals. Build then compare a rapid sword
pair against attacks separated by two seconds, using logs if enemies die quickly.

## Held-item damage repair attempt 2 (0.10.2)

0.10.1 runtime failed: client player ATTACK_DAMAGE was1 for both wooden sword
and air; both hits correctly transported10. Instead calculate base weapon damage
from the player's base attack attribute plus the held stack's ATTRIBUTE_MODIFIERS
component, using Minecraft ItemAttributeModifiers.compute for MAINHAND. Starting
from the base value avoids counting equipped modifiers twice. This scope includes
item component modifiers, but excludes temporary player effects, enchantment
bonuses, attack recharge, criticals and durability. Keep existing10x bridge scale.
Log base, observed client value, component modifier count and computed value.
Do not mutate player attributes or inventory. Transport and native guards unchanged.
Build0.10.2 then compare wooden sword and empty hand. If this second runtime
attempt fails, stop and write the template-based handoff rather than guessing.

## Held-item base damage (0.10.1 implemented; gameplay not tested)

Build one increment on verified punching: use Minecraft player's current
ATTACK_DAMAGE attribute at click time, including equipped-item base modifiers.
Send ten times that value as Skyrim damage, preserving the existing empty-hand
10-damage baseline. This is an initial balance choice, not a unit conversion.
Keep the existing500ms cooldown/range/target guards. Reject nonfinite, nonpositive
or over1000 requests; do not silently clamp them. Capture the value once per click
so switching items cannot change an in-flight request. Log item ID, attribute and
requested amount in Minecraft, and requested/before/after in Skyrim.
No enchantment-specific damage, durability, criticals or weapon attack-speed
cooldown in this increment. Vanilla Minecraft block/entity attacks retain priority.
Use reserved connection offset412 for f32 damage; protocol25 and frame12 prevent
old adapters interpreting this field. World snapshot layout remains unchanged.
Manual check: compare fully separated empty-hand and unenchanted sword hits on
hostile targets; verify10 vs greater damage in logs, then normal block breaking.

## Basic outgoing punch (0.10.0)

User clarified the final not-ready Skyrim hit was after closing Minecraft and
then Skyrim while chased; it is expected standalone behaviour, not a bug to fix.
Continue gameplay with Minecraft's normal attack input targeting Skyrim actors.

First increment is a fixed punch (10 Skyrim damage before Skyrim difficulty),
not weapon/enchantment/durability simulation. Observe Minecraft.startAttack at
HEAD without cancelling vanilla attack/block-breaking. Only send in focused,
active admitted bridge with no screen/menu/recovery; a real Minecraft block or
entity target takes precedence. Publish Skyrim crosshair handle in the reserved
world damage-header +12 word. Minecraft binds a short-lived attack to that handle.

Native dispatch validates request age<=250ms, one sequence once, idle-after-resume
barrier, cooldown500ms, unchanged crosshair handle, non-player actor type3E,
same cell and feet distance<=210 Skyrim units (3 bridge blocks), positive health.
Resolve handle with existing reference-counted lookup, retain through DoDamage
(Address Library37335), pass player as source and dontAdjustDifficulty=false.
Log target form/handle, distance and before/after health; do not infer hostility,
crime/kill attribution or animations from a returned call. Skyrim's crosshair is
the native aiming/occlusion filter, with conservative same-cell/range checks.
Target may go stale while moving: discard, never retarget an old click.

No synthetic Minecraft NPC entities, weapon stats, swings in Skyrim, criticals,
knockback or outgoing damage acknowledgements in this first step. Requests expire
rather than replay after loads/menus; Minecraft's normal swing remains. Use logs
to verify hits and test a real Minecraft block still breaks without hurting NPCs.
Protocol24; add sequence/handle/timestamp to connection; world size unchanged.

## Damage-clamp live result (0.9.9 accepted scope)

Report20261010-220115 confirms the primary slot0x127 callback captures three
negative health deltas before loss. Returning zero preserves native health100;
Minecraft consumes sequences1..3 once, with acknowledgements and expected scaled
health decreases. The second interception attempt succeeds for tested nonlethal
enemy hits. No further hook replacement is justified by this result.

A fourth hit after readiness loss passes through and reduces native health.
This confirms the documented fallback behaviour and leaves a gameplay-policy
gap: protection during unavailable bridge/death/recovery needs a separate scoped
increment. Current health ownership is conditional on active readiness. Minecraft
health rises afterward, but full healing and lethal lifecycle remain unverified.

## Incoming damage repair attempt 2 (0.9.9)

0.9.8 report 20261010-215421: native readiness reached true, maximum100, receiver
epoch1; two later HandleHealthDamage probes report -6.789 each, but zero mutation
capture or passthrough events. Enemy damage bypasses the intercepted virtual
ModActorValue entry in this runtime. This is a failed forwarding attempt.

Replace that interception with documented non-VR Actor::CheckClampDamageModifier,
primary vtable slot0x127, float(this,ActorValue,float delta). Call the prior clamp
once, log incoming and returned values plus current health, and only enqueue a
finite negative health result while readiness is valid. Return zero after capture;
return original result otherwise. Keep queue/ack/server application unchanged.
The local signature is verified, but enemy-path invocation and the effect of zero
are not yet runtime-verified. This is the second bounded attempt, not a proven fix.
If it fails, stop native hook variations and write the STATUS-handoff investigation.
Do not claim pre-mutation suppression until probe/health/application logs confirm.

## Minecraft-owned incoming damage increment (0.9.8)

Implement negative health mutation interception at PlayerCharacter's actual
ActorValueOwner subobject (+0xB8 for 1.7.104), vtable slot 6 ModActorValue.
Only modifier 2 (damage), actor value 24 (health), finite negative amounts are
eligible. This interface is earlier than the observed HandleHealthDamage probe.
Install from the known player instance; preserve the prior virtual function.

Native readiness is a short lease refreshed only after pairing/alignment and
Minecraft server readiness. Bind to peer session/world/load revision and player.
When ready, enqueue a unique sequence before suppressing this native mutation.
Queue holds at most 32 hits; outside readiness or on overflow pass through native
damage and log why. Reads use a mutex shared across callback threads. Do not hold
it while invoking the original mutation. Expire pending hits after five seconds;
loads/disconnects cancel the generation and log dropped pending hits. Delivery is
session-local, not durable across crashes; no claim of transactional cross-save
health. Preserve Skyrim's standalone behaviour outside bridge control.

Scale each captured amount by Skyrim maximum health, obtained exactly as CommonLib
GetActorValueMax: permanent health plus temporary modifier (Address Library 38469).
Minecraft requests that fraction of its own maximum through hurtServer(generic).
Minecraft armour, absorption, immunity/cooldown, creative state and death rules
remain native. Each event is attempted once; a rejected hit is acknowledged too,
with before/after health and accepted flag logged. This intentionally permits
Minecraft mitigation in addition to Skyrim's prior damage calculation; balancing
is deferred. Minecraft food/healing alone restores Minecraft hearts. Skyrim
healing is never forwarded. Native health is not set/mirrored in this increment.

Append generation plus 32 fixed-size events to world snapshots, retaining existing
recovery/lineage tail offsets relative to end. Return generation, highest consumed
sequence and server-ready flag with render samples. Native lease starts only after
that handshake. Snapshot retries cannot repeat a consumed sequence. Hold no raw
cross-process pointers. Server callbacks recheck world/player/freshness and skip
menus, recovery, dead players and invalid pairing. No queued damage after a load
into another bridge generation. Both adapters update under protocol 23.

Death in Minecraft suspends bridge control through existing alive checks. Full
Skyrim death/respawn presentation and direct scripted health setters remain out of
scope, not claimed covered by this mutation hook. First test nonlethal enemy hits,
then food regeneration; avoid testing death until this path is accepted. Log native
capture/passthrough/drop/ack and Minecraft receive/apply with matching IDs.

## Damage probe evidence affecting the next implementation

0.9.7 report 20261010-120922: HandleHealthDamage supplied -6.789 / -4.526 for
two enemy hits. Current health was already 93.211 / 89.694016 before the original
callback and identical afterward. Therefore this callback follows health mutation
in the observed path. It is not the suppression point for Minecraft health
ownership; before-minus-after here gives zero, not hit damage. Verify an earlier
health-mutation API before suppressing native loss. Callback amounts are signed;
other sources and lethal paths still require investigation. Shared command state
must account for callbacks arriving on different threads. No health-write code
is added based solely on this probe. This refines the staged plan below.

## Minecraft-owned health: accepted policy and first event probe (0.9.7)

User chose Minecraft health: Skyrim hits damage Minecraft hearts, Minecraft food
and healing control recovery. Minecraft owns death/respawn in the completed design.
This resolves the pending choice below. Existing read-only health feed remains.

Implement one increment at a time:
1. Instrument PlayerCharacter HandleHealthDamage (CommonLib non-VR slot 0x104).
   Assign a process-local hit ID; read current health before/after calling the
   original exactly once. Log supplied amount, measured health loss, attacker
   form ID and callback thread. Do not suppress Skyrim damage or modify Minecraft
   health in this probe. Confirm the callback's amount/ordering using real hits.
2. Once verified, queue final damage events to the admitted Minecraft server with
   session/load identity and acknowledgement/deduplication. Establish scaling from
   verified maximum health, not a fixed assumption. Apply through Minecraft's
   server damage API so normal health behaviour remains there.
3. Suppress/replace Skyrim health loss only while Minecraft-owned damage transport
   is admitted and ready; preserve native behaviour outside the bridge. Specify
   disconnect behaviour, lethal events, Minecraft death/respawn and Skyrim load
   handling before enabling this authority end-to-end. No dual damage or healing.

Only step 1 is implemented in 0.9.7. The callback signature/slot is documented
locally but runtime semantics are not yet tested. The original call remains intact
and no gameplay outcome is changed intentionally. Test two enemy hits and inspect
SCENE_DAMAGE_PROBE alongside SCENE_HEALTH_SEND. Minecraft hearts stay independent
until the next verified implementation stage. Do not claim health ownership is
fully implemented merely because the policy is chosen.

## Health ownership decision and damage rules (pending user choice)

Read-only damage/healing transport is accepted. Before health writes, choose
one health authority: Minecraft (food/inventory-driven survival) or Skyrim
(native combat/healing). The choice also determines death and recovery behaviour;
movement authority remains Minecraft in either case. No health-writing code is
implemented until this gameplay choice is resolved.

Rules common to either choice:
- Keep health observations separate from damage commands. A drop between sampled
  health readings is not an exact hit: regeneration, maximum-health effects and
  save loads can change the value between samples.
- Identify damage events or authoritative health revisions explicitly, with
  sequence numbers and acknowledgements. Repeated snapshots must not apply a
  hit twice, and mirrored writes must not return as new damage.
- Bind commands to the admitted save lineage, Minecraft world, peer session and
  save-load revision. Loading/reconnecting establishes a baseline, never a hit.
- Expired data, menus, recovery and unsupported locations suspend health writes.
- Do not guess a fixed Skyrim-health-to-heart ratio from the observed 100/20
  baseline; maximum health and modifiers can change. Specify scaling explicitly.
- Implement and verify nonlethal damage first. Death, respawn and inventory loss
  require an explicit policy; do not claim complete shared health before these
  are handled. Keep the current read-only build available during development.

Next implementation depends on the user's health/healing authority choice.

## Combat foundation: read-only Skyrim health (0.9.5)

Dialogue navigation is accepted. Begin combat in small increments:
1. Read Skyrim player current health on the existing player update thread and
   include it in admitted collision snapshots. Log SEND/RECV with snapshot keys,
   plus Minecraft's own health for comparison. No health writes or attacks yet.
2. After verifying readings under actual damage/healing, design health authority
   and death/load baselines before applying damage across games.
3. Add player attack targeting and damage only after those rules are established.

This increment implements step 1 only. CommonLib ActorValueOwner at Actor+0xB8
(for runtime 1.7.104, versioned since 1.6.629), vtable slot 1 GetActorValue, ActorValue::kHealth=24. Read current health,
not base/max health; negative finite values are possible near death. Use reserved
world header bytes 68..72 for f32 health, retaining packet size and all other
fields. Bump protocol and magic together. Reject nonfinite values; only admitted
snapshots log health so disconnects and recovery do not produce false zeroes.
The value is sampled with completed collision exports, not every damage event;
short-lived damage may not be captured. Menus/load/disconnect pause this feed.
User tests ordinary damage/healing and reports behaviour; compare keyed numeric
logs. Minecraft hearts are intentionally independent in this first increment.

## Menu state repair (0.9.4)

Recovery report 20261010-114219 confirms token 1 server/client positions match,
with native acceptance error_squared=0. User reports recovery appears to work.

Implement the next menu increment: keep one process-wide Mutex<State> containing
context fingerprint, monotonically increasing context number and last command.
SKSE UI callbacks may run on different threads; serialize state and dispatch using
a nonblocking lock. A contending/reentrant callback skips instead of deadlocking.
Keep only the shared-memory mapping handle thread-local, passing it to the shared
state during the callback; no unsafe Send implementation for raw mapping pointers.
All game access remains inside SKSE UI callbacks. Add callback thread identity to
state, receive and result logs. Preserve expiry, pairing and menu-only dispatch.
This fixes command ownership first; ordinary ButtonEvent routing is unchanged.
Test one Up/Down press per action, Enter and Backspace, held-key suppression and
movement resume. Do not claim dialogue selection works until user confirmation.

## Save-position recovery implementation (0.9.3)

Step 1 accepted by user; report 20261010-113517 confirms outdoor_cell_accepted
for cells 9731 and 9712 in worldspace 3C with mapping preserved.

Step 2: when paired saved positions exceed the existing 140-unit guard, Skyrim
publishes a recovery token and the loaded player's position converted through
the saved origin. It remains unadmitted and does not move the puppet. Bind the
request to save-load revision, Minecraft session/world and saved identities.
A load, disconnect, location change or invalid pairing cancels it. Do not change
PBOR or move existing blocks. Extend the world packet for the token/target and
send an acknowledgement in unused connection bytes; bump protocol together.

Minecraft validates the pairing and receives the request on its render thread,
holds motion, and queues one teleport through its integrated server's normal
player connection. That connection updates the server and sends the normal
teleport to the client. Check the server position after teleport, then acknowledge
only once the client also reaches the target. Recheck session,
world, player and request identity in deferred callbacks; stale requests cannot
move a new world. Keep collisions/rendering held until Skyrim accepts the token
and exports fresh admitted coverage. Timeout remains held with a logged reason.
Use the current Minecraft inventory; this does not restore historical inventory.

Native acceptance checks token, fresh interpolated position near the target,
save revision and current Skyrim position still near the captured position.
A 30-second request timeout holds instead of repeatedly teleporting. Logs include
request, target, server/client readiness and acceptance. Test a paired save that
previously reported disagreement, marker-block stability, and quickload while
Minecraft remains open. Menu repairs remain the next separate increment.

## Recovery plan after 0.9.1 testing (2026-10-10)

Evidence: scene-report-20261010-112905 shows a walking transition from cell
00009732 to 00009731 within worldspace 0000003C rejected as origin reason 4.
Initial load passed lineage admission but failed the 140-unit alignment guard
(reason 5). Menu context tokens alternate across callbacks and sequence 10 was
dispatched twice. The menu task state is thread-local, although callbacks can run
on different threads; context and deduplication therefore do not have one owner.

Implement and test one increment at a time:

1. Outdoor continuity (0.9.2): treat cells in the same nonzero worldspace as one
   coordinate system. Preserve the saved origin, Minecraft world identity and
   position-distance guard. Interiors still require the exact cell, and different
   worldspaces remain blocked. Rebuild collision coverage at a cell transition;
   log the accepted outdoor transition. Apply the same location rule to menu
   eligibility. This addresses walking across outdoor cell edges, not save drift.
2. Save-position recovery: retain the saved coordinate mapping and placed blocks.
   Add an explicit resynchronization handshake that moves Minecraft to the loaded
   Skyrim position expressed in Minecraft coordinates, with movement/rendering
   held until both Minecraft sides acknowledge it. Do not shift the origin or
   silently teleport Skyrim to the current Minecraft position. Keep lineage/world
   checks; report unsupported worldspace/interior changes. Inventory remains the
   current Minecraft inventory; historical inventory rollback remains separate.
3. Menu repair: give all UI callbacks one synchronized context/deduplication state,
   while keeping native game access inside the UI callback. Log callback thread
   identity and one dispatch per sequence. Then establish whether normal menu
   events change dialogue selection; if not, investigate the dialogue-specific
   input path before changing it. Until accepted, use Skyrim's native controls
   for dialogue choices after R opens the conversation.

Only step 1 is implemented in this increment. Loading mismatched positions and
arrow-key dialogue selection remain known failures, not accepted features.

# Passthrough design

## Current gameplay increment: menu navigation (0.9.1)

Add remappable arrows, Enter (confirm), Backspace (back) in Minecraft, routed only
to an open Skyrim menu. Keep Minecraft's screens and normal keys unchanged. Use
SKSE's UI task queue because pausing menus can stop player updates. A scheduler
queues at most one task every 20 ms; only the UI callback reads game objects or
dispatches input. It validates fresh Minecraft samples, handshake, loaded Skyrim
lineage, saved Minecraft world and saved location. It refreshes the existing
Skyrim connection heartbeat during UI updates without taking over another owner.

UI status reports a fresh menu-context token to Minecraft. Each key press carries
that token, a sequence and timestamp; deduplicate, reject stale/changed contexts,
and deliver one ordinary ButtonEvent down/up pair to MenuControls only (not player
controls). Own native event strings via Skyrim's string constructor/release APIs.
Menu stack fingerprint and load revision change the context token. A held key does
not auto-repeat in this first version; rapid simultaneous keys may be dropped.
Publish status again after dispatch so menu closure releases Minecraft's hold.
Minecraft clears collision/render export while fresh menu status is active and
continues stationary heartbeat samples so UI input can proceed. Missing status
does not authorize menu commands. Pause/mismatch/load guards remain in force.

Grow the connection record for separate menu request/status fields and bump
protocol. This is keyboard menu navigation, not mouse/text input, automatic
dialogue choices or inventory synchronization. Opening menus uses the existing R
interaction. Test NPC dialogue navigation/confirm/back, then a pausing container,
verify movement is held, and verify stale/held keys do not activate gameplay on
exit. Native menus retain Skyrim semantics (including purchases/taking items
only when the player presses confirm); log dispatch, not assumed outcomes.

## Gameplay priority: Skyrim interaction (0.9.0)

User chose gameplay features before travel/rollback. First add a remappable
Minecraft "Interact with Skyrim" key (R by default, Miscellaneous category),
without changing Minecraft use/attack/inventory bindings. Register it before
Minecraft reads saved key settings. In the focused, unpaused offline bridge,
with no Minecraft screen and fresh admitted coverage, each press sends one
sequence-numbered activate request for at most 200 ms. Holding the key never
repeats; clicks in menus/inactive contexts are discarded. No accumulating queue.

Carry request sequence and native issue time in the remaining connection bytes
272–287; retain them across repeated frames, expire at 250 ms on Skyrim's side,
deduplicate by sender/world/sequence and limit calls to one per 250 ms. Bump
protocol. After lineage and origin admission, on Skyrim's player-update thread,
call its normal PlayerCharacter::ActivatePickRef (AE relocation 40548), using
CrosshairPickData target at +4 (singleton AE 401585). No target means no call.
Log request sequence, target handle, issue age and dispatch/drop reason. A returned
void call means dispatched, not proof that an object changed. Skyrim owns target
selection, reach and activation outcomes; Minecraft does not choose a target ID.

Pausing/loading, missing samples, admission failure and origin holds invalidate
pending interaction. Never replay a held request on resume. Suspend movement/look/
render control while Skyrim UI reports pause, item, application, cursor-retaining
or modal menus (including dialogue's cursor-retaining flag). After activation,
check menus/loading/cell again and release immediately if changed. Menu navigation
is not forwarded yet; use Skyrim's window to close/operate its menus this increment.
Cross-cell doors will hold under the existing 0.8 origin restriction.

Test an in-place lever/switch first, an empty crosshair, a held R, a rebound key,
and key presses in Minecraft inventory. Containers/NPCs may open native Skyrim
menus but cannot yet be operated from Minecraft. Next: menu input routing, then
combat/damage. Travel/rollback stays deferred. This order supersedes the earlier
0.9 travel-first outline below.

## Release scope: 0.8 restart milestone complete; travel/save history in 0.9

Acceptance recorded in report 20261009-212809: the user reports block/inventory
persistence and the saved/restored origin fields match exactly across Skyrim
processes 19424 and 17024. This completes the scoped same-location restart milestone;
remaining limits below still apply. An earlier setup hit the proximity guard before
reloading a legacy save and creating the final persisted origin.

0.8 ends with a paired practice world that can be saved, closed and reopened at
the same Skyrim location without shifting construction. Pairing/admission are
confirmed. The remaining increment (0.8.9) saves one coordinate origin in a
versioned SKSE record: Minecraft world UUID, Skyrim cell/worldspace form IDs,
Minecraft origin XYZ and Skyrim origin XYZ. Use the UUID already in this project's
`skyrim-bridge-<UUID>` save folder name; renamed folders cannot restore this mapping.
Send it separately from the lineage ID. This distinguishes two Minecraft worlds
paired to the same Skyrim lineage. No inventory or block copying is needed.

A legacy save without an origin record establishes the first admitted origin as
before, retains it in memory and writes it on the next Skyrim save. Once present,
never silently replace it. Restore only for the same Minecraft UUID and same Skyrim
cell/worldspace (resolve saved form IDs through SKSE for load-order changes).
Require the mapped Minecraft player to be within two blocks (140 Skyrim units)
of Skyrim's loaded position before resuming. This bounds return teleports but is
not save-revision validation. Malformed, duplicate or unsupported origin records,
unresolved forms and mismatched context hold the bridge, send identity-only replies
and log a reason. Preserve invalid-origin state on subsequent saves with a rejected
record marker, so it cannot turn into a legacy absence and silently re-anchor.

Use the same mapping for movement, collision and rendering through Anchor, rebinding
only transient connection IDs after restart. Preserve it across pauses rather than
aligning construction to native movement. Grow the connection record to carry the
world UUID without overlapping fields, and bump protocol. Log exact origin values
on create/save/load/restore. User procedure: stop moving, save Skyrim to a new slot
with both games connected, Save and Quit Minecraft, close both, reload those saves
at the same place. Compare block/landmark positions and exact origin logs.

0.9 owns save-revision rollback/checkpoints, doors/interiors, location-specific
construction, coordinate-window remapping and unlimited altitude/long travel.
0.8 does not claim those behaviours or automatic matching of older save revisions.

## Admission repair (0.8.8)

The 0.8.7 pairing lookup can run before Minecraft's overworld exists; do not cache
that transient state as a rejected world. Also move the expected ID from bytes
216–231 (already occupied by renderer status) to unused bytes 240–255 in the
256-byte connection record. Bump protocol to isolate the broken encoding. Keep
all admission semantics unchanged. Recheck the matched-save path before asking
the user to test mismatches. Neither save data nor pairing metadata needs migration.

## Current increment: enforce pairing before puppet control (0.8.7)

Send the Minecraft world's stored lineage ID in each rendered position sample,
using reserved bytes 216–231. Zero means unpaired/invalid metadata. Before applying
any position, look, scene alignment or collision scan, Skyrim compares it with
its currently loaded/published lineage. Require a nonzero exact match. On rejection
release bridge control and publish an identity-only reply with no geometry and
admitted=false; this allows initial pairing without moving either player. Minecraft
observes identity replies but discards their geometry, holds movement and suppresses
scene/overlay export until a fresh admitted snapshot arrives. Normal snapshots set
admitted=true in header u32 at 120. Protocol bump isolates older adapters. Log gate
transitions with expected/loaded IDs. This is lineage admission only, not matching
save revisions or restoring origins. Build and test matched resume, first pairing
and mismatch; no gameplay success claim before log/user evidence.

## Current increment: explicit Minecraft save pairing (0.8.6)

Append the Skyrim lineage ID to collision snapshots and expose only IDs successfully
written by SKSE or loaded from its record. Bump the wire protocol. In marked offline
Minecraft saves, read a small versioned pairing file from the world's root. When
unpaired and an ID arrives, ask the player to pair this world with that Skyrim
lineage; cancellation leaves the file absent. Create the file once, never overwrite
an existing pairing, and reject malformed metadata. On each changed incoming ID,
log matched/mismatched/missing identity and show a clear mismatch warning. This is
a diagnostic pairing increment: it does not yet gate native puppet movement,
automatically choose Minecraft worlds, restore origins or guarantee paired saves.
Keep those limits visible in the confirmation and warnings. Cache metadata for the
open server session; write failures remain visible and require reopening to retry.
Validate the confirmation against the current server and latest received ID before
writing. Pairing files contain only schema version and lineage ID, no game assets.
Build, then accept pairing, save/quit Minecraft and reopen the same bridge with the
same Skyrim save. Logs should show paired then matched. Different lineage should
warn mismatch without replacing metadata; stop bridge testing on that warning.

## Current increment: Skyrim save-lineage identity (0.8.5)

Before pairing Minecraft saves, persist a randomly generated 128-bit bridge ID
through SKSE's serialization callbacks. Generate it at the first normal Skyrim
save lacking bridge data; save it in one versioned record in the SKSE companion
save. Restore it when that save loads and clear it on revert. Log creation,
save and load under SCENE_SAVE_ID. Existing saves without the record remain
usable and acquire an ID on their next save. Reject malformed, duplicate or newer
versions of our record and skip writing it rather than silently replacing identity.
Unknown record types are ignored. This ID follows a save lineage: Save As/copies
inherit it, while an older save predating the record obtains a different ID.
It is not a save revision, plugin-load-order fingerprint or paired Minecraft
checkpoint. SKSE owns all disk writing during normal game saves; the agent does
not edit save files. This increment adds no origin restoration or pairing yet.
Next steps must pair Minecraft metadata with this lineage, separately track save
revisions and mapping, and reject mismatches before applying restored origins.
Build, then save under a new slot, quit Skyrim, load that slot and compare the
SCENE_SAVE_ID saved/loaded IDs. Keep the previous slot for rollback.

## Current increment: retain alignment across temporary pauses (0.8.4)

Keep a suspended origin in memory when menus, stale samples or a short update
gap stop the bridge. Continue releasing camera, gravity, rendering and collision
as before. On resume reuse that origin only for the same Minecraft sender/world,
the same Skyrim cell and no Skyrim load/reset, and only if Skyrim's player is
within one Skyrim unit of its suspension position. Otherwise discard it and log
why before establishing a fresh origin. This tolerance permits rounding only;
movement while disconnected must not cause a return teleport. Invalidate on
cell changes and SKSE load/reset notifications. No disk persistence, save binding,
door handling or construction remapping is included. Log suspend/resume/new-origin
decisions under SCENE_ALIGNMENT so the normal collector captures them. Build,
then place a block by a Skyrim landmark and pause/resume each game to check it
stays fixed. Full restarts and movement while disconnected remain unsupported
alignment cases and must not be described as persistent save alignment.

## Current increment: identify Skyrim locations (0.8.3)

Start the stable-coordinate/save step by carrying the current Skyrim cell and
worldspace form IDs in every collision snapshot. These are game record IDs,
not process memory addresses. Log them on Skyrim publication and Minecraft
receipt when the location or connection changes. A null worldspace is represented
by zero (typically an interior); retain the cell ID as well. This first increment
does not bind saves, move construction, change alignment or enable door travel.
Form IDs depend on the installed plugin load order and are not a Skyrim save ID;
persistent pairing will also need plugin identity and an explicit save binding.
Use reserved header bytes with a protocol bump so older adapters cannot silently
ignore the new data. Build, then compare the two location logs in the same place
and after restarting both games there. Gameplay remains not tested until reported.

## Current increment: predictive local collision coverage (0.8.2)

The user confirmed save resuming and authorized travel coverage. First move the
existing scan volume ahead instead of increasing ray count without measurements.
Estimate velocity from consecutive distinct rendered position samples and their
send timestamps. Reset on cell/connection resets, gaps over 250 ms or teleport-like
motion. At scan start, predict over the previous complete scan duration plus
150 ms, bounded to 200–500 ms. Cap horizontal lead length at 1.5 blocks and vertical
lead at .75, keeping the player's body inside the four-block-radius cube. Quantize
the scan centre to the existing half-block grid and keep it fixed until complete.
New directions affect the next scan; do not cancel every in-progress scan on turns.

Retain existing ray spacing/count, per-update budget, snapshot freshness and
movement containment checks. Missing coverage still holds movement, including
rapid reversals and fast falls. Do not merge shapes from different snapshot
origins or claim unsampled areas are air. Log each scan's velocity, lead, horizon,
elapsed time, centre and publication/discard decision under the SCENE_ collector.
Distinguish coverage expiry, heartbeat loss, wrong world and outside bounds in
Minecraft, with hold/resume durations and requested body/path bounds. This is a
bounded first travel improvement; larger coverage and exact collision shapes
need measured gameplay evidence. No scale/reach, save mapping or fluid changes.

Build then use the same bridge save for walking, sprinting, turns, slopes, steps,
jumps and short drops. Compare scan timing/lead with hold reasons/durations on
both sides. Do not claim smoother motion from compilation alone.

## Current increment: resume the same bridge save (0.8.1)

The user confirmed the empty-world setup works and raised inventory persistence.
Before further travel work, make continuing an existing bridge explicit. New
Skyrim bridge is one-time setup per independent playthrough, not a startup action.
Add Resume bridge on the world list: read Minecraft's save summaries, select the
most recently played marked bridge, show its name/folder for review, then use
Minecraft's normal open/save flow. If there is no bridge, explain how to create
one; never silently create a fresh inventory. Other bridges remain selectable
from the normal world list. Confirm new-world creation explains the separate
inventory and leaves existing saves intact. Guard asynchronous results when the
user has left the screen. Do not bypass Minecraft's save/version/lock checks.

Keep inventory, equipment, experience and blocks in that same vanilla save; do
not copy them between worlds or maintain a second inventory file. Log save folder,
player UUID, slot item IDs/counts, equipment and experience after join and at the
normal player-save return hook. These are state diagnostics, not proof of disk
durability or full item-component equality. Build and ask the user to save, quit
and reopen the same bridge with distinctive inventory. Full Skyrim-save pairing,
rollback, stable world alignment and ahead-of-travel coverage remain separate steps.

## The two halves

- **Skyrim half:** an SKSE plugin keeps Skyrim in charge of terrain, NPCs, quests,
  and saves. It supplies the nearby world shape and NPC information, follows the
  Minecraft player, and draws Minecraft's graphics over Skyrim's scene.
- **Minecraft half:** a Fabric mod keeps Minecraft in charge of player movement,
  health, inventory, crafting, and blocks. A mostly empty Minecraft world receives
  Skyrim's collision shapes so Minecraft's existing physics can use them.

Follow SkyCraft's approach: translate between the games and keep their normal
gameplay logic. Use Rust stable with the MSVC compiler tools for the shared
connection code, with a thin Java/Fabric adapter and a Rust SKSE adapter in step 2.
Both games run on this PC, in single-player offline sessions.

## Data crossing between them

| Direction | Data |
| --- | --- |
| Both ways | Protocol version (the agreed data format), process identity, connection acknowledgement, and heartbeat (a regularly updated sign that the other side is alive). |
| Skyrim to Minecraft | Nearby collision shapes, NPC positions and hitboxes, player damage, keyboard/mouse input, menus, world/cell changes, time/weather, and save/load requests. |
| Minecraft to Skyrim | Player position, look and pose; camera; hits on NPCs; block changes; health/death; and open Minecraft menus. |
| Minecraft to Skyrim, rendering | Transparent hand/interface images; nearby block triangles with texture coordinates, colors and a runtime texture atlas. Skyrim renders those triangles through its own camera and depth buffer. |

Use Windows named shared memory for state and messages, as SkyCraft does. Step 1
uses a named mutex (a lock shared by the two processes) to protect its small
connection record. Later steps add event queues and frame data as needed; the
rendering step first uses asynchronous pixel readback through shared memory,
as SkyCraft's current hand/HUD exporter does; shared GPU textures are a later
optimization after measuring timing. Give this project its own memory names
so it cannot attach to a running SkyCraft instance.

## Build order

1. **Prove the connection outside the games.** Build a reusable Rust connection
   library and a command-line probe run twice, once for each game role. Exchange
   version and identity, acknowledge each other, update heartbeats, log connection
   changes, reject duplicate roles/incompatible versions, and detect and recover
   from a stopped peer. Check these with two real local processes. This step does
   not load into either game or transfer player movement.
2. **Send rendered player position and move Skyrim's puppet.** Load the Rust
   connection from Fabric and SKSE. After each Minecraft render, send
   `player.getPosition(partial)` using that frame's partial-tick fraction, plus
   a frame number and world/session identity. A latest-value slot lets Skyrim
   use the newest complete sample each player update; intermediate frames can
   be skipped when the games run at different speeds. Do not interpolate again
   on Skyrim's side. On the first usable sample, align Minecraft's position with
   Skyrim's current position, then apply deltas: `skyX += 70 * mcDeltaX`,
   `skyY -= 70 * mcDeltaZ`, `skyZ += 70 * mcDeltaY`. Move the Skyrim actor and
   its character controller together after Skyrim's normal player update, so
   Minecraft determines the final position. Clear Skyrim's controller velocity
   and temporarily suspend its gravity, restoring gravity when samples stop.
   Log each sent and consumed frame with the original coordinates and partial
   tick; also log the Skyrim target and position read back after applying it.
   Release the puppet on world changes, loading, or stale samples (250 ms).
   After a paused update loop resumes, release and start with a new alignment.
   Only an unpublished local
   single-player world sends usable positions. The SKSE adapter is pinned to
   runtime 1.7.104 and uses published SKSE/CommonLib ABI definitions and the
   matching Address Library, read at runtime. Game installation writes require
   separate authorization under the project rules.
3. **Nearby world collision and NPC positions.** Sample Skyrim's Havok physics
   world with rays along all three axes, in both directions, around the puppet.
   Use half-block spacing in an eight-block cube and continue past hits to find
   multiple surfaces. This is SkyCraft's ray-sampling first version: thin objects
   between rays can be missed; it is not exact mesh export. Budget work across
   updates and publish only complete snapshots. Convert surface hits into small
   axis-aligned boxes using the same origin and axes as player movement. Include
   nearby loaded NPC IDs, positions, and approximate body boxes. Exclude the
   Skyrim player and non-world ray hits. Preserve Minecraft blocks and append
   Skyrim shapes to block collision queries on both the client and integrated
   server, leaving movement, gravity and step-up code unchanged. Publish immutable
   snapshots between threads, restrict them to the local player's dimension,
   and clear on loading, origin changes, disconnects, or expired data. Use a
   separate versioned shared-memory snapshot slot with fixed limits. Log matching
   snapshot IDs, counts, individual boxes and NPC positions in both directions;
   also log collision-query use on each Minecraft side. Keep camera and input
   forwarding for a later step.
4. **Mouse look from Minecraft to Skyrim.** Send rendered player yaw (left/right)
   and pitch (up/down) in the same frame as position. Minecraft handles mouse
   sensitivity normally. Convert degrees to Skyrim radians: heading = yaw minus
   180 degrees, pitch keeps its sign, matching the existing world axes. Set the
   Skyrim player's angles and let its normal first-person camera follow them.
   Temporarily switch ordinary third-person view to first-person and restore it
   on release; leave special/scripted cameras alone. Log source angles, targets,
   actor read-back and camera-state rotation for diagnosis. Full camera placement,
   FOV/bob matching and input from Skyrim's window are later work.
5. **Draw Minecraft in Skyrim.** Start with hands and interface, then blocks and
   other world objects with correct depth. Measure frame timing before improving
   texture transfer performance.
   - **5a, implemented:** keep Minecraft's normal hand/HUD rendering, but skip its
     level drawing and clear to transparent only while Skyrim's overlay renderer
     reports ready. Read the resulting image asynchronously, send the newest
     finished RGBA frame through a bounded shared-memory slot, and alpha-blend
     it over Skyrim immediately before Present (the call that displays a frame).
     Save and restore Skyrim's graphics state. Cap capture at 30 frames/second
     and 2560 by 1440 pixels, report skipped/oversized frames, invalidate on world
     changes/pauses/disconnects, and restore normal Minecraft world rendering
     when the receiver is unavailable. Log frame identity, dimensions, alpha
     counts/checksum, capture/transfer/upload/draw timings and errors. No pixel
     captures or game assets are written to the repository.
   - **5b, first world-mesh build (0.6.0):** follow SkyCraft's world-mesh path:
     Minecraft's own block renderer supplies textured triangles, rather than
     another flat screenshot. Export complete bounded snapshots of nearby block
     models, their texture atlas and world identity through separate shared-memory
     mailboxes. Refresh geometry under a per-frame work budget; reject overflow
     instead of displaying an incomplete snapshot. Keep all textures in memory.
     Skyrim maps vertices through the existing movement alignment (70 units per
     block), uses its actual rendered camera projection, and hides fragments
     behind its scene depth. Probe and log the depth convention; suppress world
     drawing when depth is unavailable or untrustworthy. Give Minecraft blocks
     their own depth buffer so they also hide each other. Draw hands/HUD afterward.
     Log mesh/atlas identifiers, vertex counts, checksums, projection, depth
     samples and CPU timings. Start with nearby static block models, including
     cutout models. Fluids, translucent sorting, animated entity models and a
     mirrored third-person camera require further rendering work and are not
     claimed by this first world-mesh package. Keep ordinary Minecraft collision
     intact; rendering will make the nearby blocking terrain visible in Skyrim.
   - **5b next, breaking cracks (0.6.2):** read Minecraft's existing block damage
     stages and append a bounded crack layer to each nearby mesh snapshot. Load
     the ten crack textures from Minecraft's resource manager into an extra strip
     of the in-memory atlas. Draw crack faces just outside the block's shape
     bounds, after solid blocks, using transparency and the same Skyrim wall
     depth check. Only full opaque cubes are supported initially; skip other
     shapes explicitly rather than covering empty space with a crack box. Clearing
     damage, finishing a break or leaving the world clears the layer through the
     next complete snapshot and existing stale-data rules. Log positions/stages,
     crack vertex counts and matching send/receive/draw records. Increment the
     protocol for the new crack-count header field. Build and package this one
     change; in-game validation requires survival mining, cancellation and a
     Skyrim wall hiding a damaged block. Dropped items, camera orbit and world
     bleed are outside this increment.
   - **5b next, dropped items (0.6.3):** export nearby real Minecraft item entities
     as small, double-sided spinning icons using their item model's particle
     sprite, following SkyCraft's simple icon fallback. This first increment
     does not reproduce full 3D item models. Combine the runtime block and item
     atlases plus cracks in memory, remapping coordinates into the combined atlas.
     At each complete mesh publication, sample item positions with the current
     partial tick, apply bob/spin, and append at most 64 icons before cracks.
     Keep Minecraft in charge of motion, merging, pickup and disappearance;
     Skyrim only draws their geometry with normal depth and alpha cutout.
     Add item vertex counts to a new protocol header, log entity IDs, stack
     counts and positions on the sender plus matching counts on receive/draw.
     Limit export work and report unsupported sprites/capacity limits. Build this
     increment, then check dropping, pickup, wall occlusion and prior features.
     Terrain, camera orbit and world bleed remain separate.
   - **5b next, 3D full-block drops (0.6.4):** the user confirmed 0.6.3 icons
     spin, can be picked up and disappear correctly. Extend just dropped opaque
     full-cube blocks to quarter-block-sized spinning models, using the default
     block model's six directional faces and texture coordinates. Keep non-block
     items and unsupported models on the proven icon fallback. Preserve current
     item physics, export range/budget and depth handling. Log model versus icon
     counts; send an explicit entity count now that vertices per item vary.
     Increment the protocol, build both adapters, then check dirt/stone/log cubes,
     apple fallback, pickup, occlusion and cracks. Complex/custom block models
     and item-specific components remain later work.
   - **5b next, translucent placed blocks (0.6.5):** user chose further rendering
     support before interactions. Collect translucent block-model faces such as
     glass/ice separately, retaining Minecraft texture alpha. Sort their quads
     far-to-near from the Minecraft eye at mesh publication; refresh with existing
     roughly 10 Hz snapshots. Send their vertex count in a new protocol header.
     Skyrim draws opaque blocks/items and their cracks first, then translucent
     faces with standard alpha blending, depth testing and no depth writes, then
     hands/HUD. This allows foreground glass to blend over cracks behind it.
     Preserve Skyrim wall occlusion. Bound combined geometry with the existing
     vertex limit and log counts/timing and matching receive/draw metadata. This
     is approximate quad sorting, not exact intersecting-surface transparency;
     fluids, translucent dropped-item models and colored light remain later work.
     Build this one increment and check glass/ice, overlapping panes, opaque-block
     and Skyrim-wall occlusion plus earlier cracks/drops.
   - **5b fix, compass layering (0.6.6):** the user confirmed translucent blocks
     render, but world geometry covers Skyrim's compass. The existing world pass
     runs at Present, after Skyrim UI. Hook HUDMenu's inherited PostDisplay slot
     (CommonLib: slot 6 calls the HUD movie's Display), draw Minecraft world
     geometry before invoking the original, and retain only Minecraft hands/HUD
     at Present. Reuse isolated D3D state and restore it before Skyrim draws UI.
     Keep projection, transparency and depth comparison unchanged. Log hook
     installation, pre-HUD world-stage activity and missing-stage conditions.
     Do not fall back to world drawing at Present, which would cover the compass
     again. If HUD rendering is suppressed or the hook is unavailable, world
     drawing may be absent; log this limitation. Build and check the compass over
     a tall Minecraft wall, transparency/drops/cracks and menu transitions.
     The user now confirms the HUD is no longer blocked. Glass, panes and ice
     still drop as icons, consistent with the current opaque-full-block-only
     3D drop path; red sand still drops in 3D. Extending dropped-model support
     requires handling translucent faces and non-full geometry separately from
     the already implemented placed-block transparency.
   - **5b small extension, transparent/non-full drops (0.6.7):** reuse existing
     model export and sorted transparency. Gather directional and unculled quads
     from default block models (up to 96 quads/item), allowing non-full shapes.
     Route translucent drop faces into the same sorted collection as placed
     faces; keep opaque/cutout faces in the item prefix. Roll both buffers back
     on errors and remove prior dynamic faces before publication retries. Keep
     icon fallback for unsupported/oversized models. Send explicit model and
     translucent-item counts because item models now have variable face counts.
     Preserve pre-HUD drawing, gameplay and the existing total vertex limit.
     Check glass/panes/ice, red sand and apples, pickup and HUD visibility.
     The user confirms this extension works. Glass/ice breaking cracks are still
     absent: CrackCapture retains its opaque-full-block filter. Supporting those
     cracks needs a separate change, including their ordering with translucent
     surfaces; the dropped-model extension does not cover it.
   - **5b next, fluid surfaces (0.6.8):** leave glass/ice cracks deferred and add
     Minecraft water/lava geometry. Use Minecraft's FluidRenderer for heights,
     flow surfaces, tint, lighting and face visibility, including waterlogged
     blocks. Rebase its section-local vertices into the snapshot origin, map
     block-atlas UVs, and send opaque/translucent fluid faces through existing
     depth/blending stages before Skyrim HUD. Log fluid cells and vertex counts
     with matching native metadata. Keep the current region/vertex/time bounds.
     Minecraft still owns fluid simulation; this does not import Skyrim water,
     reshape fluids around Skyrim collision, add underwater effects, or animate
     texture frames. Build and check still/flowing water, lava, waterlogged blocks,
     Skyrim-wall occlusion, pickup/drop/crack regressions and compass visibility.
6. **Connect interactions.** Add NPC stand-ins in Minecraft, damage in both
   directions, block placement, and Skyrim NPC collision with those blocks.
   - **Next, surface targeting and placement:** use fresh Skyrim terrain shapes
     to find where Minecraft's aiming ray meets a surface, compare against normal
     Minecraft hits, and map the nearest reachable hit and face to the Minecraft
     block grid. Exclude NPCs as building surfaces. Validate placement against the
     same snapshot on the integrated server; preserve normal item use and existing
     Minecraft targets. Log hit position, face, chosen block cell and acceptance
     or rejection. Handle stale data and world/origin changes without leaving a
     usable stale target. Start with ordinary solid blocks; bucket targeting is
     a follow-up because buckets perform their own targeting query.
   - **6a first implementation (0.7.0):** only aim at Skyrim terrain while holding
     a BlockItem. Keep nearer vanilla targets. Map the hit to the first whole
     block cell outside the surface along its normal; this can leave a gap of
     less than one block on uneven terrain. Do not create invisible support
     blocks. Recheck air-target placement against fresh terrain on the local
     server using the click location, reach and intervening vanilla geometry.
     Use the latest valid snapshot on each side and log its identity; the server
     may reject a target if the sampled surface changes between aim and click.
     Reject expired targets. Leave normal placement rules and item consumption
     to Minecraft. Log SCENE_TARGET and SCENE_PLACE decisions/results. Buckets,
     terrain mining and fluid containment are outside this first increment.
   - **6a targeting correction (0.7.1):** a shallow aiming ray can miss the cell
     directly above the terrain contact after it is rounded up to the block grid.
     Instead intersect that same ray with the outward grid plane first, then use
     the cell at that intersection. Keep the original terrain contact for reach
     and occlusion checks. Log missed terrain, unusable grid targets, nearer
     vanilla hits and occupied target cells, including eye/reach/distance.
     Preserve the four-block collision-feed radius and normal placement rules.
     This fixes a geometric target rejection; it does not claim to resolve every
     terrain-overlap or vanilla placement failure. Gameplay must be checked.
   - **6a blocker after 0.7.1:** user logs confirm the grid-plane mapping pulls
     destination cells toward the eye. Both placement iterations are insufficient;
     stop further variations per AGENTS.md and record STATUS.md. Next design must
     keep terrain contact, destination cell and packet click point separate,
     validating the actual contact on the server without moving the cell merely
     to satisfy packet bounds. Dedicated placement requests and actual terrain
     normals are proposals requiring design before implementation, not completed
     features. Normal Minecraft placement remains user-confirmed.
   - **6a contact-preserving placement (0.7.2):** user resumed after the handoff.
     Keep the destination cell directly outside the actual contact, retaining its
     two tangential grid coordinates. Clamp a separate click point into that cell;
     never use that point to reconstruct the terrain ray. Because only an offline
     integrated server is supported, use a bounded in-process intent mailbox rather
     than change Minecraft's network protocol. Client BlockItem.useOn registers an
     immutable intent after local validation: player/server/level identity, hand,
     cell, face, click, actual surface, sender/epoch and expiry. Server consumes one
     matching intent and independently traces to the actual surface with fresh data,
     checking reach, origin identity, cell/face agreement, vanilla obstruction and
     terrain occupancy. At most 16 intents, expiry 500 ms; clear on bridge reset.
     Normal item-use packets still own prediction, acknowledgement and inventory
     changes. No second placement operation is issued. Client candidates are tied
     to the exact synthetic hit and expire; ordinary Minecraft placement remains
     on its existing path. Log intent IDs, actual contact and packet point separately.
     This localized placement change retains sampled box faces and radius four;
     true surface normals, buckets and fluid simulation remain separate work.
   - **6b filled bucket targeting (0.7.3):** the user accepts current placement
     for this iteration and asks to continue, while reporting large-looking blocks
     and short-feeling reach. Record scale/reach for later measurement; retain
     the shared 70 Skyrim units/block mapping and normal reach in this increment.
     Extend contact targeting to ordinary water/lava buckets. For an empty target
     cell, use the existing offline intent validation, then call Minecraft's bucket
     emptying operation at that exact cell. Return Minecraft's normal success and
     empty-bucket replacement so prediction/inventory stay in its normal item-use
     path. Leave empty buckets, mob/powder-snow buckets, ordinary Minecraft targets
     and waterlogging on vanilla behavior. Log bucket placement results with fluid
     state and the preceding placement intent. Do not create invisible supports:
     water can still flow through Skyrim terrain until the next simulation step.
     Build and check both fluids, survival empty-bucket return, creative retention,
     pickup of Minecraft source water, normal block placement and failed requests.
   - **Then, fluid interaction with terrain:** teach Minecraft's fluid simulation
     which cells/faces Skyrim terrain blocks, on both client and server. Movement
     collision alone does not supply that information. Decide how sampled partial
     surfaces map onto Minecraft's grid before implementation; log blocked flow
     decisions and handle snapshot expiry without silently draining fluid through
     unloaded terrain. Rendering fluid differently cannot fix its simulation.
7. **Handle world changes and saves.** Add doors/interiors, water, time/weather,
   and matched saves so loading Skyrim restores the matching Minecraft state.

Step 2 is implemented; the user's live logs now confirm successful Skyrim loading.
Step 3 adds sampled collision export
and NPC position/body boxes. It does not add camera control, rendering
composition, combat, or save sync. Live logs confirm matching collision/NPC
transport and collision-query matches on both Minecraft sides. The user verified
movement stops at a Skyrim wall and NPC, and jumping/landing work. General
coverage still needs verification. Step 4 implements look-direction transfer
using Minecraft's window and controls. Skyrim-window input forwarding is a
separate later part. The user now confirms Minecraft mouse look turns Skyrim's
camera. Version 0.5.0 implements step 5a and builds successfully; its live
rendering has an initial user report: first-person arms/punching, held items,
HUD and inventory appear inside Skyrim's window, confirmed by the user.
Minecraft's world view is black and third-person rendering is absent because
5a skips level drawing. Minecraft's own terrain collisions remain active;
Skyrim collision shapes supplement them. A superflat practice world reduces
terrain obstacles but retains its floor. Basic composition passed the user's
gameplay check; numerical image-log comparison remains pending. The user
authorized proceeding with 5b. Version 0.6.0 builds and packages the nearby
static-block renderer. The user confirmed blocks appear and can be removed,
stay fixed while moving, are hidden correctly by Skyrim walls, and older features
still work. The user corrected the initial three-block disappearance estimate to
roughly eight blocks; 0.6.1 logs confirm 16-cubed export bounds. Breaking crack
overlays are implemented for full opaque cubes in 0.6.2. The user reports expected
gameplay; the 20261008-084544 logs confirm matching transfer/draw counts and stages
0–9 with clearing. The user confirmed 0.6.3 dropped-item icons spin, can be picked
up and disappear. Version 0.6.4 adds default-state models for supported full
opaque block drops; the user confirms items and blocks appear as they should.
Numerical mixed model/icon comparison remains unverified. Other full item models
remain later work.

## Next priority: Skyrim terrain with an empty Minecraft simulation world

### Required behavior

Skyrim supplies the landscape: its ground, mountains, caves, walls and NPCs.
Minecraft supplies movement rules, inventory and blocks/fluids the player adds.
Generated Minecraft terrain must never restrict travel or appear in Skyrim.
Player-placed blocks should still be solid and removable. Minecraft remains
movement-authoritative; the fix must not switch to Skyrim movement or noclip.
This takes priority over more rendering, fluid containment and scale calibration.
The current behavior of keeping generated Minecraft collision is an unfinished
practice setup, not the intended final behavior.

### The two halves

- **Minecraft:** use a dedicated, persistently saved empty bridge world. Generate
  genuinely empty chunks: no ground/platform, bedrock, caves, structures, natural
  fluids or generated mobs. Load chunks around the player as usual. Only player
  construction and intentional gameplay objects exist in it. Keep vanilla movement
  and player-built block collisions, with Skyrim shapes added on client and server.
  This avoids trying to distinguish generated stone from player-placed stone in
  an existing save and fixes targeting, fluids and rendering reading that terrain
  as well as movement. A superflat world still has a blocking floor and is not the
  solution. Hiding terrain or filtering only collision queries is insufficient.
- **Skyrim:** remain the source of real terrain and continue following Minecraft.
  Export collision ahead of the player's movement. Supply a stable identity for
  the exterior world or interior cell so constructions stay associated with the
  correct place rather than following a temporary connection anchor.

### Data between them

Retain position/look, terrain/NPC shapes and rendered Minecraft geometry. Add
explicit collision coverage and freshness, stable Skyrim world/cell identity and
an acknowledged alignment generation. Minecraft reports bridge-world readiness.
Movement resumes only when both sides agree on the world and coordinate mapping
and collision coverage is ready. User construction is saved against stable Skyrim
locations; it must not move when either game reconnects or changes origin.

### Build order

1. **Create and gate the empty bridge world.** Add a dedicated empty-world setup
   and identify it in saved metadata. Create a separate save; do not delete or
   convert the user's existing Minecraft worlds. Refuse active passthrough in a
   normal generated world with an explanatory log/message, so hills cannot silently
   become a second landscape again. Do not add a spawn platform. Hold the Minecraft
   player at startup/reconnect until Skyrim is aligned and a fresh collision
   snapshot covers the spawn; otherwise the empty world would cause falling.
   Suspend bridge simulation safely on lost coverage rather than resume ordinary
   gravity into the void. Keep ordinary Minecraft worlds unchanged when bridge
   mode is inactive. Start with one Skyrim location and local walking/jumping.
2. **Make collision coverage support travel.** Publish the region actually sampled,
   not merely a recent timestamp. Refresh ahead of movement, account for speed,
   jumping and drops, and maintain complete coverage of the player's swept path.
   Do not treat a missing/unloaded region as air. Briefly hold movement on missing
   coverage and log why; improve sampling/budgets so normal walking does not hitch.
   Validate a route with slopes, cave entrances, stairs and drops. Existing sampled
   collision can miss thin objects; record and fix those failures independently
   of removing Minecraft terrain. Larger coverage needs capacity/timing measurements.
3. **Keep coordinates and saves stable over Skyrim travel.** Bind the bridge save
   to the Skyrim world/cell identity. Use a bounded local Minecraft coordinate
   window, with persistent construction keyed to stable Skyrim coordinates. Before
   Minecraft height/world limits are approached, pause, save/remap construction and
   entities into a new window, and atomically update position, collision, rendering
   and placement origins on both sides. Resume only after acknowledgement and fresh
   coverage. Treat interiors/doors/load transitions the same way, with isolated
   construction per location. Do not let Minecraft's void threshold/world border,
   or reconnect anchoring, restrict Skyrim altitude or shift placed blocks. Define
   and log this mapping before implementing it; do not simply teleport the player
   while leaving blocks or collision behind. Matched loading must restore the
   appropriate bridge state, with backups before any migration.
4. **Validate construction and then resume the feature plan.** Confirm blocks
   placed in the empty world collide, render, break, drop and persist correctly.
   Check reconnects, long travel, altitude changes and return visits. Then implement
   fluid containment against Skyrim geometry and measure scale/reach using known
   distances. Removing generated terrain does not itself make fluids recognize
   Skyrim walls or change the 70-units-per-block mapping.

### Acceptance and logging

- Log bridge save identity, empty generator, aligned world/cell/generation,
  collision bounds/age, hold/resume reasons and both position read-backs.
- At creation, log generated non-air counts for representative loaded chunks;
  expect zero before construction. After building, expect only intentional content.
- Traverse a Skyrim route in the bridge save: there must be no Minecraft mountain,
  cave roof, bedrock floor, generated water or invisible generated block stopping
  movement. Skyrim walls should still stop it. A deliberately placed Minecraft
  wall should stop it until broken.
- Test Skyrim grounding before enabling free movement, then coverage loss,
  reconnect, altitude/window transitions and construction persistence. Do not
  claim unrestricted travel from one successful flat-ground test.
- After implementation, run `.\tools\install-build.ps1` with both games closed
  (three INSTALLED lines), perform the documented bridge-world setup and route,
  then `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Add the new bridge
  diagnostics to that collector as part of implementation. Exact setup controls
  will be specified with step 1; they do not exist yet.

Step 1 is implemented in 0.8.0 and awaits gameplay verification. Later travel and coordinate-window steps remain plans. All new gameplay is **not tested**; see README for the exact installation and empty-world procedure.

### Investigating the short block range (0.6.1)

The 0.6.0 run kept submitting meshes (2,204 draw records), with only one startup
depth wait. That does not prove the GPU displayed their pixels. Before changing
occlusion, add diagnostics in this order: log Minecraft's exported bounds and
player position; sample Skyrim's centre-screen depth; log how the camera matrix
maps known distances into depth alongside the near/far calculation. Build a
diagnostic package, then reproduce the cutoff while looking at one placed block
and walking backwards. Compare whether the block leaves the exported bounds or
is rejected by the depth comparison.

Result: the user's corrected eight-block observation matches the export design.
The 20261008-081526 report has 1,443 mesh snapshots and one atlas matching across
the bridge, 317 usable depth probes, and a maximum matrix-versus-formula depth
difference of 0.00000114 at 500 Skyrim units. No cutoff fix is justified by this
run. Movement and meshes use the same 70 Skyrim units per Minecraft block;
placement reach and perceived scale still need a separate assessment. The cube
is centred on the player's integer block position, so this is an approximate
per-axis range, not an eight-block spherical distance.

Defer the reported camera orbit and Minecraft-world bleed at the user's request.
The full Skyrim log records releases and new alignment anchors during this run;
some change the world's offset. Their triggers and relationship to the reported
bugs are not established. Record this as a later investigation lead.

The package build will install its matching Minecraft JAR and both DLL copies
into the game folders listed in AGENTS.md. A standalone install script also
installs an existing package without rebuilding. It requires the games to be
closed, backs up this mod's previous files inside the ignored project `dist`
folder, checks copied files by checksum, and removes older passthrough JARs.
Other mods are left alone. A package-only switch skips installation.
The installer identifies Minecraft separately from unrelated Java tools and
checks access to existing destination files before copying. It does not stop
processes; both games must stay closed during installation.








### Empty bridge step 1 implementation plan (0.8.0)

Add a New Skyrim bridge button on the single-player world list. Create a unique
new save via Minecraft's world-creation API, with a persistent name marker and
an all-air flat generator: 128 air layers (spawn height 64), void biome, no
structures, decoration or lakes, no bonus chest. Keep other dimensions inaccessible
for bridge play in this first version. Recognize the marked save plus generator
settings before sending active position samples. Ordinary saves remain ordinary
Minecraft, with a clear inactive-bridge message. Do not modify their generation.

Keep sending stationary active samples while waiting in the marked bridge world
so Skyrim can establish alignment and export collision; otherwise startup would
deadlock. Hold player movement on client and server until a fresh complete snapshot
covers the player's body and short movement path. On first entry require a terrain
support surface within one block below the feet. After initial readiness, permit
jumping/falling within coverage; hold again on expiry or loss of coverage. Cancel
queued server move packets while held as well as entity movement; clear velocity
and accumulated fall distance. Report hold/resume reasons in logs and on screen.
This is a conservative local gate, not the ahead-of-travel sampler from step 2.

Publish the existing scan centre and radius in reserved world header bytes so
coverage is explicit, retaining the scan density/capacity. Increment protocol to
13 and world magic to PTWORLD4; require matching DLLs/JAR. Log initial generated
chunk non-air counts and bridge save/generator identity. Retain player-placed block
collision normally. No conversion/migration of existing saves, no construction
transfer, no unlimited altitude, no stable Skyrim save/cell mapping in this step.

0.8.0 source implementation includes all-air world creation, ordinary-world gating,
local coverage/startup holding and diagnostics. Runtime behavior remains not tested.
The first compile failed because displayClientMessage was removed in Minecraft
26.3; it was replaced with the inspected sendSystemMessage/sendOverlayMessage APIs.
The collector already includes all SCENE_BRIDGE_ diagnostics. Reuse the generated
bridge save for local tests, but do not claim stable Skyrim save/reconnect mapping
until the later coordinate/persistence step is implemented.

