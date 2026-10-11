# 0.10.56 left-click tests passed; chair and healing reopened

User reports **all .56 tests working**: inventory use/equip, taking, held click,
category targeting and favourites. Logs report15 native item-use clicks and2
pointer clicks; no errors. Container pass is user-reported, not independently
present in this session's logs. Conversation/buying acceptance is retained.

Chair exit and Minecraft healing remain failed. User requests one more attempt
for each after the previous two failures; permission is in AGENTS.md. Diagnosis
first, separate repair steps; no new repair build/test instruction yet.

# 0.10.56: inventory left-click selection repair ready to test

Conversation clicks and buying are **passed** from your .55 report. Inventory
left-click take/equip/use **failed**: all52 logged clicks selected a rendering
menu above the inventory, so the previous item-use repair never ran.

.56 corrects that selection and includes container inventories. Built and
packaged; gameplay **not tested**. Checks:77 Rust tests,113 Minecraft rules,
136 hook checks passed. Protocol33; no install or commit.

Close both games and run from D:\Documents\ChatGPT\Game modding:

```powershell
.\tools\install-build.ps1
```

Expect three INSTALLED lines, including Fabric0.10.56. Start both games normally.

1. **Inventory (I):** left click a highlighted usable food/potion. Its Skyrim
   count should drop by one. Left click equipment: its equip mark should change.
   Hold left click for a second: no repeated consumption. E should still work.
2. **Target selection:** switch categories, then click a different item. The
   intended item should be used; category clicks should navigate without using
   the previously highlighted item.
3. **Taking:** open a chest/container and left click an item. It should transfer
   to your inventory once; verify the item/count moved.
4. **Favourites (G):** left click an already-favourited consumable or equipment.
   Expect one use/equip. Judge Skyrim count/equip mark, not Minecraft healing.

No need to repeat the long conversation/shop/chair sequence or accepted movement,
door and bed tests. Chair exits and Minecraft potion healing remain unresolved;
workbench and earlier deferred checks remain not tested. Door facing is future work.

Rollback: `item_click=off` in project switches.txt, restart both games. This
restores generic Click. Logs should now show inventory/container/favourites
`route=item_use`, and `passive_skipped` when a renderer was above the input menu.
Those logs prove the route, not item consumption. .56 is attempt2 of2: if it still
fails, the next action is a failure handover under AGENTS.md, not a third guessed
repair. Stock mouse mappings/custom menu changes are not tested.
Older sections below are historical.

# 0.10.55: left-click item-use repair ready to test

Built and packaged; gameplay **not tested**. The bridge previously sent a generic
pointer click in every menu. This build sends Skyrim its native use action in
inventory and favourites, while other menus keep pointer clicks. Pointer movement
is sent before use. E still confirms the highlighted item.

Checks passed: 76 Rust tests, 113 Minecraft rules, 136 hook checks. Both packages
are in dist/step5b, protocol 33. No installation or commit.

Close both games and run from D:\Documents\ChatGPT\Game modding:

```powershell
.\tools\install-build.ps1
```

Expect three INSTALLED lines, including Fabric 0.10.55. Start both games normally.

1. **Inventory (I):** hover/highlight a food, potion or equipment item you can
   already use with E. Left click: expect that item to be used/equipped once;
   a consumable count should decrease by one. Hold left click for a second:
   expect no extra uses. Select another item and check that it is the one used.
   Click an inventory category too: it should navigate, without consuming an item.
2. **Favourites (G):** left click an already-favourited consumable or weapon.
   Expect one use/equip, without needing E. E should still work.
3. **Dialogue, if available:** click one choice under the pointer. Expect the
   selected choice to work. No need to repeat the whole shop/chair sequence.

Judge the native item count or equip mark. Minecraft potion healing is still
unresolved and is not the pass condition for this test. Chair exits remain
unresolved; the accepted door/bed/movement checks need no repeat. Workbench and
previously deferred checks remain not tested. Door facing is future work.

Rollback: add `item_click=off` to switches.txt in this project and restart both
games; this restores the previous generic click route. New click logs report
inventory/favourites `route=item_use`, other menus `route=pointer`. The optional
menu-click binding uses the same native left mouse action. Game/menu mods or
remapped Skyrim mouse buttons are not tested; this repair uses stock left mouse
use events. Older sections below are historical reports/instructions.

# 0.10.54 gameplay report: door and owned bed passed; chair and healing failed

Latest user report, 2026-10-11:
- Passed: leaving the building without a later teleport; owned-bed grounding;
  holding the E that entered a chair keeps the character seated.
- Failed: lower-chair exit with a table ahead and high-stool exit with an NPC
  nearby still fall through the ground. Their involvement is not proven.
- Failed: potions still do not heal Minecraft. Left click no longer uses items;
  E works. The logs confirm click delivery, not successful item use.
- Not tested: workbench, separate native-hand visibility outcome, and previously
  deferred J/K/M/N/O. Previously accepted movement/menu/combat checks stay accepted.

Chair exit and healing are stopped after two unsuccessful approaches under
AGENTS.md; the first section of STATUS.md contains the failure handover and evidence.
No new build or install is requested. Sources/packages remain 0.10.54, protocol33.
74 Rust tests pass; this does not verify the failing gameplay. Facing away from
an exit door is recorded for a future update, not implemented.

Collected evidence is in logs/scene-report-20261011-221546. To read its summary:

```powershell
.\tools\check-test-report.ps1 -Report .\logs\scene-report-20261011-221546
```

Expected: six blocked terrain destinations, one installed restore hook, zero
restore requests/captures/applications, and twenty dispatched mouse clicks.
The older test instructions below describe previous builds, not pending retests.

# 0.10.54 repair build: ready for focused gameplay testing

**Built; gameplay NOT TESTED.** 74 Rust tests, 113 Minecraft rule checks and
136 hook checks passed. Both packages are in dist/step5b. No installation or commit.
Protocol remains 33; use the two packages together.

Your latest report accepted creative movement (A), E-to-stand (C), focus/menu
seating (E), seated save/reload (F), close combat movement (H) and obstructed/empty
bow shots (I). Leaving after the original sequence worked, but the subsequent
outside teleport is a failure. Lower chair exit fell through the world; native
hand appeared. Owned bed initially rejected as airborne. Potions did not heal.
Workbench, actual shout, continuous damage, sensitivity/wait, height/depth and
other combat variants remain not tested/deferred. Do not repeat accepted tests.

The repairs, in separate builds:
- 0.10.51 checks terrain before a recovery teleport, avoiding feet inside the
  sampled floor at chair exits and door arrivals. Bounded upward correction only.
- 0.10.52 lets Skyrim land before activating a bed/crafting station and aligns
  the exit afterwards. Existing Skyrim ownership and interaction rules apply.
- 0.10.53 captures requested healing before Skyrim's full-health clamp, and keeps
  that request while using a live paired inventory/favourites menu.
- 0.10.54 hides Skyrim's first-person arm model while bridge view is active and
  restores its prior visibility on release. Camera height itself is unchanged.

Close both games. From D:\Documents\ChatGPT\Game modding run:

```powershell
.\tools\install-build.ps1
```

Expect three INSTALLED lines, including Fabric 0.10.54. Start both games normally.

**1. Lower chair:** use the chair that failed. Sit with E, then try WASD to stand;
repeat with E to stand. Expected: stay above the floor and resume normal walking;
no Skyrim hand in the bridged view. Inspect the view both seated and after standing.
A held entering E means keeping E pressed after sitting; you should remain seated
until releasing it and pressing it again. That subcheck was not confirmed before.

**2. Door exit:** leave the meadery through its front door, release all movement
keys and wait ten seconds. Expected: remain at the exterior exit without falling
or another unrequested teleport. Re-enter and leave once more. The full earlier
shopkeeper/menu/shout sequence need not be repeated just to check the exit.

**3. Bed and optional workbench:** with Minecraft active, E on the same NPC-owned
bed. Expected: ownership rejection, without pausing Minecraft to land and without
an airborne message. If an available bed permits sleep, complete/cancel it and
check movement afterwards. Workbench is still untested: if you try one, open its
menu, use or cancel it, then check movement/doors afterwards.

**4. Potion:** with Minecraft hearts missing, use a Skyrim health potion through I,
close the menu and check hearts. Repeat through G/favourites. On one try leave the
menu open ten seconds after drinking before closing it. Expected: hearts recover
when gameplay resumes; the potion is not lost just because the menu stayed open.

Report which of 1-4 passed/failed. I will collect logs directly. Older deferred
J/K/M/N/O are still optional and are not prerequisites for these repairs.

Fallback switches (switches.txt in the project folder; restart both games):
`safe_arrival=off`, `furniture_ground=off`, `healing=off`, `native_hands=off`.
Each restores the behavior for that individual step. No switches are required
for the normal test.

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

# 0.10.50 ready: creative movement and intentional sitting

Built, **gameplay not tested**: 66 Rust tests, 113 Minecraft rule checks and 136
hook checks passed. Both 0.10.50 packages are in dist/step5b. Protocol 33; install
all three files together. Codex did not install or commit anything.

## What changed

- Creative flight/falling (0.10.49): taller collision coverage, steadier movement
  prediction and more rays per frame within the existing two-millisecond budget.
  Flight permission keeps tall coverage after flight is switched off. The grid
  still uses half-block spacing; movement still waits for fresh scanned geometry.
  0.10.50 also preserves slower scan pacing while standing still in creative.
- Chairs (0.10.50): E sits; Skyrim owns the chair animation and position while
  Minecraft stays still. There is no four-second forced stand during intentional
  sitting. WASD or E requests standing; Minecraft aligns with Skyrim's exit
  position before walking resumes. E while seated stands you up first, rather
  than activating the object under the crosshair. R uses the same seat mode.
- Existing seated saves still have the 0.10.48 timed recovery. It actually stood
  the character up in the last logs, and the front door then worked.

Your flat-ground sprint-jump test passed on the ground tested; no repeat required.
The original enter / shopkeeper dialogue and menus / shout / chair / leave
sequence is **not tested**. Workbenches and beds are **not tested**. They were
regression checks because these share furniture records with chairs and may use
sit/stand animations. They are not needed to demonstrate the movement repair or
the original door sequence; they do not gate the focused tests below.

## Test this build

Close both games. From D:\Documents\ChatGPT\Game modding run:

```powershell
.\tools\install-build.ps1
```

Expect three INSTALLED lines, with Fabric 0.10.50. Start both games as usual.

**A. Creative movement:** outdoors in Minecraft creative, fly upward, move/turn
while flying, then turn flight off and fall. Expected: no brief interruptions in
ascent or descent. Report where any interruption happened; the logs will show
coverage, scan size and time. Dense terrain and actual frame costs are not tested.

**B. The original door sequence:** start outside Honningbrew Meadery, enter,
talk to the shopkeeper, navigate the same menus, try the Skyrim shout (Z), then
press E on the chair. Stay seated for at least ten seconds. Expected: you remain
seated. Press WASD to stand, walk to the front door and press E. Expected: you
leave. The key-locked boilery door is not this front-door test.

**C. E to stand:** sit with E again, release E, then press E again (you can look
away from the chair). Expected: you stand, Minecraft's inventory does not open,
and walking resumes once the positions align. There should be no automatic
stand-up before that second press. Report whether B and C both worked.

No need to repeat already accepted flat-ground jumps or the other deferred tests.
Beds/workbenches remain unverified; report their behavior whenever you use them.

## Reversible switches

Add a line to switches.txt in this project, then restart both games:

| Line | Effect |
| --- | --- |
| fast_scan=off | Use the previous scan shape, look-ahead and scan pace |
| sitting=off | Return to guarding chairs and timed recovery; E opens inventory on a chair |
| stand_up=off | Disable automatic recovery of an unowned seated save/mount; deliberate WASD/E exit remains |

Riding, mining and wood chopping remain unsupported. These are separate from
ordinary chairs, stools and benches with a can-sit marker.

After a report, Codex collects/checks the logs directly. The report checker now
counts SCENE_SITTING outcomes and skyrim_seated movement holds. These new modes
and their animations/camera are **not tested in gameplay**.

# Repairs 0.10.46 to 0.10.48: sprint-jumps and seated Skyrim characters

0.10.46 and 0.10.47 were built by Claude Code; Codex completed and built
0.10.48. Checks passed: 55 Rust tests, 99 Minecraft rule checks, 136 hook
checks. Both 0.10.48 packages are ready in dist/step5b.
Gameplay of these three repairs is **not tested**. Protocol stays 31; use all
three installed files from the same package version. Nothing was committed.

## What changed

- **Sprint-jump freezes (0.10.46):** the collision cube no longer aims too far
  down after a jump. Vertical look-ahead is 0.75 block up to 13 blocks/second,
  rising to 1.75 at 19 blocks/second; sideways stays 2.75. The new jump test
  fails with the old 0.10.41 limit and passes with this correction.
- **Seats (0.10.47):** E and R leave Skyrim chairs, stools, benches and other
  furniture without a menu alone. E opens Minecraft's inventory there. Beds
  and crafting workbenches remain usable. Logs now identify the target,
  its lock and Skyrim's sitting state for each forwarded press.
- **Recovering a seated save (0.10.48, experimental):** after four continuous
  seconds non-standing while the bridge moves the character, ask Skyrim to
  stop using the seat or mount. Try again every two seconds, five calls at
  most; if still non-standing two seconds after the last call, log one
  gave-up outcome. A menu, pause, load, disconnection or normal standing
  resets the timer. No forced call is made during a menu. The new routine's
  behavior in the real game is **not tested**.

The meadery door failure fits a character left seated after using the bar
stool. That cause is still **not proven**; the new logs will distinguish a
seated character from a locked door or another problem.

## Reversible switches

In switches.txt beside this README, add the relevant line and restart both
games. No file, or no line for the step, means it is on.

| Line | Effect |
| --- | --- |
| seat_guard=off | Restore the previous chair/stool/bench activation behavior |
| stand_up=off | Disable automatic stand-up attempts if they cause a problem |

The furniture offsets used by seat_guard have not been verified in Skyrim
1.7.104. If beds or workbenches stop working, report it and use seat_guard=off.
Sitting, riding, mining and wood chopping from Minecraft are still unsupported.

## Test 0.10.48

From this project folder, close both games and run:

```powershell
.\tools\install-build.ps1
```

Expect three INSTALLED lines, with Fabric 0.10.48. Start both games as usual.
Use a copy of your save for the experimental recovery test; keep the original
seated quicksave and the earlier entry autosave so either remains available.
Report A through D as passed or failed, and which save was used for B.

**A. Sprint-jumps:** outdoors, sprint-jump about 200 blocks. Expected: no short
freezes at the start of a jump. The client log should have no outside_bounds
holds during this run.

**B. Seated quicksave and the front door:** load the last quicksave inside
Honningbrew Meadery. With both games connected and normal play active, wait
about five seconds before pressing E on the front door. Expected: Skyrim's
character is standing and you leave the building. If it still fails, load
the autosave from entering the meadery and try the front door there; report
both outcomes. The boilery's locked door is not this test. If recovery makes
the game misbehave, stop, add stand_up=off and restart both games.

**C. Chair:** aim at a chair and press E. Expected: Minecraft's inventory
opens, Skyrim's character does not sit. Close the inventory and press R on
the chair: expected, no activation.

**D. Workbench and bed:** use a workbench, spend several seconds in its menu,
then close it. Use a bed and close the sleep menu. Expected: both menus still
work, and returning from them does not produce a forced stand-up attempt.

Then run:

```powershell
.\tools\collect-scene-logs.ps1
.\tools\check-test-report.ps1
```

The Seats section now counts stand-up before/after calls, gave_up and disabled.
For B, the desired evidence is SCENE_SEAT returning to state=0, or an after
line with sit=0, followed by the door transition. A before line without an
after line is important if the game stops inside the experimental routine.
There is no need to repeat the other tests you deferred.

# Versions 0.10.37 to 0.10.45: movement fixes, an optimisation, bows, more keys

Built successfully; gameplay **not tested**: nothing in this batch has been
played. Before the build ran: 45 Rust tests, 99 checks of the Minecraft side's
own arithmetic, 136 checks that every hook into Minecraft matches Minecraft's
own code. Made by Claude Code. Install 0.10.45; all three files together
(protocol 31, as before).

## What changed

**The two problems you reported**
- **The pause when moving fast** was not the 8-block limit. The logs show it
  happened each time you crossed one of Skyrim's map cells, every 58 blocks:
  the bridge let go of Skyrim's character and started over, and Minecraft held
  you for about a tenth of a second. Crossing a cell outdoors no longer lets
  go (0.10.37). The real 8-block case, moving faster than a sprint (flying, a
  speed effect, a long fall), is eased too: the collision cube now looks
  further ahead (0.10.41).
- **Being blocked, very slow and unable to jump.** This did not happen in the
  session I have logs for, so the cause is worked out from Minecraft's code,
  not seen. Minecraft's own local server undid your steps whenever a Skyrim
  shape appeared where you had just stepped, and Minecraft forces a crawl when
  a shape overlaps your upper body. The shapes that move are the invisible
  stand-in boxes for Skyrim's characters, and dead bodies had one too. Now:
  the local server no longer checks your steps against Skyrim's shapes
  (0.10.38), Skyrim's shapes never force a crouch or crawl, and dead bodies
  have no box (0.10.39). If it still happens, the log now records it.

**Optimisation**
- **Standing still costs Skyrim about a quarter of what it did** (0.10.40).
  The collision scan took about 1.1 ms of every Skyrim frame, moving or not.
  When nothing around you changes it now runs at a quarter of the rate.

**New**
- **More Skyrim menus:** I opens Skyrim's items, P its magic, G its favourites
  (0.10.43).
- **Bows work on Skyrim enemies** (0.10.44). Draw and release with the
  crosshair on an enemy up to 60 blocks away. No arrow is seen flying and it
  does not drop: the hit lands where the crosshair points when you let go.
  Walls and ground stop it. Minecraft uses up the arrow as usual.
- **Skyrim's shout or power on Z** (0.10.45, experimental). A tap only.

## If something goes wrong: more switches

Same file as before: `switches.txt` next to README.md, one line per step,
restart both games. New lines for this batch:

| Line | What it turns off | Use it when |
| --- | --- | --- |
| `server_trust=off` | The local server trusting your steps | You get pulled back or stuck worse than before |
| `pose_free=off` | Never being forced to crouch or crawl | You stand inside things you should duck under |
| `slow_scan=off` | The slower scan while standing still | Doors or moving things block you after they moved |
| `ranged=off` | Bow shots reaching Skyrim | Shots hit the wrong thing or Skyrim misbehaves |
| `shout=off` | The Z key | Pressing Z does anything bad |

The earlier lines still work (`hostility`, `e_precedence`, `menu_keys`,
`menu_wasd`, `damage_pool`, `pause_shield`, `healing`, `pointer_monitor`,
`void_guard`).

## Test 0.10.45

Close both games and run `.\tools\install-build.ps1`. Expect three INSTALLED
lines with Fabric 0.10.45. Start both games as usual.

Say which letters passed or failed; I will read the logs myself. If a letter
fails, carry on with the others. A to D are the ones that matter most.

**A. Start-up and what already worked**
You get control, can walk, place and break a block, hit an enemy with a sword,
open a chest or a body with E.

**B. Moving fast**
1. Outdoors, sprint in one direction for at least 200 blocks (a road is
   easiest), then sprint-jump the same stretch. Expected: no short freeze
   every 60 blocks or so.
2. If you know a spot where it kept freezing, walk back and forth there.
   Expected: no freeze.
3. Optional, in creative: fly fast. Expected: fewer and shorter freezes than
   before; some may remain.

**C. Blocked, slow, cannot jump**
1. Fight one or more enemies up close, also on a slope or stairs, and jump
   while they are on you. Expected: you are never slowed to a crawl and you
   can always jump.
2. Walk across the bodies afterwards. Expected: nothing invisible in the way.
3. If you do get blocked or slowed: note roughly what was next to you (an
   enemy, a body, a doorway, a low ceiling) and tell me. That is the one thing
   the log cannot tell me by itself.

**D. Standing still**
1. Stand still for half a minute, then walk on. Expected: nothing unusual.
2. Stand right in front of a closed door that opens without a loading screen,
   open it with E, wait a second, walk through. Expected: you get through. If
   the open doorway still blocks you after a second, that is this step.

**E. More Skyrim menus**
1. Press I. Expected: Skyrim's items. Tab leaves.
2. Press P. Expected: Skyrim's magic. Tab leaves.
3. Press G. Expected: Skyrim's favourites, if you have any favourites set in
   Skyrim; with none, Skyrim shows nothing.

**F. Bow**
1. With arrows, draw fully (hold right mouse about a second) with the
   crosshair on an enemy 10 to 30 blocks away, and let go. Expected: the enemy
   is hurt and reacts; one arrow is used.
2. Shoot again with a short draw. Expected: less damage.
3. Stand so that a wall or a rock is between you and the enemy, and shoot at
   where it is. Expected: nothing happens to it.
4. Shoot at empty sky. Expected: nothing, apart from the arrow being used.

**G. Shout or power (experimental)**
In Skyrim's magic menu (P), equip a shout or a power. Back in play, press Z.
Expected: Skyrim uses it. If nothing happens, that is the likely outcome I
warned about; say so.

**Still open from Test 0.10.36** (skip any you do not want to do now)

**H. W, A, S, D in Skyrim menus.** In a long list press S and W; hold S.
Expected: the selection moves, and keeps scrolling while held.

**I. Continuous damage.** Flames or frost from a mage, a trap, poison.
Expected: hearts go down in steps about twice a second.

**J. Skyrim healing.** With hearts missing, drink a health potion from
Skyrim's items. Expected: hearts come back at once.

**K. Pointer speed setting.** Minecraft Options > Controls > Mouse Settings,
Sensitivity 200%. Expected: the Skyrim pointer is twice as fast.

**L. Height and depth.** The sea shore or High Hrothgar; walk into deep
water. Expected: you arrive and move; no "fell out of the world".

**M. How does it run?**
Nothing to do. Say whether either game feels smoother, the same, or worse.

Where a failure would not surprise me: G (nothing has shown that Skyrim's
player controls take a forwarded key), E3 (favourites uses a Skyrim handler
not seen working yet), and C in the sense that I fixed the causes I could find
in the code without having seen the problem happen.

## 2026-10-11 0.10.36 tested by the user and checked against the logs

User: "Pass: A,B,C,D,E,F,G seems to work, look at logs,I,K,L,M fps is
definitely higher(was 50 now 60). Did not test: H". Report
scene-report-20261011-155446 (protocol 31 on both sides, no error line, no
switches used):

- **Works, and the logs agree:** start-up (A); quicksave, quickload, E opening
  things, real clicks (B); E on characters, doors and objects, with enemies
  recognised 31 times (C); Tab, M and J opening Skyrim's menus (D); pointer
  measured against the monitor (F1, F2); Sharpness and Strength, exact to the
  digit (G); pause protection, 5 hits cancelled (I); fast travel by clicking
  the map (K1); death and reload (L); Skyrim at 59 frames a second and raw
  logs about 96% smaller (M).
- **Not tested yet** (the user confirmed this afterwards): W, A, S, D and held
  directions in Skyrim menus (E1, E2), continuous damage (H), Skyrim healing
  (J). The user will test these later.
- **Not exercised in this session:** the sensitivity setting (F3), height and
  depth (K2, K3), the wait key (D4).

No rebuild or reinstall needed. Nothing is committed: the user decided not to
commit until the rest is tested.

# Versions 0.10.19 to 0.10.36: two batches, one test list

Built successfully; gameplay **not tested**: nothing since 0.10.18 has been
played. Before the build ran: 39 Rust tests, 90 checks of the Minecraft side's
own arithmetic, and 124 checks that every hook into Minecraft matches
Minecraft's own code. Made by Claude Code. Install 0.10.36; it contains
everything below. All three files must be updated together (protocol 31).

This list replaces "Test 0.10.27" further down; the first batch's tests are
included here.

## What is new since 0.10.18

First batch (0.10.19 to 0.10.27):
- **E follows Skyrim first.** E talks, opens doors, containers and bodies,
  picks plants and items. The Minecraft inventory opens only when the
  crosshair is on nothing or on an enemy. R is unchanged.
- **Keys for Skyrim's menus.** Tab: character menu. M: map. J: journal. Tab
  inside a Skyrim menu goes back. "Skyrim: wait" has no key until you bind one.
- **Fire, frost and poison now hurt:** small hits are added up and delivered
  about twice a second.
- **A paused Minecraft protects you** from Skyrim hits.
- **Sharpness, Strength and Weakness** count against Skyrim enemies.
- **Pointer speed no longer depends on the Minecraft window size.**
- **You can arrive at any height.**
- **Far less log writing** (it was about 7,000 lines a second in each game).

Second batch (0.10.28 to 0.10.36):
- **Switches.** Any of the risky steps can be turned off with one line in a
  text file, without a rebuild. See "If something goes wrong" below.
- **Skyrim healing heals you.** Potions and food from Skyrim's inventory, and
  anything else in Skyrim that restores health, now restore your Minecraft
  hearts. Skyrim's slow natural recovery is deliberately left out.
- **W, A, S, D move through Skyrim menus,** and a held direction (those or the
  arrow keys) keeps scrolling.
- **Pointer speed follows Minecraft's Mouse Sensitivity setting.** 100% is the
  speed from the first batch; raise it to go faster.
- **No "fell out of the world".** In the bridge world you can be at any depth
  (deep water, deep caves) without Minecraft hurting you for it.
- **The bridge measures its own cost** per frame in both games, so the next
  report says with numbers whether it slows them.
- **Three kinds of automatic check now run before every build,** including one
  that would have caught a hook that stops Minecraft from starting.

## If something goes wrong: switches

Create a text file called `switches.txt` in the project folder (next to
README.md), put one line in it per step you want off, and restart both games.
Delete the line (or the file) to turn it back on.

| Line | What it turns off | Use it when |
| --- | --- | --- |
| `hostility=off` | Asking Skyrim whether a character is an enemy | Skyrim crashes or freezes when you look at a character |
| `e_precedence=off` | E on characters, doors and objects (E goes back to containers and bodies only) | E gets in the way |
| `menu_keys=off` | Tab, M, J and wait | Those keys do something bad |
| `menu_wasd=off` | W, A, S, D and held directions in Skyrim menus | Menus scroll by themselves |
| `damage_pool=off` | Adding small hits together | Damage feels wrong or stops |
| `pause_shield=off` | Protection while Minecraft is paused | You stay unhurt when you should not |
| `healing=off` | Skyrim healing reaching Minecraft | Hearts fill up by themselves |
| `pointer_monitor=off` | The new pointer speed (back to the 0.10.18 speed) | The pointer is unusable |
| `void_guard=off` | The exemption from falling out of the world | Anything odd far below ground |

Tell me which lines you used; the logs also record them.

## Test 0.10.36

Close both games and run `.\tools\install-build.ps1`. Expect three INSTALLED
lines with Fabric 0.10.36. Start both games as usual.

Say which letters passed or failed. You do not need to collect logs; I will
read them myself. If a letter fails, carry on with the others. A, B and C are
the ones that matter most; D to M can be done in any order.

**A. Start-up**
Both games start, you get control in Minecraft, you can walk, and you can
place and break a block. If Minecraft stops while loading, or the two games do
not connect, stop there and tell me what you see.

**B. What already worked**
1. Hit an enemy with a sword. Expected: it takes damage as before.
2. Press F5. Expected: Skyrim's "Quicksaving..." message.
3. Look at a chest, press R, then R again inside it. Expected: opens, then
   takes all. Right click closes.
4. Look at another chest or a body, press E, then left click one item.
   Expected: it opens, and that one item is taken.

**C. E key** (off: `e_precedence=off`; enemy part: `hostility=off`)
1. Look at a friendly character until Skyrim shows "Talk", press E. Expected:
   the conversation opens; the Minecraft inventory does not. Backspace leaves.
2. Look at a door, press E. Expected: it opens or takes you through.
3. Look at a plant or an item on a table, press E. Expected: Skyrim picks it.
4. Look at the sky or open ground, press E. Expected: the Minecraft inventory.
5. In a fight, crosshair on the enemy, press E. Expected: the Minecraft
   inventory.

**D. Skyrim's menus from Minecraft** (off: `menu_keys=off`)
1. Press Tab. Expected: Skyrim's four-way character menu. Tab again: back.
2. Press J. Expected: Skyrim's journal. Tab leaves.
3. Press M. Expected: Skyrim's map. Keep it open for test K.
4. Optional: bind "Skyrim: wait" in Minecraft's Controls and press it.

**E. W, A, S, D and holding** (off: `menu_wasd=off`)
1. Tab, choose Items, open a long list. Press S and W. Expected: the
   selection moves down and up, like the arrow keys.
2. Hold S. Expected: after a short pause the list keeps scrolling; it stops
   when you let go.
3. Leave the menus. Walk up to a character holding W and press E while still
   holding it. Expected: the conversation opens and does not scroll by itself.

**F. Pointer** (off: `pointer_monitor=off`)
1. In any Skyrim menu move the mouse. Expected: the pointer moves about like
   your desktop pointer, slower than before.
2. Resize the Minecraft window and try again. Expected: the same speed.
3. In Minecraft: Options > Controls > Mouse Settings, set Sensitivity to 200%.
   Expected: the Skyrim pointer is twice as fast (and so is the camera). Put
   the setting where you like it and tell me the value you ended on.

**G. Sharpness and Strength**
Hold a sword. Type `/enchant @s minecraft:sharpness 5`, hit an enemy a few
times. Type `/effect give @s minecraft:strength 60 1`, hit again. Expected:
enemies go down clearly faster each time.

**H. Continuous damage** (off: `damage_pool=off`)
Let something that deals steady damage hit you: flames or frost from a mage, a
fire or spike trap, poison. Expected: your hearts go down in steps about twice
a second. Before, they barely moved. Sword and arrow hits should feel as they
did.

**I. Pause protection** (off: `pause_shield=off`)
With an enemy attacking you, press Esc in Minecraft and wait ten seconds.
Expected: Skyrim's health bar does not go down. Esc again: the fight carries
on and you take damage normally.

**J. Skyrim healing** (off: `healing=off`)
1. Get hurt until a few hearts are missing. Press Tab, choose Items, then
   Potions, and drink a health potion. Expected: hearts come back at once, in
   proportion: a potion that restores a quarter of Skyrim's health restores a
   quarter of your hearts. Food works the same way but restores very little.
2. Stand still for half a minute with hearts missing. Expected: they do not
   fill up by themselves any faster than Minecraft's own recovery from food.

If you have no health potion, skip this letter and tell me; I have not checked
a way to give you one.

**K. Travel, height and depth** (depth: `void_guard=off`)
1. On the map (M), left click a discovered place and confirm. Expected:
   Skyrim travels there and after the loading screen you can move again.
2. Go somewhere the bridge used to refuse: the sea shore (Solitude or
   Dawnstar docks) or high in the mountains (High Hrothgar). Expected: you
   arrive and can move. A chat line says when blocks cannot be placed there.
3. At the shore, walk down into deep water as far as you can. Expected: you
   keep walking on the sea floor (Skyrim's water is not simulated in
   Minecraft) and are never killed by "fell out of the world".

**L. Death** (only if you have time)
Let yourself be killed. Expected, as in 0.10.8: Skyrim reloads your last save
and you can play again.

**M. Does it run better?**
Nothing to do. Say whether either game feels smoother, the same, or worse than
before. The logs now carry the numbers.

**Afterwards, optional: free 27.8 GB of old logs.** With both games closed,
`.\tools\prune-logs.ps1` lists what it would remove (it keeps the newest two
raw logs of each game); `.\tools\prune-logs.ps1 -Delete` removes them.

Where a failure would not surprise me, because the step rests on something in
the games I could not see from here: D (Skyrim reacting to a forwarded
menu key), C5 (Skyrim's answer to "is this an enemy"), I (a paused Minecraft
still drawing frames), J (Skyrim reporting a potion drunk at full health).
None of them affects the other letters.

# Versions 0.10.19 to 0.10.27: a batch of eight steps

Built successfully after 33 passing Rust tests; gameplay **not tested**: none
of this has been played. Made by Claude Code after the user asked for new
features and optimisations to be planned and worked through in one go. Install
0.10.27; it contains all of them. Each step is its own version with its own
MODLOG entry, so one can be taken out without the others. All three files must
be updated together (protocol 30).

What changed, in the order of the tests below:

- **E follows Skyrim first (0.10.19).** E now does what Skyrim's E would do with
  whatever Skyrim's crosshair is on: talk to a character, open a door, open a
  container or body, pick a plant or an item, pull a lever. Your Minecraft
  inventory opens only when Skyrim's E would do nothing: the crosshair is on
  nothing, or on an enemy. R is unchanged.
- **Keys for Skyrim's own menus (0.10.20).** Tab opens Skyrim's character menu
  (skills, magic, items, map), M its map, J its journal. Inside any Skyrim menu
  Tab goes back. A fourth key, "Skyrim: wait", has no key until you give it
  one. All four are under Controls > Key Binds > Miscellaneous.
- **Fire, frost and poison now hurt (0.10.21, corrected in 0.10.27).** Skyrim's
  continuous damage arrives as dozens of tiny hits a second, and Minecraft
  ignored nearly all of them (7 of 99 in the last measured session). They are
  now added up and delivered about twice a second.
- **A paused Minecraft protects you (0.10.22).** While Minecraft is paused or
  not the active window, Skyrim's character can no longer be hurt. Before, it
  was hurt directly behind Minecraft's back.
- **Sharpness, Strength and Weakness count (0.10.23)** against Skyrim enemies,
  with Minecraft's own numbers.
- **Pointer speed is the same at every window size (0.10.24).** It will feel
  slower than you are used to: about one Skyrim pixel per mouse unit, like a
  desktop pointer. There is no speed setting yet.
- **You can arrive at any height (0.10.25).** Doors, loads and fast travel to
  the sea shore or the high mountains are no longer refused. Blocks still can
  only be placed between Minecraft's own height limits; a chat line says so
  when you arrive outside them.
- **Much less log writing (0.10.26).** The bridge wrote one log line for every
  collision box, about 7,000 a second in each game, 2.6 GB per session. It now
  writes one summary line with a checksum instead. This should take load off
  both games; whether you can feel it is for you to say.

Things that behave differently on purpose: E on a friendly character talks
instead of opening your inventory (look away first to open it). Two enemies
hitting in the same half second now both count in full. A blow can land up to
half a second late. While Minecraft is paused you cannot be hurt at all.

## Test 0.10.27

Before you start: close both games and run `.\tools\install-build.ps1`. Expect
three INSTALLED lines with Fabric 0.10.27. Then start both games as usual and
wait until you can move.

You do not need to collect logs; say which letters passed or failed and I will
read the logs myself. If one letter fails, carry on with the others.

**A. E key**
1. Look at a friendly character until Skyrim shows its "Talk" prompt, press E.
   Expected: Skyrim's conversation opens; the Minecraft inventory does not.
   Leave with Backspace.
2. Look at a door, press E. Expected: it opens or takes you through.
3. Look at a plant, or an item lying on a table, press E. Expected: Skyrim
   picks it.
4. Look at the sky or open ground, press E. Expected: the Minecraft inventory
   opens. Close it.
5. In a fight, with the crosshair on the enemy, press E. Expected: the
   Minecraft inventory opens.
6. R on a container, then R again inside it. Expected: opens, then takes all,
   as before.

**B. Skyrim's menus from Minecraft**
1. Press Tab. Expected: Skyrim's four-way character menu. Pick "Items" with the
   arrow keys or the mouse, look around, press Tab until you are back in play.
2. Press M. Expected: Skyrim's map. Move the pointer onto a place you have
   discovered and left click it, then confirm. Expected: Skyrim travels there,
   and after the loading screen you can move again in Minecraft.
3. Press J. Expected: Skyrim's journal. Leave with Tab.
4. Optional: in Minecraft's Controls give "Skyrim: wait" a key and press it.
   Expected: Skyrim's wait menu.

**C. Continuous damage**
Let something that deals steady damage hit you: a mage casting flames or
frost, a fire or spike trap, a spider's poison. Expected: your Minecraft hearts
go down steadily, a step about twice a second. Before this batch they barely
moved. Ordinary sword and arrow hits should feel as they did.

**D. Pause protection**
With an enemy attacking you, press Esc in Minecraft and wait about ten seconds
while it keeps swinging. Expected: Skyrim's health bar does not go down.
Press Esc again. Expected: the fight carries on and you take damage normally.

**E. Sharpness and Strength**
Hold a sword and type `/enchant @s minecraft:sharpness 5`, then hit an enemy a
few times. Then type `/effect give @s minecraft:strength 60 1` and hit again.
Expected: enemies go down clearly faster each time. I will check the exact
numbers in the log.

**F. Pointer speed**
Open any Skyrim menu and move the mouse. Expected: the pointer moves about like
your desktop pointer, slower than before. Resize the Minecraft window and try
again. Expected: the same speed. Tell me if it is too slow to use.

**G. Height**
Travel to somewhere the bridge used to refuse: the sea shore (for example the
docks at Solitude or Dawnstar) or high in the mountains (High Hrothgar), by the
map from B2 or on foot. Expected: you arrive and can move. If the place is
outside Minecraft's building range a chat line says blocks cannot be placed
there, and trying to place one shows Minecraft's own height message.

**H. Everything that already worked**
Place a block and break it, hit an enemy, press F5 for a quicksave, open a
container with E and take one item with a left click. Expected: all as before.

**I. Does it run better?**
Nothing to do. Just say whether either game feels smoother, the same, or
worse than before.

**Afterwards, optional: free 27.8 GB of old logs.** With both games closed, run
`.\tools\prune-logs.ps1`. It only lists what it would remove (129 old raw log
files; it keeps the newest two of each game). To really remove them run
`.\tools\prune-logs.ps1 -Delete`. Collected reports are not touched.

Not tested by anything but reading: every step above. Two things in particular
may simply not work, because they rely on Skyrim reacting to a forwarded key or
question the way I expect: the menu keys in B, and telling an enemy from a
friend in A5. If B fails, nothing else is affected. If A5 fails, E will try to
talk to enemies; look away to open your inventory, and tell me.

## 2026-10-11 0.10.18 tested by the user and checked against the logs

User: "All tests passed." Report scene-report-20261011-101205 (protocol 29 on
both sides, no error or disabled line) agrees:
- **E key works:** Skyrim reported the crosshair kind for everything looked at
  (containers 13 times, living characters 11, bodies 4, doors 9, other objects
  2). E opened a container or body 10 times and confirmed inside a Skyrim menu
  5 times. R was used 8 times and still works.
- **Real left click works:** 25 left clicks were sent as true Skyrim clicks and
  each was carried out once.
- All 75 menu commands were sent, received and carried out exactly once. 17 of
  18 activations were carried out; one was dropped by the quarter-second
  repeat guard, as designed. 3 of 3 position recoveries were accepted.
Not reported: whether a left click on the map offers travel (the optional
item). No rebuild or reinstall needed.

Requested for later, in the DESIGN backlog: Skyrim's E should take precedence
everywhere, with the Minecraft inventory opening only when E would do nothing
in Skyrim.

# Version 0.10.18: real left click, and E opens Skyrim containers

Built successfully after 28 passing Rust tests; gameplay **not tested**. Made
by Claude Code. Install 0.10.18; it contains 0.10.17. User confirmed beforehand
that R inside a container takes everything.

**0.10.17, left click.** In a Skyrim menu, left click is now a real Skyrim
mouse click at the pointer (the same click the middle button proved), so list
entries, buttons and map markers react to where you click. Enter still confirms
the highlighted entry. One difference: a left click with the pointer over
nothing no longer confirms the highlighted entry.

**0.10.18, the E key.** E is your Minecraft inventory key. Now:
- Looking at a Skyrim container or a dead body within reach, E opens it (as R
  does), and the Minecraft inventory does not open.
- Inside any open Skyrim menu, E confirms, as in Skyrim: it takes the
  highlighted item or picks the highlighted choice.
- In every other case E opens the Minecraft inventory as before. That includes
  looking at a living character or a door, so you can still open your
  inventory in a fight. R still activates anything.
This follows whichever key is bound to "Open/Close Inventory".

How it knows: Skyrim now tells Minecraft what kind of thing its crosshair is on
(nothing, container, door, living character, body, other). That needed a small
change to the data format, so all three files must be updated together
(protocol 29).

## Test 0.10.18

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines with Fabric 0.10.18.
2. Look at a chest or barrel until Skyrim shows its prompt, press E. Expected:
   the container opens and the Minecraft inventory does not.
3. Inside it, point at an item and left click. Expected: that item is taken.
   Highlight another with the wheel and press E. Expected: it is taken. R
   should still take everything; right click should close.
4. Look at open ground or the sky and press E. Expected: the Minecraft
   inventory opens as usual. Close it.
5. Look at a living NPC and press E. Expected: the Minecraft inventory opens (R
   is still the key to talk). Look at a door and press E: same.
6. Kill something and look at the body, press E. Expected: its loot menu opens.
7. Talk to an NPC with R, then left click directly on a choice that is not
   highlighted. Expected: that choice is picked.
8. Optional: open Skyrim's map and left click a known place. Tell me whether
   Skyrim offers to travel there.
9. Tell me what happened; I will collect the logs. Expected: `SCENE_CROSSHAIR`
   lines with kind=2 on containers, kind=4 on bodies, kind=3 on living
   characters, kind=5 on doors; `SCENE_INTERACT_SEND ... source=inventory_key`
   for each E that opened something; no `event=ERROR`.
   If Minecraft does not start or the bridge never connects, stop and report.

## 2026-10-11 0.10.16 tested by the user and checked against the logs

User: test groups A, C, D and E passed; for B, the middle mouse button "acts
like left mouse in Skyrim" (items 5 to 10 were not reported one by one).
Report scene-report-20261011-094530 has no error, disabled or fallback line in
any of the three files.

**Works (tested):**
- Blocks stay visible through doors. The leftover-picture case occurred 6 times
  and was skipped each time; 6410 block draws followed, most of them indoors,
  and Skyrim's depth information was usable indoors (1310 of 1312 probes).
- Doors, interiors, fast travel and loads: 9 position recoveries requested and
  9 accepted, none refused for height. One was a jump of about 13,000 Skyrim
  units inside the open country (fast travel).
- Menu controls: 115 commands sent and each dispatched exactly once: wheel 25,
  right click 14, R in a menu 5, middle-button click 27, left-click confirm 27,
  keys 13, quicksave 3, quickload 1. The pointer speed was remembered from the
  fourth menu on (starting value 2 instead of 1).
- Damage dealt (this also verifies 0.10.5): 102 attacks sent, 90 carried out,
  11 dropped as out of range. Every Skyrim health loss matched the request
  within 0.00001. Full hits: fist 1.75, wooden sword 7; critical hits 2.625
  and 10.5 (37 critical requests).
- Weapon wear: 59 hits on one wooden sword, one point each, and it broke on
  the 59th, as a wooden sword should.
- Armour: the same Skyrim blow (67.89) cost 13.58 half-hearts without armour
  and 8.43 with 14 armour points. Shield: 2 blows blocked with no health lost.
- Death by an enemy: cause logged as a mob attack, Skyrim's character killed,
  reload, hold cleared and position recovered 7 seconds later.

**Not confirmed:** whether R takes everything from a container (it was sent 5
times; the outcome was not reported).

**Found in this report, not fixed:**
- Rapid small hits are mostly lost. Of 99 Skyrim hits under 2 damage (a
  continuous effect such as a flame stream), Minecraft took 7: 12.76 half-hearts
  requested, 0.69 applied. Minecraft refuses hits for half a second after the
  last one, and a refused hit is not kept. This predates tonight's changes.
- For about 19 seconds Minecraft was not sending to the bridge (paused or not
  the active window; Skyrim menus were opened in that time). 113 small Skyrim
  hits then went straight to Skyrim's own health (52.6 points). Known gap: the
  bridge does not protect Skyrim's character while Minecraft is not active.
- 22 hits already queued were discarded at that same moment (logged).

No rebuild or reinstall needed for this note.

# Version 0.10.16: fix for invisible blocks, plus five features (all untested)

Built by Claude Code on 2026-10-11 while the user was away. Every build ran the
Rust tests first (26 pass). **Nothing below has been played.** Install 0.10.16;
it contains every step. Each step can be reverted alone (see MODLOG.md).

What the 0.10.9 run showed (user report plus report 20261011-012218): doors and
interiors **work**. Entering a building, walking around on solid floors, placing
blocks and leaving again all functioned, and the recoveries took 47 and 31 ms.
The one failure, invisible blocks indoors and afterwards outdoors, had a single
cause and is fixed in 0.10.10.

| Step | What it does |
| --- | --- |
| 0.10.10 fix | Blocks are drawn again after going through a door. A block picture left over from the previous place is now skipped instead of switching block drawing off. |
| 0.10.11 | In a Skyrim menu: mouse wheel moves the selection, right click goes back, R is Skyrim's own R (take all in a container, drop in the inventory). New optional key "Skyrim menu: click at pointer" (middle mouse button) sends a real Skyrim click, as an experiment. |
| 0.10.12 | The menu pointer no longer starts each menu at double speed (from the second menu of a session on). |
| 0.10.13 | Critical hits: a full-strength hit while falling deals 1.5 times damage, with Minecraft's critical sound. |
| 0.10.14 | Weapons lose durability when you hit a Skyrim enemy, by Minecraft's own rule (sword 1, axe 2 per hit). |
| 0.10.15 | Minecraft armour now reduces Skyrim hits, and a raised shield blocks hits from the enemy you face. Until now neither did anything. Skyrim hits now also knock you back slightly, as Minecraft hits do. |
| 0.10.16 | Safety correction to the 0.10.11 mouse handling. |

## Complete test list for 0.10.16

Install first: close both games and run `.\tools\install-build.ps1`. Expect
three INSTALLED lines with Fabric 0.10.16. Then load the paired saves, wait
until you can walk, and press F5. When you have finished, just say so; the logs
are collected directly. If Minecraft does not start, or Skyrim crashes, stop
and report that first.

**A. Blocks after a door (the fix; do this first)**
1. Outdoors, place a block. Expected: you see it in Skyrim.
2. Aim at a building door, press R, go in. Place a block inside. Expected: you
   see it, and Skyrim walls and furniture hide it when they are in front.
3. Leave through the door. Expected: the outdoor block from step 1 is visible.
4. Go back in. Expected: the indoor block is still there and visible.

**B. Menu controls**
5. Open a container (a chest or barrel) with R. Move the pointer over an item
   and left click. Expected: the item is taken (left click sends Skyrim's
   confirm key for the item under the pointer).
6. Turn the mouse wheel. Expected: the highlighted item moves up and down, and
   your Minecraft hotbar slot does not change.
7. Press R inside the container. Expected: everything is taken (Skyrim's "take
   all"). If nothing happens, note that; it is the least certain item.
8. Right click. Expected: the container closes, and no Minecraft block is
   placed and no item used, even if you hold the button a moment.
9. Talk to an NPC. Expected: wheel moves between choices, left click picks,
   right click leaves.
10. Open any Skyrim menu twice (for example talk to an NPC twice) and move the
    pointer a little each time. Expected: from the second time on, small
    movements no longer overshoot.
11. Optional experiment: in a container or on the map, point at something and
    press the middle mouse button. Tell me whether Skyrim reacted as to a click.

**C. Combat (also covers the untested 0.10.5 damage values)**
12. Hit an enemy with a wooden sword, waiting two seconds between hits.
    Expected: about 7 health per hit (a wolf takes about four hits).
13. Jump and hit while coming down. Expected: Minecraft's critical sound and a
    harder hit (about 10.5 with a wooden sword).
14. Look at the sword's durability bar after a few hits. Expected: it has gone
    down by one per hit.
15. Put on any armour and let an enemy hit you. Expected: you lose fewer hearts
    than without armour, and you are pushed back slightly.
16. Hold up a shield facing the enemy. Expected: the hit is blocked with the
    shield sound and the shield loses durability.
17. Let an enemy finish you. Expected: as with `/kill`, Skyrim's death and
    reload, then you can move again. (Only `/kill` has been tested so far.)

**D. Things that must still work**
18. Mouse look, walking and jumping; breaking and placing blocks outdoors; F5
    and F9; arrows, Enter and Backspace in menus; punching with an empty hand.
19. Hold right click with a block in hand in normal play. Expected: blocks are
    placed as usual. (Checks the new right-click handling did not break it.)

**E. Optional, if you have time**
20. Fast travel from Skyrim's own window to a place at a similar height.
    Expected: a short hold, then movement. A chat line about height means that
    place is out of the Minecraft world's range.
21. Fight or talk to someone inside a building.

# Version 0.10.9: doors, interiors and other places

Built successfully after 20 passing Rust tests; gameplay **not tested**. Made
by Claude Code. First attempt.

Until now, going through a loading door stopped the bridge with "different
Skyrim location" until you walked back out using Skyrim's own controls. Now
every Skyrim place has its own region of the Minecraft bridge world:
- The place you have been playing in keeps its saved mapping. Nothing you
  built moves.
- Every building interior, dungeon section and walled city gets its own region,
  worked out from that place's Skyrim record ID by a fixed formula. Nothing is
  stored for it, so loading an older Skyrim save (or dying) cannot mix places up.
- Blocks placed inside a building stay with that building and are there again
  when you come back. They are not visible from outside, like Skyrim's own
  interiors.
- After a door, a fast travel or a load, the bridge holds briefly while the
  Minecraft player is moved to the matching position, then you carry on.

Limits: Minecraft is only 384 blocks tall. With your saved mapping, Skyrim
places below about -11,600 height units (including the sea) or above about
15,000 (high mountains) cannot be reached; the bridge now tells you in chat
instead of holding silently. Picking a fast-travel marker from Minecraft may
not work yet (it may need a true mouse click); use Skyrim's window for the map.
Collision, NPCs, menus, combat and death inside buildings use the same code as
outdoors but have never been run there.

## Test 0.10.9

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines with Fabric 0.10.9.
2. Load the paired saves, wait until you can walk, and press F5.
3. Walk to a building door, aim at it and press R. Expected: Skyrim loads the
   inside, Minecraft chat says it is moving you to the matching position, and
   after a short hold you can walk around inside with floors and walls solid.
4. Place a block inside, somewhere easy to recognise.
5. Aim at the door from inside and press R. Expected: you are outside again,
   you can move, and blocks you had placed outdoors are where they were.
6. Go back in. Expected: your block is still in the same spot inside.
7. Optional: fast travel from Skyrim's own window to a nearby place at a
   similar height, then return to Minecraft. Expected: a short hold, then
   movement. A chat line about height means that place is out of range.
8. Tell me what happened; I will collect the logs. Expected in Skyrim's log:
   `SCENE_ORIGIN state=derived` with the building's cell ID,
   `SCENE_RECOVERY state=request` then `state=accepted`,
   `SCENE_ALIGNMENT state=origin_ready`, and no `event=ERROR`.
   If a door leaves the bridge held for more than 30 seconds, stop and report.

## 2026-10-11 0.10.8 death and respawn verified

User: "The respawn is working now. The bridge reconnects and I can resume
gameplay." Report scene-report-20261011-005940 agrees, with no error lines:
- Minecraft death (`/kill`) sent once, with immediate respawn and keep
  inventory both on; Minecraft sent the chat line "You died. Waiting for
  Skyrim to reload its last save." (logged; not reported as seen).
- Skyrim received it 19 ms later, health 100 to -200, life state alive, then
  dying on the next frame, then dead 2.5 s later.
- Skyrim reloaded its last save 6.3 s after the hit; the hold cleared, the
  position recovery was requested and accepted within 31 ms, and Minecraft
  reported ready 7.1 s after the death.
Works (tested): dying in Minecraft from `/kill`, Skyrim death and reload,
reconnect at the loaded position, inventory kept. Also still good in this run:
recovery at session start.
**Not tested:** death caused by a Skyrim enemy's hits, by a fall or by other
Minecraft damage; dying while a Skyrim menu is open; dying with no recent save.
Not built: resetting Minecraft health when only the Skyrim character dies.
0.10.5 damage values are still untested in combat (no attacks in this report).
F5/F9 were not pressed in this run; they were verified in the previous one.
No rebuild or reinstall needed.

# Version 0.10.8: death reconnect repair

Built successfully after 15 passing Rust tests; the repair is **not tested** in
game. Made by Claude Code.

What the 0.10.7 run showed (user report plus reports 20261011-005050/005252):
- **F5 quicksave works** and **F9 quickload works**: 2 quicksaves and 6
  quickloads were sent, received and dispatched once each, and all four
  position recoveries after loads were accepted. User confirms steps 1 to 3.
- **Death half works**: Minecraft died, respawned by itself and kept its
  inventory (user confirmed). Skyrim received the death 5 ms later, its health
  went from 100 to -200, and it reloaded its last save 6.3 seconds later.
- **Reconnect failed**, because of a bug in 0.10.6: the new "you died" reply
  was rejected by the bridge's own packet check, and the Skyrim side then
  switched itself off until restart. Fixed in 0.10.8, with a test that plays a
  death and would have caught it.

Also in 0.10.8: the Rust tests are repaired and now run automatically before
every build; the log collector keeps error and connection lines.

## Test 0.10.8

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines with Fabric 0.10.8.
2. Load the paired saves, wait until you can walk, press F5.
3. Type `/kill` in Minecraft. Expected: Minecraft chat shows "You died. Waiting
   for Skyrim to reload its last save.", the Skyrim character dies, Skyrim
   reloads the quicksave, then after a short hold you can move again at the
   saved spot with full hearts and your items.
4. Tell me what happened. I will collect the logs. Expected: no `event=ERROR`
   line in Skyrim's log; `SCENE_DEATH_RESULT` with health_after at or below
   zero; `SCENE_DEATH state=clear` after the reload; then `SCENE_RECOVERY
   state=accepted` and `SCENE_ALIGNMENT state=origin_ready`.
   If it still does not reconnect, stop; the next step is a written handoff.

# Version 0.10.7: Skyrim quicksave (F5) and quickload (F9) from Minecraft

Built successfully; gameplay **not tested**. Made by Claude Code. Includes
0.10.6 (death, next section) and 0.10.5 (damage balance, also untested).

With Minecraft focused, no screen open and no Skyrim menu open:
- **F5** asks Skyrim for its own quicksave.
- **F9** asks Skyrim for its own quickload.
Both are listed under Controls > Key Binds > Miscellaneous and can be rebound.
F5 is also Minecraft's "toggle perspective". In a bridge world, while both use
the same key, the perspective toggle is switched off so F5 only quicksaves;
rebind one of them if you want the perspective toggle back. Ordinary Minecraft
worlds are unaffected. Only Skyrim saves or loads. Minecraft's world is not
saved by F5 and does not roll back on F9: blocks and inventory stay as they are.
Presses are ignored while a Skyrim menu is open, while loading, with an
unpaired save, or within one second of the previous one.

# Version 0.10.6: dying in Minecraft is a Skyrim death

Built successfully; gameplay **not tested**. First attempt.
When your Minecraft hearts reach zero (any cause), the Skyrim character is
killed and Skyrim reloads its last save, as it does for any death. Afterwards
the bridge moves you to the loaded position, as it does after a quickload.
In bridge worlds Minecraft now respawns you at once with no death screen and
you keep your inventory and experience (both are game rules the mod sets every
time a bridge world loads). Minecraft does not roll back: placed blocks and
your items stay as they were at the moment you died.
Not covered yet: if only the Skyrim character dies, Minecraft's health is not
reset. No "You died" picture from Minecraft appears in Skyrim's window; a chat
line is sent in Minecraft.

**Before testing:** make a normal, named Skyrim save from Skyrim's own menu.
The kill uses a game function that has only been proven on enemies, never on
the player; a failure or even a Skyrim crash is possible on this first try.

## Test 0.10.7 (covers 0.10.6)

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines with Fabric 0.10.7. All three files must update (protocol 28).
2. Load the paired saves and wait until you can walk. Place a marker block.
3. Press F5 in Minecraft. Expected: Skyrim shows its usual quicksave message
   and Minecraft's camera does not switch perspective.
4. Walk several blocks away and press F9. Expected: Skyrim loads the quicksave,
   Minecraft holds briefly, then you are back at the marker block.
5. Death: type `/kill` in Minecraft (or let an enemy finish you). Expected: no
   Minecraft death screen, the Skyrim character dies, Skyrim reloads the
   quicksave from step 3, and you can move again at the marker with full
   Minecraft health and your items.
6. Tell me what happened at steps 3, 4 and 5 and whether I may read the logs.
   Expected log lines: `SCENE_MENU_RECV ... action=7 ... state=dispatch` and
   `action=8`; `SCENE_DEATH_SEND` in Minecraft; in Skyrim `SCENE_DEATH_RECV
   decision=kill`, `SCENE_DEATH_RESULT` with health_after at or below zero, then
   `SCENE_RECOVERY ... state=accepted` after the reload.
   If Skyrim crashes or the character survives, stop and report; do not retry.

## 2026-10-11 0.10.5 damage balance left untested by user choice

User: "I won't test this. It sounds like it should feel right." The 0.10.5
damage values therefore remain **not tested** in game and are not installed as
far as the agent knows. They are accepted as the working design, not verified.
The next build will carry them; check `scale=1.75` in the first combat logs then.

# Version 0.10.5: outgoing damage balance

Built successfully; gameplay **not tested**. Made by Claude Code.
Hits from Minecraft on Skyrim enemies now use Minecraft attack damage x 1.75
instead of x 10. Full hits: fist 1.75 (was 10), wooden sword 7 (was 40), stone
sword 8.75, iron 10.5, diamond 12.25, netherite 14. That puts each Minecraft
sword about level with the Skyrim sword of the same rank (iron sword 7 up to
daedric 14; those Skyrim and Minecraft base values are quoted from memory, not
read from game files). Rapid hits are still weaker through the recharge rule.
Damage you take from Skyrim enemies is unchanged.

Expect a wolf (22 health in earlier logs) to need about four full wooden-sword
hits instead of one, and a mudcrab (5) one sword hit or three full punches.
The fist is weak on purpose, as in Minecraft. This is a first balance choice;
say if it now feels too weak or still too strong and the one number changes.

## Test 0.10.5

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines with Fabric 0.10.5. Protocol is unchanged (27) but install all three.
2. Load the paired saves and find a hostile enemy that survives a hit.
3. Wait two seconds, hit once with an unenchanted wooden sword. Wait two
   seconds, hit again. Then put the sword away, wait two seconds and punch once.
4. Tell me how it felt and whether I may read the logs. Expected in the logs:
   `SCENE_ATTACK_SEND` lines with `scale=1.75`, `full_damage=7.0` for the sword
   and `1.75` for the fist, and Skyrim `SCENE_ATTACK_RESULT` lines where
   before minus after equals the requested amount.

## 2026-10-11 0.10.4 pointer accepted, including NPC conversation

User confirms the mouse works inside an NPC conversation. Together with the
earlier menu report and log check, the 0.10.4 dialogue pointer is accepted.
Pointer speed scaling is deferred at the user's request and listed in the
backlog at the top of docs/DESIGN.md, next to the known first-movement doubling.
Punching an enemy after a menu is still **not tested**. No rebuild needed.

## 2026-10-11 0.10.4 pointer verified from logs

Report scene-report-20261011-001910 (user supplied and allowed reading it).
Skyrim accepted the forwarded mouse motion: 3 baselines and 65 logged moves
(move counters reach 380 and 51 in two menu sessions). In all 92 logged axis
movements the pointer moved exactly 2 screen pixels per requested count, same
direction as requested, on a 2560x1440 Skyrim screen. 8 mouse clicks were sent
as confirm; all 24 menu commands (8 mouse, 16 keys) were received and dispatched
exactly once, no duplicates. No ERROR/DISABLED lines in the three files.
User also confirms: arrows/Enter/Backspace work in NPC conversation; placing
and destroying blocks and their animations work after leaving a menu.

Known small defect: Skyrim's stored sensitivity reads 1 but the real factor is
2, so the first few small movements in each menu travel double until a movement
of 8+ counts corrects it. Not yet fixed.
Still **not tested**: the pointer inside NPC dialogue itself. The two sessions
with pointer movement have no key presses and the two with key presses have no
pointer movement; the logs do not name which menu was open. Punching an enemy
after a menu was not tried. Minecraft's window was 854x480, so one mouse unit
moves the pointer about 3 Skyrim pixels; whether that speed feels right is the
user's call. No rebuild or reinstall needed for this note.

## 2026-10-11 0.10.4 pointer confirmed by user

User reports 0.10.4 "seems to work". Described test: opened Skyrim's Tab menu
from the Skyrim window, focused Minecraft again, and the Skyrim pointer still
followed the mouse; left click moved through the menu; Backspace left the menu,
the bridge reconnected and movement resumed. This is user-confirmed gameplay.
No logs were read (Claude Code has no log permission yet), so pointer speed,
the learned pixels-per-count factor and event counts are **not verified** by
numbers. The user's description covers the Tab menu; hover highlight and click
on NPC dialogue choices were not described separately. Mouse look, block
breaking and punching after leaving a menu were not reported either way.
No rebuild or reinstall needed for this note.

# Version 0.10.4: mouse pointer in Skyrim dialogue

Built successfully; in-game behaviour is **not tested**. Made by Claude Code.
While a Skyrim menu (for example NPC dialogue) is open and Minecraft is focused:
- Moving the mouse should move Skyrim's own pointer instead of turning your view.
- Pointing at a dialogue choice should highlight it.
- Left click confirms the highlighted choice (the same action Enter sends) and
  does not swing at or break a Minecraft block.
Arrows, Enter and Backspace still work. Mouse wheel and right click are not
forwarded; right click still does its normal Minecraft action. Menus where the
exact clicked spot matters (containers, barter) are not covered by this step.

This is a first attempt. Whether Skyrim accepts the forwarded mouse motion, how
fast the pointer moves and whether up/down is the right way round can only be
settled from a real run. Pointer speed adjusts itself after the first larger
movements, so the first second may feel off.

## Test 0.10.4

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines with Fabric 0.10.4. All three files must update together (protocol 27).
2. Load the paired saves, keep Minecraft focused, aim at an NPC and press R.
3. When the choices appear, move the mouse slowly left/right, then up/down.
   Expected: Skyrim's pointer follows in the same direction and Minecraft's view
   stays still. Then make one larger sweep across the screen.
4. Point at a choice other than the highlighted one. Expected: the highlight
   moves to it. Left click. Expected: that choice is selected.
5. Check Enter/arrows/Backspace still work, then leave the conversation.
   Expected: mouse look, block breaking with left click and punching a Skyrim
   enemy all work as before.
6. Tell me what you saw for steps 3 and 4, and whether I may collect logs.
   Expected in Skyrim's log: one `SCENE_CURSOR state=baseline`, then
   `state=move` lines whose after_x/after_y differ from before_x/before_y.
   Minecraft's log should show `SCENE_CURSOR_SEND` while moving and
   `SCENE_MENU_SEND ... source=mouse` for each click.
   If the pointer never moves, stop there and report it; do not retry variations.

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

# Version0.10.3: bridge attack recharge

Built successfully; recharge gameplay **not tested**. Rapid follow-up bridge
attacks now scale damage by0.2+0.8*charge^2, using elapsed Minecraft client ticks
and held-item ATTACK_SPEED component. First attack is full strength. Item switches
share the timer.100ms spam guard replaces fixed500ms gate on both sides.
Only emitted Skyrim attack requests consume this timer; vanilla targets/misses
are outside it. The vanilla HUD recharge indicator is not synchronized yet.

**Balance feedback:** user says damage is incredibly high. Schedule a later pass
on the10x scale. Full hits remain sword40/fist10; these are not final balanced values.

Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED lines,
Fabric0.10.3; update all adapters (protocol26). With an unenchanted wooden sword,
try two clicks roughly0.2-0.3s apart against a hostile enemy that survives the first
hit, then wait two seconds and hit again. The quick follow-up should request less
damage; the rested hit should be full. Try a wooden axe likewise if convenient.
Break a normal Minecraft block afterward. Tell me Done for log collection;
charge/multiplier/full_damage/requested show the calculation even for lethal hits.
No new durability, enchantments, criticals or temporary effect handling.

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

# Version 0.10.1: held-item base damage

Built successfully; new damage calculation is **not tested** in game.
Minecraft's current attack-damage attribute times10 now sets Skyrim hit damage.
Ordinary empty hand retains10; unenchanted weapons should hit harder. This initial
balance keeps500ms cooldown and210-unit range regardless of weapon. No durability,
enchantment-specific damage, criticals or weapon attack-speed behaviour yet.

Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric0.10.1). Load paired saves. Hit a nearby hostile enemy once empty-handed,
then equip an unenchanted wooden sword, wait one second and hit again. Use another
hostile enemy if the first dies. Expect the sword to do more damage. Break a
Minecraft block normally afterward. Tell me Done; I will collect and compare
SCENE_ATTACK_SEND requested with Skyrim RECV and RESULT health changes using
rg --no-ignore. All adapters must be updated together (protocol25).

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

## 0.10.0 punch gameplay confirmed (2026-10-10)

User confirms punching damages a Skyrim enemy. Report scene-report-20261010-221938
contains no SCENE_ATTACK_SEND/RECV/RESULT events; the raw logs/step2 search also
found no SEND/RESULT events. This is user-confirmed gameplay, not numerical log
verification. Exact damage, range rejection, block priority and other regression
checks are **not tested** by this report. No new build or installation needed.
This supersedes the outgoing-punch untested status below only for enemy damage.

# Version 0.10.0: basic punches from Minecraft into Skyrim

Built successfully; outgoing attacks are **not tested** in game. Minecraft's
normal attack button (left-click by default) now requests a basic punch against
Skyrim's crosshair actor when no real Minecraft block/entity is being targeted.
Each punch requests10 Skyrim damage before difficulty adjustment, with a500ms
cooldown and maximum210 Skyrim units (three bridge blocks) feet-to-feet distance.
This uses fixed strength regardless of held item. Weapon stats/enchantments,
durability, Skyrim swing animation and knockback are not implemented.

Native checks pairing/alignment, age, target handle, actor type, cell, range and
health before calling Skyrim DoDamage with player source. Aiming changes/expired
clicks are discarded. Vanilla block breaking and Minecraft entity attacks continue.
The last native hit in the previous test occurred after the user closed Minecraft;
that was expected standalone Skyrim behaviour, not an active-bridge defect.

## Test 0.10.0

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines with Fabric0.10.0. Update all adapters together (protocol24).
2. Load paired saves. With Minecraft focused and preferably an empty hand, approach
   an existing hostile Skyrim enemy closely, aim at it briefly, then click attack.
   Expect enemy health to decrease. Try two clicks at least one second apart.
3. Step well beyond three bridge blocks and click; no damage should be applied.
   Aim at empty space and click; no Skyrim actor should be damaged.
4. Place a Minecraft block, then aim at and break it normally. That action should
   break the block without sending an NPC attack, even if an NPC is beyond it.
5. Check incoming enemy damage still affects Minecraft hearts and R/dialogue still
   works. Tell me when finished; I will collect logs directly.
   Expect SCENE_ATTACK_SEND -> RECV state=dispatch -> RESULT with the same sequence
   and target health decrease. Range/changed-target requests log a dropped reason.
   A returned call alone does not establish combat reactions or kill attribution.

## 0.9.9 incoming damage confirmed in active bridge (2026-10-10)

User confirms damage worked. Report 20261010-220115 shows three captured hits,
three unique Minecraft applications and three acknowledgements (generation3,
sequences1..3). Skyrim health remains100 during those hits; Minecraft health
20 -> 18.6422 -> 17.737 -> 16.831799, then rises to17.831799 in later samples.
The damage-clamp interception works for the tested enemy path. No new build needed.

A later fourth hit passed through with reason=not_ready and reduced Skyrim to
93.211 after Minecraft coverage disappeared and save events occurred. Thus the
active bridge path is accepted; protection while disconnected/not ready remains
limited. Full death/respawn, other damage sources and complete regeneration are
not established by this test. Earlier untested statements are superseded only for
the specific live nonlethal path verified here.

# Version 0.9.9: second incoming-damage interception attempt

0.9.8 failed: bridge readiness was true, but enemy hits bypassed the mutation
hook, leaving Minecraft hearts unchanged. 0.9.9 targets the documented Skyrim
damage-clamp interface instead and logs input/output/health. Runtime behaviour
is **not tested**; do not treat this as a confirmed fix.

Close both games; run `.\tools\install-build.ps1` (three INSTALLED lines, Fabric
0.9.9). Load paired saves and confirm movement is normal. Take one nonlethal hit,
then retreat. Expected if this path works: Minecraft hearts decrease and Skyrim
health remains unchanged. Stop if it still fails or crashes; report the result.
I will collect logs and inspect CLAMP/CAPTURE/APPLY/ACK plus both health readings.
If this second attempt fails, native hook variations stop for a documented handoff.
Protocol remains23; queue/scaling/server damage logic unchanged.

# Version 0.9.8: Skyrim hits request Minecraft damage

Built successfully; new gameplay is **not tested**. While the paired bridge and
Minecraft server are ready, negative Skyrim health mutations are queued and
suppressed at ActorValueOwner::ModActorValue. Each queued hit is attempted once
through Minecraft's normal server damage API, with acknowledgements. Minecraft
food/healing controls recovery of Minecraft hearts; Skyrim regeneration is not
forwarded. Damage scales by each game's maximum health. Minecraft mitigation and
hurt cooldown still apply; a rejected damage attempt is consumed rather than retried.

This is the first incoming-hit increment. Direct scripted health setters and full
Skyrim/Minecraft death/respawn coordination are not covered. If the bridge is not
ready or the queue is full, native Skyrim damage passes through. Pending events
are session-local and can be dropped on disconnect/load/timeout; this is not a
persistent combat transaction system. See logs for passthrough/drop reasons.

## Test 0.9.8 (nonlethal only)

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines and Fabric 0.9.8. All adapters must update (protocol 23).
2. Load paired saves, confirm movement and block collisions still work, and wait
   a few seconds for the bridge. Use Minecraft survival with enough food to heal.
3. Let the same Skyrim enemy hit you once. Expect Minecraft hearts to decrease;
   Skyrim should not separately lose health from that intercepted hit. Then take
   a second hit at least one second later and retreat. Avoid a lethal test now.
4. Let Minecraft heal, eating if needed. Hearts should recover under Minecraft's
   ordinary food rules. Skyrim healing should not restore Minecraft hearts.
5. Tell me when finished. I will run `.\tools\collect-scene-logs.ps1` and compare
   SCENE_DAMAGE_CAPTURE, SCENE_DAMAGE_APPLY and SCENE_DAMAGE_ACK by epoch/sequence.
   Expect one APPLY per captured event, suppressed=true, finite skyrim_max, and
   matching damage fractions. Capture without application, duplicate APPLY,
   unexpected native health loss or a crash is a failed test to report.

## 0.9.7 damage probe result (2026-10-10)

Report 20261010-120922 captured two user-tested enemy hits: supplied amounts
-6.789 and -4.526, attacker 000EBC4D. Health was already reduced on callback
entry (93.211 and 89.694016); before/after original callback values are identical.
This callback reports damage after health mutation in this test. Skipping it
would not prevent the recorded loss. Health subsequently regenerated to 100.
All 587 received health snapshots match Skyrim; Minecraft health stayed 20.
No new build or test needed for this documentation update. Shared health remains
unimplemented; an earlier health-mutation path must be established for authority.

# Version 0.9.7: inspect individual Skyrim damage events

Minecraft-owned health/food/healing is the chosen policy. This build instruments
Skyrim's damage callback to establish exact event behaviour before changing
Minecraft hearts. Original Skyrim damage still runs; shared health is not yet
implemented. Build passed; the callback probe is **not tested** in game.

Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.9.7). Load paired saves, check loading remains stable, then receive two
ordinary enemy hits outdoors and retreat. Minecraft hearts should remain unchanged
by Skyrim damage at this stage. Tell me when finished; I will run
`.\tools\collect-scene-logs.ps1` and compare SCENE_DAMAGE_PROBE supplied damage,
before/after health, sequence and attacker with existing health snapshots.

## 0.9.6 damage and regeneration confirmed (2026-10-10)

User reports two enemy hits, with regeneration after each. Report
20261010-120304 confirms health 100 -> 93.22922 -> 100, then
100 -> 93.28407 -> 100. Return to full took approximately 14.95 and 14.41
seconds from the first sampled low values. All 1043 received native snapshots
match Skyrim exactly; 1043 client records show Minecraft health remains 20.
No collected native ERROR or origin-block events. Read-only health observation
is accepted; cross-game damage/death and player attacks are not implemented.
No rebuild or further test is required for this documentation update.

## 0.9.6 load-stability check (2026-10-10)

Report 20261010-115741 shows 69.719 seconds of health exports after loading, with
later Skyrim save events. All 327 received health snapshots match Skyrim exactly;
329 were exported (latest-snapshot transport can skip intermediate exports).
Skyrim health stays 100; Minecraft health stays 20 in the reported readings.
No ERROR/LOAD_FAILED or sender-disabled events found in collected logs. The
load-time crash was not reproduced in this run. Damage/regeneration is still
**not tested**. Next: with the installed build and paired bridge active outdoors,
let an existing hostile Skyrim enemy hit you, retreat and allow healing; report
completion so I can run `.\tools\collect-scene-logs.ps1` and compare the changes.
No new build is required.

# Version 0.9.6: correct health-read runtime offset

0.9.5 crashed on save loading; damage and regeneration were NOT tested. The
health read used ActorValueOwner+0xB0 incorrectly. CommonLib's versioned accessor
requires +0xB8 for this project's Skyrim 1.7.104. Corrected in 0.9.6; build passed,
but the crash fix is **not tested** in game. Older 0.9.5 test instructions below
are deferred until loading is stable.

Close both games and run `.\tools\install-build.ps1`. Expect three INSTALLED
lines with Fabric 0.9.6. Start Skyrim through SKSE, load the same save, and resume
its paired Minecraft bridge. Stay still for 30 seconds, then walk briefly. Expect
no crash and normal bridge movement. Do not test damage yet. Tell me the result;
I will collect logs and look for SCENE_HEALTH_SEND/RECV readings.

# Version 0.9.5: combat foundation, health readings

Rust/Fabric builds passed. Skyrim health reading is **not tested** in game.
This increment sends current Skyrim player health in admitted world snapshots
and logs it on both sides. Minecraft's own health is also logged for comparison.
No damage is transferred yet; Minecraft hearts remain independent. Snapshot-rate
sampling may miss brief changes; menus/loading/disconnect pause the feed.

## Test 0.9.5

1. Close both games and run `.\tools\install-build.ps1` from the project folder.
   Expect three INSTALLED lines with Fabric 0.9.5. Both adapters must update
   together (protocol 22).
2. Load your paired saves and keep the bridge active outdoors. Stand still briefly
   to establish a health baseline.
3. Let an existing hostile Skyrim enemy hit you, then retreat and allow healing.
   Avoid using Minecraft fall damage as the test: it affects a separate health bar.
4. Tell me when finished and whether damage/healing visibly occurred. I will run
   `.\tools\collect-scene-logs.ps1`. Expect SCENE_HEALTH_SEND and
   SCENE_HEALTH_RECV with matching sender/world/epoch/snapshot and skyrim_health;
   SCENE_HEALTH_CLIENT includes both games' readings. Values should fall after
   damage and rise during healing. No Minecraft-heart change is expected from
   Skyrim damage in this build.

## 0.9.4 confirmed gameplay (2026-10-10)

User confirms Up/Down move dialogue selection once per press, Enter selects and
Backspace goes back. Report 20261010-114651 shows 14 menu dispatches with no
repeated sequence; context remains stable across different callback threads.
Building entry correctly blocks the bridge (interior cell 000133C8, worldspace 0,
reason 4). User exited using Skyrim controls and the bridge resumed; the log
confirms origin_ready with the same saved mapping afterward. Interior bridge
play remains unsupported. No new build or test is required for this documentation
update. Earlier statements that 0.9.4 dialogue navigation is untested are superseded
by this acceptance; other menus, remapping and held-key behaviour are not tested.

# Version 0.9.4: shared Skyrim menu command state

Rust/Fabric build passed; dialogue navigation is **not tested** with this fix.
Menu callbacks now share one context and command history, preventing separate
callback threads from rejecting each other's keys or dispatching a command twice.
The existing menu event routing is unchanged; selection still needs game testing.
Save recovery report 20261010-114219 confirms token 1 accepted with zero position
error after matching server/client positions. User reports it appears to work.

## Test 0.9.4

1. Close both games and run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines and Fabric 0.9.4. Load your paired saves.
2. Focus Minecraft, aim at an NPC and press R. Once dialogue choices appear,
   press Down once, then Up once. Each should change selection once.
3. Press Enter to select a choice; use Backspace to leave when Skyrim permits.
   Holding an arrow should not repeat. Close dialogue and check normal movement.
4. If arrows still do nothing, stop there and tell me. I will collect logs using
   `.\tools\collect-scene-logs.ps1`. Context should stay stable within the same
   menu even when callback thread IDs differ. A sequence should dispatch at most
   once. Dispatch returning alone is not proof of visible selection movement.

# Version 0.9.3: recover a loaded save's player position

Outdoor continuity was confirmed by the user and report 20261010-113517 (cells
9731 and 9712 accepted in worldspace 3C). This build implements recovery step 2.
Rust/Fabric builds passed; new recovery behaviour is **not tested** in game.

When the paired saves disagree by more than the existing position tolerance,
Minecraft should move to the loaded Skyrim save's position using the saved
coordinate mapping. Placed blocks and current Minecraft inventory are retained.
The bridge holds movement/rendering until the server/client position handshake
finishes. Invalid pairing, a different worldspace/interior, unsupported height,
or a timeout remain blocked. Historical Minecraft inventory is not restored.
Dialogue arrow navigation remains broken; its repair is the next increment.

## Test 0.9.3

1. Close both games. Run `.\tools\install-build.ps1` from this project folder.
   Expect three INSTALLED lines with Fabric 0.9.3. Install all three files together
   because protocol 21 and the world-packet size changed.
2. Resume your existing Minecraft bridge and load the paired Skyrim save that
   previously reported disagreeing player positions. Return focus to Minecraft.
   Expect a short recovery hold followed by normal movement/rendering, with
   Skyrim remaining at its loaded position. Do not create a new bridge.
3. At a known marker block, quicksave Skyrim, walk several Minecraft blocks away,
   then quickload Skyrim and focus Minecraft again. Expect the player to recover
   to the quicksave position; the marker should stay fixed. Current Minecraft
   inventory and placed blocks should remain, rather than roll back to that save.
4. Check walking, jumping, camera and outdoor boundary crossing still work. If
   recovery remains held for 30 seconds, stop there and report the message.
5. Tell me when finished. I can run `.\tools\collect-scene-logs.ps1` directly.
   Expected SCENE_RECOVERY events: request/received/server_applied/client_ready/
   accepted, with the same token and matching target positions. Then fresh
   admitted collision coverage should resume.

# Version 0.9.2: outdoor cell continuity

Rust/Fabric build passed; in-game behaviour is **not tested**. This implements only
step 1 of the recovery plan in docs/DESIGN.md. Neighboring outdoor cells in the
same worldspace now use the existing saved coordinate mapping. Interiors still
need the same cell; different worldspaces and save-position mismatches remain held.

Known failures from 0.9.1: arrow-key dialogue navigation does not work reliably;
menu context changes reject requests and duplicate dispatch was logged. Saved
position mismatch can still prevent loading an otherwise paired save. These are
steps 3 and 2 respectively, not fixed in this build. Use Skyrim's own controls for
conversation choices meanwhile. Previous 0.9.1 instructions below are historical.

## Test this increment

1. Close both games; run `.\tools\install-build.ps1` from the project folder.
   Expect three INSTALLED lines and Fabric version 0.9.2.
2. Use a paired save combination that already connects. Walk along the same route
   past the point that previously showed the different-location message, then
   walk back. Expected: movement/collision/rendering continue across the outdoor
   boundary and the message does not appear. A brief coverage hold may occur
   while collision data refreshes; a persistent hold is a failure to report.
3. Place a marker block before crossing; it should stay fixed when you return.
   Do not use a door or fast travel for this increment.
4. Tell me when finished; I will run `.\tools\collect-scene-logs.ps1`. Expected
   Skyrim log: SCENE_ORIGIN state=outdoor_cell_accepted mapping=preserved, with
   cell IDs changed but worldspace unchanged. If initial loading still reports
   position mismatch, report it rather than resetting the bridge or changing saves
   repeatedly; step 2 will address this.

# Version 0.9.1: Skyrim menu keys

Built and packaged. In-game menu navigation is **not tested**.
NPC activation with R was confirmed in 0.9.0.

While Minecraft is focused and a Skyrim menu is open:
- Arrow keys: navigate.
- Enter: confirm the highlighted choice.
- Backspace: go back/close where Skyrim permits.

Bindings appear under Controls > Key Binds > Miscellaneous. One action per press;
held keys do not repeat. Minecraft movement is held during a reported Skyrim
menu. Minecraft screens do not send these commands. Mouse navigation, text entry
and transferring Skyrim items into Minecraft inventory are not implemented.

## Test 0.9.1

1. Close both games. From the project folder run `.\tools\install-build.ps1`.
   Expect three INSTALLED lines, with the Fabric jar at 0.9.1. Both native copies
   and the jar must be updated together (protocol 20).
2. Load the existing paired saves at the same practice location. Focus Minecraft,
   aim at an NPC and press R. Use Up/Down to change the highlighted dialogue choice,
   Enter to select it, and Backspace to leave when the conversation permits.
3. Try walking during dialogue: the bridge should hold your position. After
   closing dialogue, movement, camera and placed-block rendering should resume.
4. Hold a navigation key: expect one action, not repeated scrolling. Release and
   press again. Try these keys with no Skyrim menu and from Minecraft inventory:
   they should send no Skyrim menu action. Try a remapped menu key as well.
5. If a container is available at the same practice location, open it with R and
   check navigation/back while Skyrim is paused. Confirm follows Skyrim's normal
   action, including taking an item; Skyrim and Minecraft inventories stay separate.
6. Tell me when finished; I can run `.\tools\collect-scene-logs.ps1` myself.
   Expect SCENE_MENU_SEND and SCENE_MENU_RECV with matching sequence/action/context,
   followed by SCENE_MENU_RESULT. A returned dispatch does not prove the selection
   changed: report what happened in the game too.

Previous release notes follow.

# Passthrough practice

## Latest: interact with Skyrim (0.9.0)

Press **R** while Minecraft is focused to activate Skyrim's current crosshair
target. The key is listed as **Interact with Skyrim** under Minecraft Controls >
Key Binds > Miscellaneous and can be rebound. Minecraft's normal use/attack keys
are unchanged. Each press sends one short-lived request; holding does not repeat.
Skyrim's normal activation function chooses the outcome. Rust/Fabric builds passed.
User confirms NPC conversation opens. Report 20261009-214457 confirms one request,
one native activation dispatch/return and menu block/unblock transitions. Rebinding,
held-key behaviour, empty targets and non-NPC objects remain **not tested**.

### Test 0.9.0

1. Close both games and run `.\tools\install-build.ps1`; expect three INSTALLED
   lines including 0.9.0. Both adapters need updating (protocol 19).
2. Open the same paired saves at the established practice location. Focus Minecraft
   and aim at a nearby Skyrim lever/switch or another object that stays in the same
   area. Wait for Skyrim's usual interaction prompt, then press R once. Expect the
   normal Skyrim action. Holding R should not trigger repeated actions.
3. Aim at empty space and press R; nothing should activate. Open Minecraft inventory
   and press R; that should not activate Skyrim either.
4. Rebind Interact with Skyrim in Minecraft controls and try that key. The binding
   should persist when Minecraft is restarted normally.
5. Optional: activate a Skyrim container or NPC. A native Skyrim menu may open;
   bridge movement/look should suspend. This version does not forward menu choices,
   clicks or exit keys. Switch to Skyrim's window to operate/close the menu, then
   return to Minecraft. Keep both players at the same practice location.
6. Tell the agent when finished. It can collect logs directly. Expect matching
   SCENE_INTERACT_SEND/SCENE_INTERACT_RECV sequence numbers, a target_handle and
   state=dispatch for activation, state=no_target for empty aim, and no repeated
   dispatch for a held key. SCENE_INTERACT_RESULT returned confirms a function
   call returned, not that gameplay changed; report what the object actually did.

First test in-place objects; doors that load another cell still encounter the
existing saved-location hold. Menu input routing is next, followed by combat and
damage. Travel/rollback remain deferred. Interaction cooldown is 250 ms; inactive/
stale/menu requests are discarded, not queued. Native activation uses Skyrim's
target, not Minecraft block aim.


## Latest: restart alignment (0.8.9)

0.8 now targets a paired practice world that can be closed and reopened at the
same Skyrim location. 0.9 will handle save rollback/revisions, doors/interiors,
location-specific construction and long-distance/height remapping.

0.8.9 stores the coordinate origin in Skyrim's normal SKSE companion save and
reuses it for the same Minecraft world and Skyrim cell/worldspace. Movement,
collision and rendering all use that origin. Different worlds/locations, invalid
origin data or more than two blocks of disagreement between saved player positions
hold the bridge and show a reason. This checks proximity, not save revisions.
Rust/Fabric builds passed. The user confirms block/inventory persistence after
restart; report 20261009-212809 and the two Skyrim sessions confirm exactly equal
saved/restored world ID, location and origin vectors. The agreed same-location
restart milestone for 0.8 is complete. This is not unrestricted travel or rollback
support. An earlier setup attempt logged a position-mismatch hold before reloading
and making the final successful save; that limitation remains documented.

### Test 0.8.9: finish the 0.8 acceptance check

1. Close both games; run `.\tools\install-build.ps1` from the project folder.
   Expect three INSTALLED lines including 0.8.9. Both adapters must update
   (protocol 18); existing pairing metadata stays unchanged.
2. Launch Skyrim through SKSE, load the usual paired practice save, and Resume
   the existing Minecraft bridge. Stay in the same small practice area.
3. Place a distinctive block beside a Skyrim landmark. Note the relationship.
   Stop moving and save Skyrim to a **new slot while both games are connected**.
   Keep the previous Skyrim slot. Then Save and Quit Minecraft. Close both games.
4. Restart both games, loading that exact new Skyrim slot and the same Minecraft
   bridge. The block should remain beside the same landmark; movement, camera,
   collision and rendering should resume normally.
5. Tell the agent when finished. It can collect logs directly and inspect both
   native Skyrim sessions for SCENE_ORIGIN created/saved, then loaded/restored,
   with identical mc_origin/sky_origin and Minecraft world ID.

Legacy Skyrim saves without an origin establish one on first admitted connection;
pre-0.8.9 placement alignment cannot be reconstructed from those saves. Do not
rename the `skyrim-bridge-<UUID>` Minecraft save folder: that UUID identifies the
world. Renaming the display name also retains the older bridge-marker restriction.
Use the latest paired saves and avoid moving Skyrim alone between saving the two
games. If a hold message appears, stop and report it; do not delete pairing files
or create another bridge. Old same-lineage Skyrim saves, different locations and
migrating construction remain 0.9 work. No automatic save selection or rollback.

Earlier version sections below are historical implementation/test notes.


## Latest repair: matched-save startup (0.8.8)

0.8.7 failed to activate the user's existing bridge. Logs show eligible=false
before admission. Fixed an early pairing lookup caching world-not-ready as a
permanent rejection, and an independently found wire bug where expected lineage
bytes overlapped renderer status. Pairing now uses unused bytes 240–255; protocol
17 isolates the broken encoding. Rust/Fabric builds passed. User confirms normal
movement/rendering; report 20261009-205054 confirms eligible=true, matched IDs and
native allowed=true. User testing and report 20261009-205719 also confirm missing-ID
blocking, mismatched-ID blocking and recovery after loading the original paired
slot. Save-revision matching and restart-origin restoration remain unimplemented. Existing
saves and pairing metadata do not need changing.

Close both games and run `.\tools\install-build.ps1`; expect three INSTALLED
lines including 0.8.8. Load the original paired Skyrim save and Resume the existing
Minecraft bridge. First check only normal activation, movement and rendering.
Tell the agent the result so it can collect logs for eligible=true, matched and
SCENE_ADMISSION allowed=true. Defer mismatch testing until this path works.


## Latest: pairing enforcement (0.8.7)

Skyrim now checks Minecraft's stored pairing ID before applying bridge movement,
look or rendering. Unpaired, invalid or mismatched IDs leave bridge control
inactive. Identity-only replies still allow pairing; Minecraft waits for admitted
collision data before moving and exporting its scene. Rust/Fabric builds passed;
runtime enforcement is **not tested**. Same-lineage older saves
still match: save revision checks and restored block alignment remain pending.

### Test 0.8.7

1. Close both games and run `.\tools\install-build.ps1`; expect three INSTALLED
   lines including the 0.8.7 JAR. Both DLLs must update (protocol 16).
2. Load your paired Skyrim save through SKSE and Resume the paired Minecraft
   bridge. Expect normal movement/rendering after collision becomes ready.
3. For mismatch testing, use a Skyrim save with a different bridge ID (a slot
   predating the ID must first be saved to a new slot). Keep the paired slot.
   With the existing Minecraft bridge, expect the mismatch warning, held Minecraft
   movement and no Skyrim puppet/camera control from Minecraft. Native Skyrim
   control is released; this does not freeze Skyrim itself.
4. Load the original paired Skyrim slot. The bridge should resume after fresh
   collision arrives. Do not assess saved-block alignment across these loads;
   origin restoration is not implemented yet.
5. Tell the agent when finished. It will collect SCENE_PAIRING and SCENE_ADMISSION
   logs (allowed=false for mismatches, true for matching IDs). New/unpaired bridge
   worlds should still offer Pair before enabling movement.

User-confirmed 0.8.6 pairing was verified in report 20261009-204217: unpaired,
paired, matched, then matched again after reopening with the same lineage ID.


## Latest: explicit Minecraft pairing (0.8.6)

Minecraft offers to remember the connected Skyrim lineage for the current bridge
world. Accepting creates `passthrough-pairing.txt` in that world's root; an existing
pairing is never overwritten. Future connections log matched or warn mismatch.
This version **does not block mismatched play or restore origins**. Stop bridge
play on a mismatch and load the matching saves. Builds passed; pairing gameplay
is **not tested**. Both adapters need updating (protocol 15).

### Test 0.8.6

1. Close both games and run `.\tools\install-build.ps1`; expect three INSTALLED
   lines including the 0.8.6 JAR.
2. Launch Skyrim through SKSE and load the save created in the previous test.
   Start Minecraft and Resume bridge. Accept **Pair** when prompted.
3. Save and quit Minecraft to its title screen, then Resume the same bridge with
   the same Skyrim save. It should recognize the pairing without another prompt.
4. Tell the agent when finished so logs can be collected. Expected SCENE_PAIRING
   states: unpaired, paired, matched; matched again after reopening.
5. Optional mismatch check: load an older Skyrim slot predating its ID, save to
   a new slot to create a different lineage, then reopen this paired bridge.
   Expect a mismatch warning; stop play and return to the matching Skyrim slot.
   The pairing file must remain unchanged. Copies of the identified Skyrim save
   share a lineage and will not produce a mismatch.

Cancellation leaves the world unpaired; reopening offers again. A missing Skyrim
ID asks you to save Skyrim first. Bad pairing metadata or write failure is logged
and shown; it is never silently replaced. This is a lineage association, not a
save revision, automatic world selection or cross-restart block alignment.


## Latest: Skyrim save identity (0.8.5)

Adds a bridge ID to Skyrim's normal SKSE companion save through SKSE callbacks.
The ID is created on the first save and restored when that save loads. Save As
and copied saves share the ID: it identifies a lineage, not an individual save
revision. Old saves without bridge data acquire an ID when next saved. Minecraft
pairing and restart-origin restoration are still unimplemented. Build passed;
save/load identity persistence is confirmed by the user's Skyrim-only restart
test and matching logs from two processes (27040 and 14360). Protocol remains 14.

### Test 0.8.5

1. Close both games; run `.\tools\install-build.ps1`. Expect three INSTALLED
   lines, including the 0.8.5 JAR.
2. Start Skyrim through SKSE, load your practice save, and save to a **new slot**.
   Keep your previous slot for rollback. Resume the usual Minecraft bridge if
   checking existing gameplay too.
3. Fully quit Skyrim, restart through SKSE and load the new slot. Save once more.
4. Tell the agent when finished. The agent will run
   `.\tools\collect-scene-logs.ps1` and compare SCENE_SAVE_ID entries: the ID
   marked saved before quitting should equal loaded after restarting and saved
   again. Avoid extra Skyrim restarts before collection, since the collector
   selects recent sessions. This verifies identity persistence, not paired saves
   or block alignment across restarts.


## Latest: pause alignment (0.8.4)

Keeps the current coordinate origin in memory across temporary pauses or missing
position samples. It reuses that origin only in the same sessions/cell, without
a Skyrim load/reset, and with at most one Skyrim unit of movement while suspended.
Otherwise it creates a fresh origin. Rust/Fabric builds passed. The user confirms
the pause test behaves as expected. Report 20261008-212546 records three origin
reuses and two rejected reuses after Skyrim moved while suspended (one involved
only vertical movement). This does not prove every pause preserves alignment.
Save binding, restart alignment and door travel remain pending.

### Test 0.8.4

1. Close both games and run `.\tools\install-build.ps1`; expect three INSTALLED
   lines and the 0.8.4 JAR.
2. Resume the same bridge and place a block beside a Skyrim landmark. Stop moving.
3. Open and close Minecraft's pause menu, then Skyrim's pause menu. Wait for
   coverage to resume each time. The block should remain beside the same landmark;
   movement, collision and camera control should resume normally.
4. Tell the agent testing is finished while the session logs are still current.
   The agent can run `.\tools\collect-scene-logs.ps1` and check
   SCENE_ALIGNMENT for suspended followed by resumed. A new_origin event states
   why reuse was rejected. No claim of persistence across restarts is made.


## Latest: location identity (0.8.3)

Both adapters now carry Skyrim cell/worldspace record IDs in collision snapshots.
Logs expose location changes as SCENE_LOCATION_SEND and SCENE_LOCATION_RECV.
Rust and Fabric builds passed. Report 20261008-212004 confirms matching cell and
worldspace IDs in both games for two alignment epochs in one session. Stability
across full restarts and location changes is **not tested**. This is preparation
for stable construction/save alignment, which remains unimplemented. IDs depend
on plugin load order and do not identify a Skyrim save. Sprint changes are deferred.

### Test 0.8.3 location identity

1. Close both games. Run `.\tools\install-build.ps1` from the project folder;
   expect three INSTALLED lines including the 0.8.3 JAR. Both DLLs must be updated
   because protocol 14 uses a new shared-memory namespace.
2. Open the same Skyrim location and Resume bridge. Move briefly, then run
   `.\tools\collect-scene-logs.ps1` before restarting Minecraft. Expect three
   COLLECTED lines. Send the resulting report path.
3. The report should contain SCENE_LOCATION_SEND in Skyrim and
   SCENE_LOCATION_RECV in Minecraft with matching eight-digit cell_id and
   worldspace_id values for matching epoch/snapshot. Zero worldspace is allowed
   (typically interiors). Restart both games in the same Skyrim cell, repeat,
   and compare IDs; session/epoch numbers may change.
4. Existing movement, collision, camera and rendering should remain usable.
   Door travel, save binding and construction remapping are not implemented.


Version **0.8.2** starts each collision scan ahead of recent movement, with a
bounded prediction based on scan duration. It keeps the existing four-block
radius, half-block sampling and work budget. New logs separate missing/stale
collision from moving outside its bounds and report hold durations. **Not tested
in game**; smoother travel is not yet confirmed. Sudden turns, fast falls and
missing geometry can still cause holds. This is not unlimited travel or exact
Skyrim mesh collision. User confirms 0.8.1 save resuming works.

### Test 0.8.2 travel coverage

1. Close both games and run `.\tools\install-build.ps1`; expect three `INSTALLED`
   lines. No rebuild needed after the packaged build.
2. Load Skyrim at the same practice location and use Minecraft **Resume bridge**.
   Wait for ready. Walk then sprint a straight route, turn around sharply, and
   try slopes, stairs, jumping and short drops. Expected: Skyrim walls/ground
   still collide, with fewer edge-of-coverage pauses during sustained movement.
   Report where pauses remain; do not assume thin objects are sampled perfectly.
3. Briefly pause Skyrim and resume. The bridge should hold when collision is
   unavailable, then resume when coverage returns. Check ordinary block placement
   and your saved inventory. Save/cell alignment limits remain unchanged.
4. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three
   `COLLECTED` lines. Send that report path and the route observations. Skyrim
   SCENE_WORLD_SCAN includes lead, horizon, duration, centre and active status.
   Client/server SCENE_BRIDGE_MOTION includes hold_ms, requested bounds and reasons
   such as outside_bounds, no_snapshot or snapshot_expired. A ready transition
   reports the elapsed preceding hold; startup waits are separate from route pauses.

Version **0.8.1** adds **Resume bridge** to Singleplayer. It offers the most
recently played bridge save, showing its name/folder before opening. Select a
different bridge directly from the normal world list. **New Skyrim bridge** now
explains that it starts a separate inventory and requires an explicit Create new.
Use it once per new bridge, not every session. Reusing the same Minecraft save
uses Minecraft's existing inventory, equipment, experience and block persistence.
No inventory is transferred between worlds. New resume UI and inventory logging
are **not tested in game**; both packages built successfully.

The user confirms the basic 0.8.0 empty-world setup works. Matching Minecraft to
a particular Skyrim save, rollback and stable construction alignment across
Skyrim reconnects/interiors remain unfinished. Resume bridge chooses a Minecraft
save by last-played time; it does not identify the loaded Skyrim save.

### Test 0.8.1 resume and inventory

1. Close both games and run `.\tools\install-build.ps1`; expect three `INSTALLED`
   lines. No rebuild needed.
2. Load Skyrim in the same practice location. In Minecraft Singleplayer, use
   **Resume bridge** and check the offered save name. Use your existing bridge,
   not New Skyrim bridge. You can cancel and select another save from the list.
3. Put distinctive items in your inventory, for example
   `/give @s minecraft:diamond 3`. Note counts, slot positions and equipment.
   Use Minecraft's normal **Save and Quit to Title**, leaving Minecraft open.
4. Choose Singleplayer > Resume bridge again. Expected: the same inventory,
   equipment, experience and Minecraft blocks reload. Check that no new save was
   created. Skyrim alignment of those blocks after reconnect is a separate known
   limitation. Try New Skyrim bridge > Cancel and check the existing save is intact.
5. Run `.\tools\collect-scene-logs.ps1` after reopening, before closing/restarting
   Minecraft; expect three `COLLECTED` lines. Send the report path. Compare the
   previous SCENE_BRIDGE_INVENTORY phase=save_return with the new join_return for
   the same save/player. Slots, counts, equipment and XP should match absent
   gameplay changes. Logs do not compare enchantments, other item components,
   ender chests, or prove disk-write durability. If restarting Minecraft itself,
   collect a report before exiting and another after loading, and provide both.

Version **0.8.0** implements step 1 of the empty bridge-world plan. The Singleplayer
world list has a **New Skyrim bridge** button. It creates a separate survival save
with commands enabled, all-air chunks, no terrain/platform/structures/lakes or
natural mob spawning. Existing worlds are not changed or converted. Passthrough
is inactive in ordinary worlds. Player construction keeps normal Minecraft physics.
**Not tested in game.** Both adapters must be installed together (protocol 13).

The marked bridge world holds player movement until fresh Skyrim collision covers
the body/path and, initially, terrain is detected below the feet. It holds again
on missing coverage rather than falling into the void. The four-block sampler
is unchanged; fast travel/drop edges can cause holds. This first version is for
local walking/jumping at one Skyrim location. Unlimited altitude, smooth long
travel, stable construction across Skyrim reconnects/cell changes and matched
saves are not implemented. Scale/reach calibration and fluid containment are pending.
Do not rename the generated save display name: it includes the bridge identity.

### Test 0.8.0 empty bridge world

1. Close both games and run `.\tools\install-build.ps1`. Expect three `INSTALLED`
   lines; no rebuild needed.
2. In Skyrim load a single-player location and stand on solid ground, outside menus.
3. In Minecraft choose **Singleplayer > New Skyrim bridge**. This creates a new
   save; it does not transfer your old inventory/builds. Keep its generated name.
   Expect an initially empty world and a waiting message, then **Skyrim bridge
   ready** when collision data is available. No Minecraft floor/platform should
   appear. If waiting persists, collect logs rather than using teleport/noclip.
4. Walk and jump a short route in Skyrim using Minecraft controls. Expected:
   Skyrim ground supports you, Skyrim walls block you, and no Minecraft terrain
   obstructs travel. Run `/give @s minecraft:stone 64`; place a small wall, check
   that it blocks you, then break it. Terrain-created non-air counts should be zero;
   deliberately placed blocks are expected afterward.
5. Pause/close Skyrim briefly. Expected: Minecraft movement holds instead of
   falling through the empty world. Resume and report the result; persistent
   construction alignment across reconnects is not yet guaranteed. As a separate
   check, load an old ordinary Minecraft save: expect the bridge-inactive message
   and normal Minecraft rendering/movement, with no Skyrim puppet control.
6. With the bridge world active, run `.\tools\collect-scene-logs.ps1` before
   restarting Minecraft. Expect three `COLLECTED` lines. Send the report path.
   Look for SCENE_BRIDGE_WORLD eligible=true, SCENE_BRIDGE_CHUNK non_air_blocks=0,
   SCENE_BRIDGE_COVERAGE bounds/remaining_ms and SCENE_BRIDGE_MOTION transitions
   from waiting to ready on both client and server. These logs are already included
   by the collector's SCENE_ filter. Reopening an existing bridge save need not
   regenerate chunks, so generation logs are expected primarily on creation.

Version **0.7.3** adds filled water/lava bucket placement directly on Skyrim
surfaces, using the contact-preserving target and offline server validation.
**Not tested in game.** Water still flows through Skyrim geometry; terrain-aware
fluid simulation is the next separate step. No invisible support blocks are added.
Empty buckets and ordinary Minecraft bucket targets keep their normal behavior.

The user accepts current block placement for this iteration but reports that
Minecraft blocks look too large and placement reach feels too short. Scale/reach
calibration remains pending; this build retains 70 Skyrim units per Minecraft
block, normal interaction reach and the four-block terrain collision coverage.

### Test 0.7.3 terrain bucket placement

1. Close both games and run `.\tools\install-build.ps1`; expect three `INSTALLED`
   lines. The package is built; no rebuild needed.
2. In the offline practice world run `/gamemode creative`,
   `/give @s minecraft:water_bucket` and `/give @s minecraft:lava_bucket`.
   Hold each filled bucket and right-click separate nearby Skyrim floor targets
   without a Minecraft starter block. Expected: that fluid appears in the targeted
   Minecraft cell. It may flow down through Skyrim terrain; containment is absent.
3. Check buckets still work against ordinary Minecraft blocks and waterlog a
   Minecraft slab normally. In survival, a successful pour should return an empty
   bucket; a rejected pour should keep the filled bucket. Creative should retain
   the filled bucket. An empty bucket should still pick up Minecraft source water.
4. Check ordinary stone placement, movement and the compass. Run
   `.\tools\collect-scene-logs.ps1`; expect three `COLLECTED` lines and send the
   report path. SCENE_PLACE should match intent/cell across client and server;
   SCENE_BUCKET should show decision=emptied and the resulting fluid state.
   invalid_target means the preceding placement diagnostic explains rejection.

Version **0.7.2** keeps direct terrain placement tied to the actual contact.
The user confirmed 0.7.0/0.7.1 could place blocks but direct terrain placement was
pulled toward the player; normal Minecraft adjacency worked. The new version
removes the grid-plane adjustment and carries the actual contact separately to
the offline integrated server. Both packages built. **Not tested in game.**

The destination uses the contact's tangential coordinates, with normal-axis grid
rounding. A gap smaller than one block can remain. Sampled box-edge faces,
four-block collision coverage and conservative occupancy checks remain limitations.
Water/lava rendering was added in 0.6.8; water was reported visible, but fluid
containment by Skyrim terrain and direct bucket placement remain unimplemented.

### Test 0.7.2 contact-preserving placement

1. Close both games and run `.\tools\install-build.ps1`; expect three `INSTALLED`
   lines. Both packages are built; no rebuild needed.
2. In the offline practice world run `/gamemode creative` and
   `/give @s minecraft:stone 64`. Hold stone and aim at identifiable near and far
   points on fairly flat Skyrim ground, within roughly four Minecraft blocks.
   Expected: placement follows the floor contact instead of sliding toward you
   as the aim becomes flatter. No Minecraft starter block should be required.
3. Try a wall and normal placement beside Minecraft blocks. Check movement,
   breaking and that nothing duplicates. In survival, a successful placement
   should consume one block; a rejected placement should consume none.
4. Hold a failed aim steady for two seconds and click once, then run
   `.\tools\collect-scene-logs.ps1`; expect three `COLLECTED` lines. Send that
   report path. SCENE_TARGET should include mapping=contact_cell; client/server
   SCENE_PLACE should share intent IDs. For an up-facing surface, cell X/Z should
   equal the floors of surface X/Z. SCENE_PLACE_RESULT gives the final state.
   missing_or_expired_intent and no_current_intent identify handoff failures.

Version **0.6.7** extends 3D dropped blocks to transparent and non-full default
block models, including glass, panes and ice. Translucent drop faces share the
placed-block sorting/blending pass. The confirmed pre-HUD draw stage is retained.
The user confirms this drop extension works. Broader cases and numerical log
comparison remain **not tested**. Models are limited to 96 quads with vertices inside the
block bounds; unsupported models retain icons. Item-specific model components,
special block-entity renderers and exact intersecting-surface sorting remain
unsupported. Panes use their default block-state shape, not a full item-renderer
reproduction.

Known limitation: breaking cracks are still restricted to opaque full blocks.
The user reports no breaking animation on glass or ice, consistent with that
separate crack-export filter. Extending dropped models did not extend cracks.

### Test 0.6.7 dropped glass and panes

1. Close both games and run `.\tools\install-build.ps1`. Expect three `INSTALLED`
   lines; no rebuild needed. Install both adapters together for protocol 11.
2. In Minecraft run `/give @s minecraft:glass 8`,
   `/give @s minecraft:glass_pane 8`, `/give @s minecraft:ice 8`,
   `/give @s minecraft:blue_stained_glass 8` and `/give @s minecraft:red_sand 8`.
   Drop each with Q. Expected: small spinning 3D blocks/pane shapes; ice and
   stained glass should blend transparently. Red sand should stay 3D.
3. Check pickup/disappearance, walls hiding drops, placed glass and cracks,
   ordinary apple icons, and Skyrim's compass remaining above world geometry.
4. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three
   `COLLECTED` lines. `SCENE_ITEM` should report `geometry=block_model`; ice/stained
   glass should also have nonzero `translucent_vertices`. Compare native
   `items`, `models`, `item_vertices` (opaque item faces) and
   `translucent_item_vertices` across matching send/receive/draw snapshots.
   Send those files and observations if a drop is missing or still an icon.

Version **0.6.6** addresses world geometry covering Skyrim's compass by moving
world drawing from Present to immediately before the HUD menu's display call.
Minecraft hands/HUD remain at Present. The user confirms the HUD is no longer
blocked. Broader menu/HUD-hidden cases are **not tested**. Projection,
transparency sorting and depth comparison are unchanged. If the HUD display hook
does not run (including some HUD-hidden modes), world geometry may not draw; it
does not fall back to the old compass-covering stage.

Dropped glass, glass panes and ice still use 2D icons by design in this version:
the 3D drop path accepts only supported opaque full-block models. The user reports
these icons and confirms red sand remains 3D. Placed translucent-block support
does not yet extend to dropped translucent or non-full block models.

### Test 0.6.6 compass layering

1. Close both games and run `.\tools\install-build.ps1`. Expect three `INSTALLED`
   lines. The packages are already built.
2. Start offline first-person worlds with Skyrim's HUD visible. Place a tall
   Minecraft wall, then look so its blocks overlap the compass on screen.
   Expected: the compass remains visible over the blocks, with no duplicated
   world geometry. Repeat with stained glass and check world depth still works.
3. Check dropped blocks/items, cracks, Minecraft hands/HUD, and opening/closing
   Skyrim menus. Report whether blocks disappear unexpectedly or still cover UI.
4. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft. Expect three
   `COLLECTED` lines. Skyrim should log `SCENE_STAGE_HOOK installed=1` (with other
   fields between), `SCENE_STAGE stage=before_skyrim_hud present_world_draw=0`
   and normal `SCENE_DRAW` records. An initial `SCENE_STAGE_WAIT` may occur while
   Present initializes the renderer; repeated waits indicate the new stage is
   not running. Send the report with the visual result.

Version **0.6.5** adds translucent placed block models, such as stained glass and
ice. Faces are sorted far-to-near at mesh publication and alpha-blended after
opaque blocks, dropped items and cracks, with Skyrim-wall depth checks retained.
The user confirmed transparent blocks display properly, and reported blocks
covering Skyrim's compass; the 0.6.6 change above addresses that draw order.
Broader transparency/regression cases are **not tested**. Water/lava and other
fluids remain unsupported. Sorting
is approximate and refreshes at roughly 10 Hz; intersecting surfaces, both-sided
faces and fast camera movement may show blending artifacts. This does not add
colored lighting, refraction or translucent 3D dropped-block models.

### Test 0.6.5 translucent blocks

1. Close both games and run `.\tools\install-build.ps1`. Expect three `INSTALLED`
   lines. Both adapters must use this build (protocol 10).
2. In an offline first-person practice world, give yourself glass, blue stained
   glass, red stained glass and ice using `/give @s minecraft:glass 8`,
   `/give @s minecraft:blue_stained_glass 8`, `/give @s minecraft:red_stained_glass 8`
   and `/give @s minecraft:ice 8`. Place them nearby. Expected: Skyrim remains
   visible through transparent areas; stained glass tints what is behind it.
   Ice may melt under Minecraft's ordinary rules, so check it promptly.
3. Place stone behind glass, and two differently colored glass blocks along the
   same view direction. Move around them. Expected: visible stone through glass,
   opaque stone hiding glass behind it, and Skyrim walls hiding both. Check
   cracks on stone viewed through glass, 3D drops, icon drops and hands/HUD.
4. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft. Expect three
   `COLLECTED` lines. Client `SCENE_TRANSLUCENT` should show nonzero vertices for
   nearby stained glass/ice and sort/copy timings. Match `translucent_vertices`
   across `SCENE_SEND`, `SCENE_RECV` and `SCENE_TRANSLUCENT_DRAW` by snapshot.
   Remove the translucent blocks: counts should return to zero if none remain
   in range. Plain glass can use Minecraft's cutout path, so use stained glass
   or ice for the translucent-count check. Send the report and observations.

Version **0.6.4** adds 3D drops for supported full opaque blocks, using their
default block model's six faces, texture coordinates and default block tint.
They are quarter-block-sized and retain bob/spin and Minecraft-controlled pickup.
Other items and unsupported block models keep the spinning icon fallback.
The user confirms dropped items and blocks appear as they should in 0.6.4.
Numerical mixed model/icon log comparison and broader regressions are **not tested**.
Complex models, item-specific block-state components,
full item lighting/glint and smooth interpolation between exports remain later work.

### Test 0.6.4 block drops

1. Close both games and run `.\tools\install-build.ps1`. Expect three `INSTALLED`
   lines. Both adapters must use this build (protocol 9).
2. In an offline first-person world, run `/give @s minecraft:dirt 8`,
   `/give @s minecraft:oak_log 8` and `/give @s minecraft:apple 8`. Drop each with
   **Q**. Expected in Skyrim: small 3D dirt/log blocks, with the log's end-grain
   and bark on different faces; apple stays a flat spinning icon.
3. Move around the drops and collect them. Expected: they stay fixed in the world,
   disappear on pickup and remain hidden behind walls/blocks. Check block cracks,
   ordinary textures, hands/HUD and movement/look.
4. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft. Expect three
   `COLLECTED` lines. `SCENE_ITEM` should show `geometry=block_model vertices=36`
   for supported blocks and `geometry=icon vertices=6` for apples. Matching
   send/receive/draw snapshots should agree on `items` and `item_vertices`;
   Skyrim also logs model/icon counts. Send the report with your observations.

Version **0.6.3** adds a first dropped-item renderer: small double-sided spinning
icons for nearby Minecraft item entities. It uses a sprite supplied by each item's
model; blocks currently appear as flat face icons, not miniature cubes. Minecraft
still handles falling, merging and pickup. The user confirmed 2D drops spin,
can be picked up and then disappear. Numerical item-log comparison and broader
occlusion/merging cases remain **not tested**. The export is
limited to 64 item attempts and a 3 ms budget per completed snapshot, at roughly
10 Hz; motion may look stepped and crowded scenes may omit items. Custom models,
glint, tint, full 3D geometry and animated textures are not reproduced.

### Test 0.6.3 dropped items

1. Close both games and run `.\tools\install-build.ps1`. Expect three `INSTALLED`
   lines. Both adapters must be updated together for protocol 8.
2. Start offline first-person worlds. In Minecraft run `/give @s minecraft:apple 8`
   and `/give @s minecraft:dirt 8`. Hold an apple and press **Q**, then do the same
   with dirt. Expected in Skyrim: a small spinning apple icon and dirt-face icon
   at the dropped items' positions. They should bob, fall, stay in the world as
   you move, and be hidden by Skyrim walls and Minecraft blocks.
3. Walk over them after the pickup delay. Expected: the icons disappear and the
   items return to Minecraft inventory. In survival, mine dirt bare-handed to
   check a naturally created drop. Check stone cracking, ordinary block textures,
   hands/HUD and movement/look still work.
4. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft. Expect three
   `COLLECTED` lines. Client `SCENE_ITEM` records contain entity IDs, stack counts
   and interpolated positions. Matching `SCENE_SEND`, `SCENE_RECV` and
   `SCENE_ITEM_DRAW` should agree on `item_vertices` (6 per icon, then zero once
   all nearby drops are gone). Send the report with your observations.

Version **0.6.2** implements the next small part of step 5b: breaking cracks on
nearby full opaque blocks, initially stone and similar full cubes. Minecraft's
damage stages select runtime crack textures; Skyrim draws them after blocks,
with transparency and the same wall-depth check. The user reports gameplay works
as expected. The 20261008-084544 report confirms 399 matching meshes plus one
atlas, all ten crack stages, 45 draws with cracks and four returns to zero cracks.
There were no renderer errors in that report; exhaustive graphics cases remain
**not tested**.
Slabs, stairs, glass, leaves and other non-full/transparent shapes are skipped.
Dropped items remain unimplemented. Camera orbit, world bleed and placement reach
are still deferred/unresolved as recorded below.

Minecraft ground rendering is expected in this build: the exporter includes all
supported nearby blocks, including generated terrain and a superflat floor. It
does not distinguish player-placed blocks from generated blocks. Minecraft terrain
also keeps its collision. The intended mostly empty Minecraft world avoids
duplicating Skyrim's terrain; hiding the floor alone would leave invisible
collisions. A void-world setup/removal of generated terrain is not implemented.

### Test 0.6.2 breaking cracks

1. Close both games and run `.\tools\install-build.ps1`. Expect three `INSTALLED`
   lines. Install both adapters together: this build uses protocol 7.
2. Start the offline practice worlds in first person. In Minecraft, run
   `/setblock ~2 ~1 ~ minecraft:stone`, then `/gamemode survival`. Use an empty
   hand and hold attack on the stone. Expected: cracks advance on the block in
   Skyrim while the Minecraft arm swings. Creative's instant break cannot show
   a useful progression.
3. Release attack before it breaks. Expected: cracks clear while the block stays.
   Mine again to completion: the block and cracks should disappear. Item drops
   are still not rendered. A Skyrim wall should hide both block and cracks.
   Check ordinary block textures, movement, mouse look and HUD remain intact.
4. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft. Expect three
   `COLLECTED` lines. Client records should show `SCENE_CRACK` stages 0–9 (some
   stages can be skipped at the roughly 10 Hz export rate) and `SCENE_CRACKS`.
   For matching snapshots, bridge `SCENE_SEND`/`SCENE_RECV` and Skyrim
   `SCENE_CRACK_DRAW` should agree on `crack_vertices`: 36 for one block, then 0
   after cancellation or completion. Send the report if this does not happen.

Version **0.6.1** adds rendering diagnostics. The user corrected the initial
three-block estimate: the observed cutoff is roughly eight Minecraft blocks,
consistent with the intended 16x16x16 export cube. The collected run contains
1,443 matching mesh snapshots and one matching atlas; all 317 depth probes were
usable, and the camera depth calculations agree within 0.00000114 at 500 Skyrim
units. The diagnostics and report collector ran successfully in this session.
Movement and block geometry both use 70 Skyrim units per Minecraft block.
Whether that scale feels appropriate, and why placement requires getting very
close, remain unresolved. Camera orbiting and Minecraft-world bleed are reported
issues deferred at the user's request. Alignment resets in the full Skyrim log
are a lead for later investigation, not a confirmed explanation of those issues.

To reproduce with the diagnostic package:

1. Close both games and run `.\tools\install-build.ps1` from the project folder.
   Expect three `INSTALLED` lines. The package is already built.
2. Start both offline worlds in first person. In Minecraft run
   `/setblock ~2 ~1 ~ minecraft:stone`. Keep looking at that block and slowly
   back away until it disappears, then approach it again. Also check a Skyrim
   wall still hides it. Expect an export boundary roughly eight blocks away.
3. Before restarting Minecraft, run `.\tools\collect-scene-logs.ps1`.
   Expect three `COLLECTED` lines and a report folder under `logs`.
   Send the three files in that folder. They should contain `SCENE_BOUNDS`,
   `SCENE_PROJECTION`, and `SCENE_DEPTH` with `center` and `sampled_ms` fields.
   The collector selects each game's newest bridge log, so use it immediately
   after the shared run.

Version **0.6.0** adds the first **step 5b world-mesh renderer**: nearby opaque
and cutout Minecraft block models are drawn with Skyrim's camera and compared
against Skyrim's depth buffer. The user confirmed block appearance/removal,
stable world placement while moving, Skyrim walls hiding blocks correctly, and
continued operation of the older features. The initial three-block disappearance
estimate was corrected by the user to the intended roughly eight-block range.
Block-breaking crack animation and dropped-item rendering are not implemented.
Its packages are built. The original installer incorrectly blocked VS Code's Java
tools; its process check has been corrected, but that fix is **not tested**.
Fluids, translucent blocks, dropped items, animated entities and third-person
mirroring remain unsupported, so the full world-object scope of 5b is unfinished.

Version 0.5.0 implemented **step 5a: Minecraft hands and HUD over Skyrim**. The HUD
is Minecraft's on-screen interface, such as the hotbar and health display.
Minecraft sends a transparent image; Skyrim draws it over its scene. The user
confirms first-person arms/punch animation, held items, hotbar, health, hunger
and inventory appear inside Skyrim's window. Basic overlay composition passed
the user's gameplay check. Matching image logs have not been checked.
The hand/HUD pass is retained alongside the new world-mesh pass.

The user also reports a black Minecraft world view, invisible Minecraft terrain
still blocking movement, and no third-person rendering. Level drawing is disabled
in Minecraft's own window, including its third-person player model, but vanilla Minecraft collisions
remain active alongside Skyrim's added shapes. Superflat reduces terrain
obstacles; its floor still collides and is not aligned automatically to Skyrim.

The user confirmed step 4 mouse-look transfer works: moving Minecraft's camera
now turns Skyrim's camera. Automated look checks have not been rerun.

Step 3 added **sampled Skyrim collision and NPC positions** to the Fabric mod and
Rust SKSE DLL. Minecraft still sends its interpolated player position each render
frame. Skyrim follows it and sends nearby surface boxes and NPC records back.
Minecraft adds those boxes to its normal collision queries on both the client
and local server. Its movement and gravity code is unchanged.

The user's live run passed transport comparison: 117 received snapshots,
149,415 collision boxes and 117 NPC records matched Skyrim exactly. Skyrim's log
confirms PLUGIN_LOADED and HOOK_INSTALLED. Minecraft's collision-query logs show
nonzero matches on both client and server (12 client entries and 1 server entry
in the inspected run). In the subsequent run, the user confirmed movement in
Minecraft stopped at a Skyrim wall and NPC. The position comparison passed for
5,986 received frames out of 11,437 sent, with exact coordinates/partial ticks
and zero Skyrim target/read-back difference. The new world comparison also passed:
543 complete received snapshots, 1,354,585 collision boxes and 139 NPC records
matched exactly. These are records across snapshots, not unique objects.
The user subsequently confirmed jumping and landing work. Slopes and broad
collision coverage remain **not tested**. The user has now tested 0.5.0 in game.

**Use Minecraft's controls while connected.** Skyrim's player is a puppet: its
position is overwritten by Minecraft's latest position. Keyboard/mouse forwarding
from the Skyrim window into Minecraft is not implemented, so Skyrim movement
input will not drive the connected player. Keep Minecraft focused for movement
and configure Skyrim to continue running in the background as described below.
The user confirmed camera following after correcting the 0.4.0 installation.

The plan is in [docs/DESIGN.md](docs/DESIGN.md). The narrow, version-specific
Skyrim interface is documented in [docs/ABI.md](docs/ABI.md).

## Previous step 2 checks (not rerun for step 3)

- 14 Rust tests pass: 8 unit tests, 5 connection process tests, and 1 position
  lifecycle test. They cover connection recovery, protocol layout, coordinate
  conversion, invalid/stale samples, role ownership, and simulated puppet calls.
- A separate Java 25 process calls the actual native DLL and sends 180 synthetic
  render samples. Another Windows process receives 100 of them; every received
  coordinate and partial tick matches its sender exactly.
- The simulated puppet moves the actor and clears controller velocity; it
  restores gravity on inactivity, stale data, and resumption after a pause.
- The Fabric mod compiles against Minecraft 26.3 and Fabric Loader 0.19.5. The
  render method and its injection target exist in that game's class signatures.
- The DLL exports `SKSEPlugin_Load`, `SKSEPlugin_Version`, and the Minecraft
  bridge functions. Its loader accepts only runtime 1.7.104 and validates the
  matching format-5 Address Library before registering its callback.
- The comparison script rejects mismatched coordinates and empty receive logs.

## What is not tested / not implemented

- Slopes, general player/capsule behavior,
  background execution, and interaction with other mods: **not tested**.
- The earlier missing Address Library error is resolved for the user's latest
  run: Skyrim reached PLUGIN_LOADED and exported snapshots.
- The connection's abandoned-lock crash path and other PCs: **not tested**.
- Sampled collision and NPC transport and query injection are observed in live
  logs. Wall and NPC blocking passed the user's manual check; general collision
  accuracy and coverage remain **not tested**.
- First-person look transfer passed the user's manual check. Full camera
  placement, third-person mirroring, FOV/bob matching, Skyrim-window input
  forwarding, combat, and matched saves are not implemented. Nearby static
  block/depth drawing is implemented in 0.6.0 but **not tested**.
- Basic hands/items/HUD/inventory composition passed the user's gameplay check.
  Detailed transparency/color/orientation checks, resizing, pause/reconnection
  behavior and graphics-mod compatibility are **not tested**. This step does
  not create a mirror world.

## Step 3 behavior and limits

The exporter samples an eight-block cube around the player, using rays in both
directions on each axis at half-block spacing. It continues past up to eight
hits per ray, includes world collision layers, and ignores actor/player ray hits.
Each surface becomes a thin box behind the hit. Thin objects between rays can
be missed, curved/sloped surfaces are approximate, and doors can lag until the
next snapshot. This is SkyCraft's ray-sampling first version, not exact mesh export.

Work is spread across frames (about 2 ms or 384 casts per update, checked between
rays; one ray and lock waiting can exceed that target). A complete scan contains
1,536 rays plus continuation casts. Scans taking over one second or completed
after the player moves outside the useful center are discarded. Collision data
expires after 2.5 seconds; a stalled Minecraft render loop stops using it after
250 ms. Ordinary Minecraft blocks remain solid too. A sparse practice world makes
the exported geometry easier to distinguish. Stale/missing data falls back to
Minecraft's ordinary world; there is no invisible emergency floor.

NPCs are loaded high-process actors in the player's current Skyrim cell and
within eight blocks per axis. Their IDs and positions are exact transported
values, but their Minecraft body boxes are a fixed 0.6 by 1.8 blocks. Creature
sizes, animated hitboxes, neighboring-cell NPCs, NPC interaction and combat are
not supported by this step. Maximum snapshot: 8,192 surface boxes and 128 NPCs;
overflow disables the adapter with an error instead of silently truncating it.

Both halves must be upgraded together to 0.6.0 / protocol 6. Old protocol-2/3/4/5
plugins use different shared-memory names and cannot connect to this version.
The existing `logs/step2` folder is retained for all live bridge logs.

## Repeat the automated checks

From PowerShell in this project folder:

```powershell
.\tools\test-link.ps1
.\tools\test-position-native.ps1
```

Expect 14 Rust tests to pass, then the Java/native check to report:

```text
PASS: Java native smoke publisher sent 180 synthetic render samples.
PASS: 100 matching received frames (synthetic fixture); all Minecraft coordinates and partial ticks match exactly.
```

These commands do not launch either game. Connection-test logs are under
`passthrough\target\link-tests`; simulated and Java/native logs are under
`logs\step2-fixtures`. They are separate from real-game logs.

Close both games, then build, package and install both adapters using the
dependencies already cached here:

```powershell
.\tools\build-step5b.ps1 -Offline
```

Successful package builds now call `tools/install-build.ps1` automatically.
It writes to the Minecraft `mods` and Skyrim `Data\SKSE\Plugins` folders listed
in AGENTS.md. This uses the direct Skyrim installation; mod-manager profiles
may override those files, so use `-PackageOnly` and install the ZIP through your
manager if it controls this plugin.

To install the already-built package without rebuilding:

```powershell
.\tools\install-build.ps1
```

Expected: three `INSTALLED ... SHA256=...` lines and
`Install complete: 3 files copied and verified.` Existing mod files are backed
up under ignored `dist\install-backups`; older `passthrough-fabric-*.jar` files
are removed after copying succeeds. Other mods are untouched. If installation
fails, the script attempts to restore the previous files and reports any restore
failure. The script checks for Skyrim and Minecraft processes; unrelated Java
tools such as VS Code's language server are allowed. It also checks exclusive
write access to existing destination files before copying. If Java process
details are unavailable, it warns and relies on the file checks; keep both games
closed throughout installation. Raw process command lines are never printed.
This deployment script is **not tested**; it has not been executed against the
game folders.

Use `.\tools\install-build.ps1 -WhatIf` to preview the intended installation
without writing files (source and closed-game checks still apply). Use
`.\tools\build-step5b.ps1 -Offline -PackageOnly` to build without installing.
Older step build commands forward `-PackageOnly` too. Low-level Rust/Fabric
build commands do not install an incomplete pair of adapters.

The packages are:

- `dist\step5b\Passthrough-Skyrim-0.6.0.zip`: `SKSE\Plugins\passthrough_link.dll`.
- `dist\step5b\Passthrough-Minecraft-0.6.0.zip`: `mods\passthrough-fabric-0.6.0.jar`
  and `mods\passthrough_link.dll`.

Rust stable MSVC, compiler tools, Windows SDK libraries, Java 25, and Gradle 9.7.1
are needed. This PC's Rust, SDK libraries, and Gradle are cached in ignored
`.tools`. The build scripts select the installed MSVC tools and Java 25. On a
clean machine, both builds need dependency downloads before `-Offline` works.
Rust now uses Microsoft's `windows` crate (locked to 0.62.2) for D3D11/DXGI.
Run `.\tools\cargo.ps1 fetch` to cache those Rust dependencies. The renderer
requires Windows' D3D11.1 context interface and D3DCompiler_47 at runtime.

## Install and test in the games

Installation writes require your authorization under AGENTS.md rule 4. The
following is the manual procedure; it has **not been run**.

1. Install the [Address Library for Skyrim runtime **1.7.104**](https://www.nexusmods.com/skyrimspecialedition/mods/32444?tab=files) through your normal
   mod setup. SKSE must see `Data\SKSE\Plugins\versionlib-1-7-104-0.bin`.
2. Install the Skyrim zip through your mod manager, or put its `SKSE` folder
   under `E:\SteamLibrary\steamapps\common\Skyrim Special Edition\Data`.
3. Put the Minecraft zip's two `mods` files in
   `C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\mods`.
   Remove the previous passthrough JAR when upgrading; keep only the new version.
4. In this Prism instance's Java arguments, include
   `--enable-native-access=ALL-UNNAMED`. Use Java 25 and Fabric 0.19.5.
5. For a manual test with two windows, Skyrim needs to continue updating while
   Minecraft has focus. The usual Skyrim setting is `bAlwaysActive=1` under
   `[General]` in the active `Skyrim.ini` (the mod-manager profile's copy when
   applicable). This background configuration is **not tested** here. In
   Minecraft, use F3+P to disable pausing on lost focus if needed.
6. Start Skyrim through SKSE and load a practice save in an open area after the
   opening sequence. Start Minecraft through Prism and enter a local
   single-player world; keep Open to LAN off. Close both pause menus.
7. Walk a short distance in Minecraft, stop, then jump. Use short movements:
   Skyrim collision is sampled nearby and has incomplete coverage. Stop both
   games before comparing their completed logs.

For the direct Skyrim installation, the launch command is:

```powershell
& 'E:\SteamLibrary\steamapps\common\Skyrim Special Edition\skse64_loader.exe'
```

If using a mod manager, launch its SKSE entry instead so it supplies your mods.

A successful SKSE load writes this actual message to `logs\step2\skyrim-*.log`:

```text
plugin loaded runtime=1.7.104 event=PLUGIN_LOADED
```

The Fabric initializer writes `Passthrough Fabric plugin loaded` to Minecraft's
normal `logs\latest.log`. Its position data is in this project's
`logs\step2\minecraft-*.log`. The paths are compiled into this practice build,
so this build expects the project to stay at its current location.

To compare the latest pair of real-game logs, from the project folder:

```powershell
$mcLog = Get-ChildItem .\logs\step2\minecraft-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$skyLog = Get-ChildItem .\logs\step2\skyrim-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
.\tools\compare-position-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName
```

Expect a nonzero number of matching received frames, exact Minecraft coordinate
and partial-tick matches, and a Skyrim target/read-back difference no larger than
0.01 Skyrim units. A missing received frame is not counted as success.

### Check step 3 in the logs

After installing both new packages, load an offline practice world and a Skyrim
save near a floor, wall, and loaded NPC. Let sampling settle, then walk slowly
against the wall, jump and land, and approach the NPC. Expect Minecraft's normal
movement to stop at exported surfaces and NPC boxes. Step 2's matching SEND/RECV
player positions must continue. These are expected results, **not tested**.

Stop both games, select their latest bridge logs as above, then run:

```powershell
.\tools\compare-world-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName
Select-String -LiteralPath 'C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\logs\latest.log' -Pattern 'event=COLLISION_QUERY'
```

Expect the comparison to report complete matching snapshots, nonzero collision
boxes, and NPC records when near an NPC. `WORLD_SEND` in Skyrim must correspond
to `WORLD_RECV` in Minecraft with the same `sender`, `world`, `epoch`, and
`snapshot`. Every `BOX_SEND`/`BOX_RECV` and `NPC_SEND`/`NPC_RECV` pair carries
identical coordinates. `WORLD_SCAN` gives ray counts, hit counts, rays reaching
the hit limit, and scan duration. In Minecraft's normal log, expect
`COLLISION_QUERY side=client` and `side=server`, with nonzero `matches` when near
a surface. Query logs are limited to one per second per side. The comparison
proves transport only; it does not prove live movement or accurate geometry.

### Check step 4 mouse look

If Minecraft produces no bridge log after upgrading, check its normal
`logs/latest.log`. The reported 0.4.0 startup failure was a mixed installation:
the new JAR was present but `mods/passthrough_link.dll` still matched 0.3.0.
That DLL lacks `pt_mc_frame4`, causing `NoSuchElementException` in NativeLink
before the bridge logger starts. The agent replaced this instance's outdated DLL
with user approval and confirmed its hash matches the 0.4.0 package. For future
upgrades, close Minecraft and replace **both** the JAR
and DLL from the same current Minecraft package. Skyrim's protocol-4 startup
succeeded in that run. The user subsequently confirmed camera following works.

Install both current packages together, removing the older Fabric JAR. Keep
Minecraft in first-person view and focused; leave both pause menus closed.
Skyrim must run in the background. Slowly look left/right, up/down, then turn
through a complete circle. Expected: Skyrim's first-person view follows the same
direction without reversing axes or jumping at the angle wrap. The initial view
may turn to align with Minecraft's absolute direction. Skyrim's native camera
animation and one-frame update delay can remain; FOV and camera position are
not synchronized in this step.

Ordinary Skyrim third-person view temporarily switches to first-person while
following Minecraft. It is restored on release only if that same camera is still
in first-person. Other camera states (mounts, furniture, death, free camera and
scripted transitions) are left alone by the look adapter.

Stop both games, select `$mcLog` and `$skyLog` as above, and run:

```powershell
.\tools\compare-position-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName -RequireLook
```

Expect position PASS plus a nonzero number of matching look frames, with at most
0.00001 radians conversion/read-back error. `mc_yaw`/`mc_pitch` are degrees in
SEND and LOOK_RECV. Skyrim logs target and read-back radians plus its camera-state
rotation as `camera_qw/qx/qy/qz` (a four-number rotation representation). Those
camera values are sampled before the next normal camera update; the PASS checks
transport and actor angles, not the final rendered camera. If the view does not
follow, send both logs and report whether left/right, up/down, or both failed.

## Read the position logs

Match `sender`, `world`, and `frame` between `event=SEND` and `event=RECV`.
Both lines carry `partial`, `mc_x`, `mc_y`, and `mc_z`. Skyrim also records
`target_x/y/z` and `actual_x/y/z`, read from the actor immediately after its move.

An `ANCHOR` line records how the initial Minecraft sample was aligned with the
current Skyrim position. After that, one Minecraft block moves 70 Skyrim units:
Minecraft X maps to Skyrim X, Minecraft Z to negative Skyrim Y, and Minecraft Y
to Skyrim Z. This avoids an initial teleport to an unrelated part of Skyrim.

The connection holds the latest frame, so Skyrim may skip intermediate Minecraft
frames. It never interpolates the received position again. Samples older than
250 milliseconds, inactive worlds, loading, or paused updates release the puppet;
resuming establishes a fresh alignment. The two adapters use a dedicated
`Local\PassthroughPractice_link6_...` connection, separate from SkyCraft and the
older protocols. Use logs to check behavior; no visual game inspection is
required.

## Check step 5a hands and HUD (basic appearance confirmed by user)

Build with `.\tools\build-step5b.ps1 -Offline`, which installs both current packages
using the procedure above. Replace **both** Minecraft files: the JAR and DLL.
Restart both games after installation. Use first-person Minecraft, a visible
hotbar (F1 off), and a window no larger than 2560 x 1440. Start at 1280 x 720
to reduce copying cost. Keep both windows visible, both pause menus closed,
and Minecraft focused. Use the same aspect ratio in each game to avoid stretching.

Expected: Skyrim shows Minecraft's hand/held item and hotbar over the Skyrim
scene. Move the mouse and change the selected hotbar slot. Minecraft's own world
view becomes a dark background with hands/HUD while connected; use Skyrim's
window to view the combined scene. Input still comes from Minecraft's window.
The image updates at up to 30 frames/second. Movement/look transmission remains
per render frame. Minecraft blocks were absent in 5a; 0.6.0 adds the supported
nearby block models to Skyrim through a separate rendering pass.

Stop both games, then run these exact commands from the project folder:

```powershell
$mcLog = Get-ChildItem .\logs\step2\minecraft-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$skyLog = Get-ChildItem .\logs\step2\skyrim-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
Select-String -LiteralPath $mcLog.FullName -Pattern 'event=OVERLAY_(SEND|ERROR)\b' | Select-Object -Last 8
Select-String -LiteralPath $skyLog.FullName -Pattern 'event=OVERLAY_(HOOK|READY|RECV|DRAW|DISABLED)\b' | Select-Object -Last 16
Select-String -LiteralPath 'C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\logs\latest.log' -Pattern 'event=OVERLAY_' | Select-Object -Last 12
```

Expected log evidence:

- Skyrim startup: `OVERLAY_HOOK installed=1`, then `OVERLAY_READY` when connected.
- Minecraft: `OVERLAY_SEND`; Skyrim: `OVERLAY_RECV` with matching `sender`,
  `world`, `frame`, `width`, `height`, `bytes`, `checksum` and `alpha_pixels`.
  Intermediate frames can be skipped. `checksum` is a numeric fingerprint of
  all image bytes, recomputed in Skyrim. `alpha_pixels` counts pixels that are
  not fully transparent; with a visible hand/HUD it should be greater than zero.
- Skyrim: `OVERLAY_DRAW` for received frame identities. It means the graphics
  draw was submitted, not proof of correct appearance or completed GPU work.
- Minecraft's normal log: `OVERLAY_MODE active=true`, `OVERLAY_BUFFER` with
  `RGBA8_UNORM`, and `OVERLAY_CAPTURE ... sent=1`. `sent=0` means a skipped stale
  or contended transfer. `OVERLAY_DISABLED` or `OVERLAY_ERROR` needs investigation.

Timing fields are measured on the CPU: `readback_age_us` includes waiting until
a later frame polls the completed GPU copy; `transfer_us` covers checksum and
shared-memory publication; `validate_upload_us` covers checksum and texture
upload submission; `submit_us` covers drawing setup/submission. They do not
measure GPU execution time. `skipped` counts capture attempts with all three
readback buffers busy. `age_ms` is time since Minecraft began rendering that image.

Also try pausing, resuming, changing Minecraft's window size, and exiting its
world. Expected: old overlays disappear on inactivity; normal Minecraft world
rendering returns when Skyrim stops reporting ready (within 500 ms). Retry at
a supported size if `OVERLAY_SIZE_UNSUPPORTED` appears. A renderer error disables
only the overlay until restart; position/look and collision continue. These
lifecycle and error paths are **not tested**.

Limits: paused menus are not mirrored; crosshair inversion against Skyrim,
exact color/gamma matching, matching Minecraft's own camera to Skyrim, graphics
mods and GPU device replacement are not handled. Hands/HUD draw over Skyrim UI as
well as its scene while active. Three readback buffers and a single latest-image
mailbox bound memory use; pixel data stays in memory and is never saved here.

## Check step 5b nearby blocks and depth (0.6.0, not tested)

The packages are already built. Close both games and run:

```powershell
.\tools\install-build.ps1
```

Expect three `INSTALLED` lines and the installation-complete message. This
installs `dist\step5b`; no rebuild is needed. If Minecraft/Skyrim or a locked
destination is reported, close the relevant application before retrying;
VS Code's Java tools can stay open. The installer does not stop processes.
Use `-PackageDirectory .\dist\step5` to
restore the earlier 0.5.0 package if needed, with both games closed.

Start both games offline with Minecraft in first person. Keep Minecraft focused
and Skyrim updating in the background. Load a practice Minecraft world and a
Skyrim save near a wall and floor. In a Minecraft world with commands enabled,
these chat commands provide a small, repeatable block to inspect in Skyrim:

```text
/setblock ~2 ~1 ~ minecraft:stone
```

Expected: the block is visible in Skyrim when facing it and stays fixed to the
world while you turn and walk. The export covers a 16-by-16-by-16 block cube
around Minecraft's player (about eight blocks each way). Move around the block;
stand so a Skyrim wall falls between your camera and it. The wall should hide
the block. Move back around the wall; the block should reappear. Nearby Minecraft
ground and other opaque/cutout block models should now be visible too. Remove
the test block in Minecraft and expect its exported geometry to disappear.
Hands and HUD should continue drawing over the combined world.

Stop both games and run:

```powershell
$mcLog = Get-ChildItem .\logs\step2\minecraft-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$skyLog = Get-ChildItem .\logs\step2\skyrim-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
Select-String -LiteralPath $mcLog.FullName -Pattern 'event=SCENE_(SEND|ERROR)\b' | Select-Object -Last 10
Select-String -LiteralPath $skyLog.FullName -Pattern 'event=SCENE_(RECV|DEPTH|DRAW|WAIT|DISABLED)\b' | Select-Object -Last 20
Select-String -LiteralPath 'C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\logs\latest.log' -Pattern 'event=SCENE_' | Select-Object -Last 10
```

- `SCENE_SEND` and `SCENE_RECV` must match by `kind`, `sender`, `world`,
  `snapshot`, `atlas`, `count`, `height`, `bytes`, `checksum`, and origin XYZ.
  `kind=2` is an atlas: count/height are its width/height. `kind=1` is a mesh:
  count is its number of triangle vertices; height is zero. Some mesh snapshots
  may be skipped because only the latest complete one is kept.
- Expect `SCENE_DEPTH usable=true` with nonidentical min/max samples, followed
  by `SCENE_DRAW vertices=...` for nonempty meshes. `sky_reversed` is the inferred
  Skyrim depth convention; `mesh_reversed` comes from Skyrim's camera matrix.
  They need not match: Skyrim's depth is converted to distance before comparison.
- `projection_x/y` should be near zero and `projection_w` positive for the
  diagnostic point straight ahead. Draw submission does not prove correct
  appearance or occlusion; the wall check above is still required.
- A short `SCENE_WAIT reason=depth_not_yet_usable` during initialization is
  expected. Persistent WAIT, SCENE_ERROR or SCENE_DISABLED needs investigation.
  Send both project logs plus Minecraft's `latest.log` if drawing fails.

The first version exports opaque/cutout block models using Minecraft's tessellator.
It uses the first animation frame of each atlas sprite and simplified Minecraft
brightness. It attempts one full snapshot every 100 ms with a 3 ms per-frame
geometry budget. Meshes older than two seconds expire. Maximums: 262,140 vertices
and a 4096-by-4096 texture atlas; excess data disables only world export, with a
log error. Native validation, initial atlas copying, GPU upload and logging add
work outside that geometry budget. CPU timing logs do not measure GPU execution.

Limits: translucent block faces are skipped and counted; fluids, dropped items,
block-entity models, mobs and the third-person player model are not exported.
Atlas animations, Skyrim lighting/shadows/fog and post-processing integration are
not included. The pass draws onto the finished Skyrim frame before the HUD and
does not alter Skyrim's own depth buffer. Other mods' depth replacements, MSAA,
partial camera viewports and device replacement are unsupported. Depth convention
is inferred from 16 GPU samples as in SkyCraft's fallback path; uniform or invalid
samples suppress world drawing. Numerical transport, wall occlusion, performance,
resource reloads and lifecycle behavior are **not tested**.










