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

Gathered 2026-10-11 from README, STATUS, MODLOG and this file. Items are
unfinished or untested things already recorded there, grouped by area. Nothing
here is planned in detail yet.

Combat and health:
- Death: a Minecraft death now kills the Skyrim character (0.10.6, untested).
  Still open: a Skyrim-only death does not reset Minecraft health, and no death
  message from Minecraft is shown in Skyrim's window.
- A Skyrim hit that lands while the bridge is not ready still lowers Skyrim
  health; protection while disconnected or recovering is undecided.
- Other incoming damage sources (falls, magic, scripted health changes) and
  full healing behaviour are unverified.
- **Rapid small Skyrim hits are mostly lost (measured 2026-10-11).** Minecraft
  refuses a new hit for half a second after the last one unless it is larger.
  Skyrim's continuous damage (a flame or frost stream, burning, poison) arrives
  as dozens of tiny hits per second, each used up whether or not Minecraft took
  it. Report 20261011-094530: of 99 hits under 2 Skyrim damage, 7 were taken;
  12.76 half-hearts were requested and 0.69 applied. Ordinary weapon blows are
  fine (9 of 11 taken, the other 2 shield-blocked). Candidate fix: add up the
  refused amounts and deliver them with the next hit Minecraft will accept.
- Critical hits (0.10.13), weapon wear (0.10.14) and armour/shields against
  Skyrim hits (0.10.15) are verified for a wooden sword, fists, 14 armour
  points and a shield. Axes and other weapon types are untested. Still missing:
  enchantment damage (Sharpness and the like), temporary effects such as
  Strength, sweep attacks and knockback on enemies.
- Shield blocking and knockback take their direction from the nearest Skyrim
  character within about 8 blocks, which may not be the real attacker.
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
  pixels-per-unit mapping when the user asks.
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
  friendly), and whether R then stays a separate key.
- No text entry, no held-key repeat; rebinding and non-dialogue menus are
  largely untested.
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
- Long-distance and high-altitude travel need the coordinate-window remapping
  planned under "Next priority" below.
- Save revisions/rollback, automatic choice of the matching Minecraft world,
  and restoring historical inventory are not implemented.
- Collision coverage is a four-block sampled cube; thin objects can be missed
  and fast movement can cause holds.

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
- Sprint state is not logged; Skyrim-window keyboard/mouse forwarding into
  Minecraft does not exist.

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

