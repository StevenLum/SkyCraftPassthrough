# 0.10.18 crosshair target kind (current)

Protocol 29 / Local\PassthroughPractice_link29_. World magic PTWORL13, size
398,024 (was 398,016). Eight bytes are inserted after the NPC slots and before
the damage section: u32 target kind at 397,440 (world.rs TARGET_AT, Java
NativeLink.TARGET_KIND_AT = WORLD_SIZE-48-528-8), u32 reserved at 397,444. The
damage section, recovery fields and lineage ID keep their end-relative offsets
(damage section now starts at 397,448). Connection record (456 bytes) and scene
packets are unchanged. All three files update together.

Target kinds (validated <= MAX_TARGET_KIND, test required for new ones):
0 none, 1 other object, 2 container, 3 living character, 4 dead character,
5 door. Set with the crosshair handle whenever an admitted snapshot is
published: LookupReferenceByHandle (AE 12332, released through the existing
reference wrapper); reference form type u8 at +0x1A; for 0x3E (character) dead
means life state 1 or 2 (u32 at +0xC8, bits 21..24) or health not above 0;
otherwise the base object pointer at +0x40 (TESObjectREFR.h data.objectReference)
and its form type at +0x1A: 0x1C container, 0x1D door (FormTypes.h). The reads
on arbitrary references are from CommonLib and not yet verified in game.
SCENE_CROSSHAIR logs handle and kind on change.

Minecraft: the click queue of options.keyInventory is drained in the
handleKeybinds HEAD hook when (a) a Skyrim menu is active, sending menu command
5, or (b) the bridge is admitted, no screen is open and the fresh kind is 2 or
4, queuing the same activation request as the Interact key. Otherwise the click
is left for vanilla.

# 0.10.17 left click (no layout change; protocol 28)

In a Skyrim menu the left mouse button sends menu command 10 ("Click", mouse
device 1, button 0) instead of 5 ("Accept"). Verified path: 27 command-10
dispatches in report 20261011-094530 and the user's report that it acts like a
left click in Skyrim.

# 0.10.10 to 0.10.16 (no layout change; protocol 28)

Menu command field (u32 at 296) values: 1 Up, 2 Down, 3 Left, 4 Right,
5 Accept, 6 Cancel, 7 Quicksave, 8 Quickload, 9 "XButton" (keyboard, scan code
0x13, Skyrim's R in item menus), 10 "Click" (mouse device 1, button code 0).
1-6, 9, 10 need an open Skyrim menu and its context number; 7, 8 need context
0 and normal play. menu_input.rs holds the lists (MENU_COMMANDS,
GAMEPLAY_COMMANDS), the event table event() and the pure rule verdict(); a test
requires every number to appear in exactly one list and the table. "XButton"
and "Click" come from CommonLib UserEvents.h; that Skyrim's handlers act on
them when forwarded is not verified.

Scene drawing: scene.rs mesh_origin() returns None when a mesh origin is more
than MESH_REACH = 64 blocks from the sample's Minecraft position on any axis,
or cannot be expressed as a Skyrim position; scene_draw.rs then logs SCENE_WAIT
reason=mesh_from_another_place and draws nothing that frame. Previously the
second case was an error that disabled the world pass until restart.

Pointer: the measured pixels-per-count ([f32;2]) is kept in the menu state for
the process lifetime and used as the starting gain of each new menu context.

Minecraft side (no native change): critical hits multiply the requested attack
damage by 1.5; weapon wear uses DataComponents.WEAPON itemDamagePerAttack via
ItemStack.hurtAndBreak(int, LivingEntity, EquipmentSlot) on the server thread;
incoming hits use DamageTypes.MOB_ATTACK from Registries.DAMAGE_TYPE with
new DamageSource(holder, Vec3) or new DamageSource(holder), falling back to
damageSources().generic(). In the 26.3 data, generic is in the bypasses_armor
tag and bypasses_shield includes that tag; mob_attack is in neither and is not
in no_knockback.

# 0.10.9 places: derived origins (no layout or record change)

Protocol 28, packets, PBID and PBOR records all unchanged. The single saved
origin (PBOR) is now the origin of its own place only. saved_origin.rs
Origin::for_location(cell, worldspace) returns: None for cell 0; the saved
origin when same_location holds (same nonzero worldspace, or same interior
cell); otherwise a derived origin with the saved Minecraft world ID, the
current cell/worldspace IDs, Skyrim origin (0,0,0) and Minecraft origin:
- worldspace == 0 (interior): key = cell form ID. x = ((key & 0xFFF) - 2048) *
  2048, y = 128, z = (((key >> 12) & 0xFFF) - 2048) * 2048.
- worldspace != 0: key = worldspace form ID. x = 4,718,592 + (key & 0xFF) * 8192
  + 4096, y = 136, z = (((key >> 8) & 0xFF) - 128) * 8192 + 4096.
- If |x - saved mc x| < 16,384 and |z - saved mc z| < 16,384, recompute with the
  row's top bit flipped (0x800 for interiors, 0x80 for worldspaces).
Runtime form IDs are used as loaded; bits above 23 (interiors) or 15
(worldspaces) are ignored. Collisions between different forms with equal low
bits are not detected. Axis mapping and 70 units per block are unchanged, so
derived Minecraft y = base height + Skyrim z / 70.

Origin::restore uses that origin: Err(3) wrong Minecraft world, Err(4) only for
cell 0, Err(2) invalid sample, Err(5) when the Minecraft position maps further
than 140 units from Skyrim's player or cannot be expressed at all (previously a
too-distant position returned 2 and never recovered). save_identity origin(),
recovery_target() and menu_authorized() use for_location. A derived origin is
logged as SCENE_ORIGIN state=derived. Nothing derived is written to any save.

Minecraft: BridgeRecovery logs SCENE_RECOVERY state=rejected
reason=outside_minecraft_world and sends one chat line per request when the
target is nonfinite, beyond 29,999,000 blocks, or outside the level's height.
The saved home origin of the current user maps Minecraft y -64..319 to Skyrim
z about -11,600..15,000; places outside that cannot be reached yet.

# 0.10.8 death repair (no layout change)

Protocol 28 and all layouts as 0.10.6/0.10.7. World header u32 at 124 (hold
reason) is now validated against world.rs MAX_HOLD_REASON = HOLD_DEAD = 7; in
0.10.6/0.10.7 the check still said 6, so the sender's own strict encode check
rejected every reason-7 reply ("invalid world snapshot") and the Skyrim adapter
disabled itself. Any new reason must raise the constant. Hold condition changed
to: health (ActorValueOwner slot 1, value 24) not above 0, or within 3000 ms of
a kill attempt. Life state (Actor+0xC8 bits 21..24) is only logged as
SCENE_DEATH_LIFE: it read 0 immediately after the lethal DoDamage call returned
(health already -200), so the offset is unconfirmed. Observed in game: DoDamage
with null source and dontAdjustDifficulty=true returned false, reduced health
by exactly the requested 300, and Skyrim's death and automatic reload followed.
Report 20261011-005940 confirms the life-state location: it read 1 (dying) on
the frame after the lethal call, 2 (dead) 2.5 s later and 0 after the reload.
It is one frame behind the health change, which is why the hold uses health.

# 0.10.7 quicksave/quickload commands (no layout change)

Protocol 28 and all layouts as 0.10.6. Menu command field (u32 at 296) gains
values 7 = quicksave and 8 = quickload, sent with menu context 0 (u64 at 304).
Native accepts 7/8 only when: handshake and active sample are fresh, context is
0, request age <= 250 ms, >= 1000 ms since the previous 7/8 dispatch, SKSE
reports a loaded game, no UI counter blocks (same test as interaction), and
the published save lineage equals the sample's. Otherwise SCENE_MENU_RECV logs
state=inactive/context_changed/expired/cooldown/loading/menu_open/unpaired_save.
Dispatch is the existing synchronous keyboard ButtonEvent down/up pair through
MenuControls::ProcessEvent with user event "Quicksave" (scan code 0x3F, F5) or
"Quickload" (0x43, F9), names from CommonLib UserEvents.h. MenuControls holds a
QuickSaveLoadHandler (MenuControls.h offset 0x60); its acceptance of a forwarded
event is not verified. No BGSSaveLoadManager call is made.
Minecraft: two KeyMappings (key.passthrough.quicksave/quickload, defaults
InputConstants.KEY_F5/KEY_F9, category MISC) appended in the Options.load mixin.
Minecraft.handleKeybinds HEAD drains options.keyTogglePerspective clicks in an
eligible bridge world while either mapping shares its key (KeyMapping.same).

# 0.10.6 death ABI

Protocol 28 / Local\PassthroughPractice_link28_. Connection record 440 -> 456
bytes: u64 death number at 440 (0 = none), u64 death time at 448 (sender's
boot-relative milliseconds). pt_mc_frame14 appends u64 death_seq and u32
death_age_ms after cursor_y; the native sender stores now - age (minimum 1) and
sends 0/0 on inactive frames. Java descriptor adds JAVA_LONG, JAVA_INT. World
packet layout unchanged; header u32 at 124 (hold reason) gains 7 = player dead
or dying, reply carries no geometry and admitted=0.

Native (pinned local CommonLib; runtime behaviour not verified):
- Life state: Actor.h AsActorState is Actor+0xC0 from 1.6.629 on; ActorState.h
  actorState1 at +8, so u32 at Actor+0xC8, lifeState = bits 21..24
  (0 alive, 1 dying, 2 dead). Health: ActorValueOwner (Actor+0xB8) slot 1,
  actor value 24, as already used.
- Kill: Actor::DoDamage AE ID 37335, bool(Actor*, f32, Actor* source, bool
  dontAdjustDifficulty), called on the player with amount
  max(health,0)*2+100, null source, true. The puppet is released first.
  A flag makes the CheckClampDamageModifier hook pass this hit through
  (SCENE_DAMAGE_PASSTHROUGH reason=bridge_death).
- Hold while (life is 1 or 2 and health <= 0) or within 3000 ms of a kill
  attempt. Checked after lineage admission and before origin/recovery work.
  One kill per sender/world/death number, only if age <= 10000 ms.
Minecraft: ServerPlayer.die(DamageSource) HEAD observer (not cancellable) for
marked bridge worlds on an unpublished integrated server; published for 10 s.
BridgeRulesMixin also sets GameRules.IMMEDIATE_RESPAWN and KEEP_INVENTORY true
at every load of a bridge world.

# 0.10.5 damage balance (no ABI change)

Protocol 27, 440-byte record, pt_mc_frame13 and all packet layouts are unchanged
from 0.10.4. Only the Java-side factor changed: attack_damage sent at offset 412
is now Minecraft attack damage x 1.75 x recharge multiplier (was x 10). Native
validation (finite, above 0, at most 1000) and DoDamage call are unchanged.
SCENE_ATTACK_SEND adds a `scale` field. Plugin version field is 0x000A_0005.

# 0.10.4 dialogue pointer ABI (current)

Protocol 27 / Local\PassthroughPractice_link27_. Connection record grows from 424
to 440 bytes (header size field 440, layout version still 2). New fields, little
endian: u64 pointer context at 424 (0 = no pointer motion), f32 total X at 432,
f32 total Y at 436. Totals are running mouse motion since that menu context
began, in Minecraft window widths/heights; the receiver uses differences only.
Nonfinite totals make a sample invalid. pt_mc_frame13 appends u64 cursor_context,
f32 cursor_x, f32 cursor_y after attack_damage; Java descriptor adds JAVA_LONG,
JAVA_FLOAT, JAVA_FLOAT. Inactive frames and nonfinite inputs send context 0.
World (PTWORL12/398016) and scene packets unchanged. All adapters update together.

Native, from the pinned local CommonLib checkout (not verified in game):
- MenuCursor singleton pointer slot: AE Address Library ID 403551 (MenuCursor.cpp).
  Non-VR members: cursorPosX 0x4, cursorPosY 0x8, screenWidthX 0x14,
  screenWidthY 0x18, cursorSensitivity 0x1C (MenuCursor.h). Read only.
- MouseMoveEvent vtable: AE ID 236703 (Offsets_VTABLE.h). Layout 0x30 bytes
  (MouseMoveEvent.h/IDEvent.h/InputEvent.h): vtable 0, device u32 at 8,
  event type u32 at 0xC, next at 0x10, user-event BSFixedString at 0x18,
  idCode u32 at 0x20, mouseInputX i32 at 0x28, mouseInputY i32 at 0x2C.
- Event sent: device 1 (mouse), type 1 (mouse move), user event "Cursor",
  idCode 0xA, through MenuControls::ProcessEvent vtable slot 1 with the existing
  isProcessing/remapMode guard, string ctor/release and UI-task thread.
  The user-event name and idCode follow Skyrim's control map from memory, not
  from a local file; treat both as unverified until the log shows the pointer move.
Either missing ID disables only the pointer (SCENE_CURSOR state=disabled); menu
keys keep working.

Per UI task (about every 20 ms) while the menu context matches: add the change
in total times screen size to an owed amount (clamped to one screen), request
round(owed/gain) counts per axis (limit 4096), dispatch, read the pointer back,
subtract the requested amount from owed, and relearn gain per axis from moves of
8+ counts ending between 5% and 95% of the screen with |ratio| in 0.05..20.
Initial gain is cursorSensitivity when in 0.05..20, else 1. A new sender/world/
context only records a baseline. Pointer numbers must be finite and screen size
within 64..16384 or nothing is dispatched.

Minecraft: MouseHandler.turnPlayer(double) HEAD, cancellable, shadows
accumulatedDX/accumulatedDY (javap on the cached 26.3 jar). Captured only while
menu status is fresh and no Minecraft screen is open. Minecraft.startAttack HEAD
is now cancellable: in a Skyrim menu it queues menu action 5 (confirm) and
returns false; continueAttack(true) is cancelled while the menu status is fresh.
Outside menus both behave as before. Runtime mixin application is not tested.

#0.10.3 recharge protocol

Protocol26 / Local\PassthroughPractice_link26_. Payload remains424 bytes, frame12,
attack damage f32 at412; world remains PTWORL12/398016. Version bump isolates old
500ms native cooldown from new100ms guard. Java sends final recharge-scaled damage;
no new native fields. Charge uses client ticks and held component ATTACK_SPEED,
scale0.2+0.8*q^2; capture once per emitted request. No acknowledgement added.

# 0.10.1 held-item damage ABI

Protocol25 / Local\PassthroughPractice_link25_. Connection remains424 bytes;
previous reserved412..416 now f32 attack_damage (little endian). pt_mc_frame12
appends f32 attack_damage after attack_handle; Java descriptor adds JAVA_FLOAT.
Snapshot magic/size unchanged (PTWORL12/398016). Both adapters require update.
Minecraft captures ATTACK_DAMAGE*10 at click time. Native rejects nonfinite,
nonpositive or >1000 requests and passes the amount to existing DoDamage.
The request sequence, deduplication, age, range and target checks remain unchanged.

# 0.10.0 outgoing punch ABI

Protocol24/Local\PassthroughPractice_link24_. Connection424 bytes: attack sequence
u64 at400, handle u32 at408, reserved412..416, first-request timestamp u64 at416.
pt_mc_frame11 appends u64 attack_seq,u32 attack_handle after damage_ready. Native
supplies timestamp on first new world/sequence; requests expire250ms.
World magic PTWORL12, size398016 unchanged. Reserved damage-section+12 u32 now
contains Skyrim crosshair handle; all damage event/recovery/lineage offsets unchanged.

Crosshair singleton401585, handle+4; lookup12332 retains reference using existing
ActorRef Drop path. Actor form type3E at+0x1A, form ID+0x14, position+0x54,
cell+0x60; current health ActorValueOwner+0xB8 slot1. DoDamage AE37335 signature
bool(Actor* this,float healthDamage,Actor* source,bool dontAdjustDifficulty).
Use amount10, sourceplayer, false. Validate target equals current crosshair,
nonplayer actor, samecell, finite distance<=210 and positive finite health.
Minecraft startAttack HEAD observer uses CallbackInfoReturnable<Boolean> and
never cancels vanilla processing. Logs prove dispatch/results only after testing.

# 0.9.9 interception change

Protocol23/packet layouts unchanged. Failed ModActorValue interception removed.
Damage hook now primary PlayerCharacter vtable slot0x127 per CommonLib non-VR
Actor::CheckClampDamageModifier(ActorValue,float)->float. This pointer is Actor*,
not ActorValueOwner*. Call previous function once; log requested/returned delta
and current health; enqueue a finite negative health result if ready, return zero
after capture or original result otherwise. Runtime invocation and suppression
semantics unverified; second bounded attempt, not confirmed engine behaviour.

# 0.9.8 incoming-damage ABI (current)

Protocol 23, Local\PassthroughPractice_link23_. Connection grows to 400 bytes:
u64 damage generation at 376, u64 consumed sequence at 384, u32 ready at 392.
pt_mc_frame10 appends these three arguments after recovery_ack; active=false
forces ready=0. Existing menu/status/position offsets unchanged.

World magic PTWORL11, size 398016. Damage section at SIZE-48-528 (397440):
u64 generation at +0, u32 count at +8, reserved +12; up to 32 events at +16,
stride 16: u64 sequence, f32 fraction, f32 raw Skyrim amount. Events strictly
increase in sequence and require finite positive amounts. Existing recovery
and lineage fields retain offsets relative to packet end. Collision counts are
independent of damage counts. Both adapters must update together.

Native player instance ActorValueOwner+0xB8 vtable slot 6 is ModActorValue(this,
u32 modifier,u32 actorValue,f32 delta). Modifier kDamage=2; health=24. Install
once under a mutex, storing/chaining the original. No other actor's table selected.
Health maximum follows CommonLib: owner slot 2 GetPermanentActorValue(24) plus
Actor::GetActorValueModifier(this,1,24), AE Address Library ID38469. Hook only
suppresses finite negative health deltas after queue insertion with a 250ms lease.
Queue generation binds peer/world/IDs/load revision; capacity32, five-second expiry.

Minecraft validates pairing, freshness, world and live player; server consumes
sequence before hurtServer(overworld,damageSources.generic(),fraction*maxHealth).
Accepted=false is still consumed (vanilla immunity/cooldown/creative behaviour).
Readiness is server-refreshed, and damage state/deduplication is shared across
threads. Capture/apply/ack/drop/passthrough/ready diagnostics describe outcomes.
This is session-local at-most-once application, not crash-persistent exactly-once
transactions. Mutation path and scaling remain not tested in-game.

# 0.9.7 damage callback probe

Packet ABI/protocol 22 unchanged. PlayerCharacter primary vtable slot 0x104 is
HandleHealthDamage(Actor* attacker, float damage) for non-VR per CommonLib Actor.h
and Actor.cpp RelocateVirtual(0x104,0x106). Wrapper signature includes player this,
attacker and f32. Original pointer stored before patching; wrapper calls original
once, samples current health through ActorValueOwner+0xB8 slot 1 before/after,
and emits SCENE_DAMAGE_PROBE. No gameplay damage interception/suppression yet.
Vtable bound validation includes the new slot. Runtime semantics not tested.

# 0.9.5 health supplement (current)

Protocol 22, Local\PassthroughPractice_link22_, world magic PTWORL10 (8 bytes).
World size stays 397488 and connection stays 384; pt_mc_frame9 unchanged.
Previously reserved world header 68..72 now holds little-endian f32 current
Skyrim player health. Decode rejects nonfinite values. Recovery/lineage offsets
unchanged. Only active admitted snapshots emit SCENE_HEALTH_SEND/RECV.

CommonLib AsActorValueOwner uses subobject 0xB8 for 1.7.104 (since 1.6.629); vtable slot 1 is
float GetActorValue(ActorValue) const; health enum 24. Pass the subobject pointer
as this. Read on player update thread at completed collision snapshot export.
This is current health, not base/maximum, and finite negative values are retained.
No actor-value setters or damage APIs are called. Runtime reading not tested.

# 0.9.4 menu state ownership supplement

Protocol 21 and packet layouts unchanged. SKSE UI callbacks may execute on
multiple threads. One process-wide Mutex<State> serializes context/deduplication
and dispatch. try_lock skips contention/reentrancy; poison disables menu tasks.
Shared-memory mapping handles remain thread-local and are passed by reference
only during a callback. No raw handle/pointer is transferred between threads.
SCENE_MENU_STATE/RECV/RESULT now include Rust callback thread ID for diagnosis.
Native event ABI and mappings remain unchanged and require runtime verification.

# 0.9.3 recovery supplement (current)

Protocol 21; Local\PassthroughPractice_link21_. Connection remains 384 bytes;
u64 recovery acknowledgement at 368 (previously reserved). pt_mc_frame9 appends
u64 recovery_ack after menu_context. Position decode/encode preserves this field.

PTWORLD9 size = 128 + 8192*48 + 128*32 + 48 = 397488 bytes. Existing header, shape
and NPC offsets remain unchanged. At SIZE-48: u64 recovery token; SIZE-40/32/24:
three f64 Minecraft target coordinates. The lineage ID remains the final 16 bytes.
Recovery snapshots are active, unadmitted, origin_block=5; no collision geometry.
Native handshake/session/world and packet age validation precedes Java handling.
Java additionally requires matching expected lineage, age <=250ms and build bounds.
Normal snapshots have token zero. Both adapters must update together.

Native pending request binds load revision/session/world/lineage/Minecraft UUID/
cell/worldspace, expires after 30 seconds, and rejects Skyrim drift >140 units.
Acknowledgement requires matching token and interpolated Minecraft position within
0.05 blocks. Minecraft sends it after integrated-server teleport reports target
position and the local client reaches within 0.05 blocks. Server callbacks recheck
request identity/freshness/world/player; one applied teleport per request survives
transient snapshot gaps. No origin or save-record serialization changes.

# 0.9.2 location-check supplement

No protocol or save layout change (protocol 20, PBOR 72 bytes). Saved origin's
cell remains the original cell. same_location requires a nonzero current cell
and equal worldspace; if worldspace is zero it also requires equal cell IDs.
Thus exterior cells in a shared worldspace retain the same coordinate origin.
Restore still validates Minecraft UUID and target proximity <=140 Skyrim units.
Menu authorization uses the same location predicate. Accepted distinct exterior
cells log SCENE_ORIGIN state=outdoor_cell_accepted with both cells and worldspace.

# 0.9.1 menu input supplement (current)

Protocol 20; Local\PassthroughPractice_link20_; connection size 384, layout 2.
Existing position, lineage, Minecraft UUID and activation fields retain offsets.
Menu request: u64 sequence at 288, u32 action at 296, u64 context at 304,
u64 first-request boot timestamp at 312. Actions 1..6: Up, Down, Left, Right,
Accept, Cancel. Status: u32 active at 320; u64 context/boot timestamp/Skyrim
session/Minecraft session/world at 328/336/344/352/360. Remaining bytes reserved.
pt_mc_frame8 adds u64 sequence, u32 action, u64 context after activation sequence.
pt_mc_menu writes 32 bytes: active u32 at 0, context u64 at 8, world u64 at 16;
returns 1 on read, 0 on transient lock contention, -1 on failure.

SKSE query interface 4, version 2, AddUITask uses a two-entry Run/Dispose vtable
without a destructor slot. Worker queues at most one task; UI callback alone
reads native objects and dispatches. Native addresses from local CommonLib AE:
PlayerCharacter singleton 403521; MenuControls 401263; BSInputDeviceManager
402776; ButtonEvent vtable 208708; BSFixedString ctor 69161; release8 69192.
MenuControls ProcessEvent is vtable slot 1; processing/remapping flags at 0x80/82.
ButtonEvent is 0x30 bytes: device/type at 8/12, next at 16, owned user-event string
at 24, scan code at 32, value/held duration at 40/44. One synchronous down/up pair
is sent only to MenuControls. Strings are created/released using Skyrim APIs.
UI menuStack at 0x110, count at 0x120; stack pointer identities, save revision and
connection/world identities form the context fingerprint. Request age limit
250 ms; duplicate/context-mismatched requests are discarded. Saved lineage,
Minecraft UUID and Skyrim cell/worldspace must match. UI tasks refresh only the
existing local Skyrim owner's heartbeat, preserving handshake ownership.

Build passed; native runtime dispatch and pause behaviour are **not tested**.

# 0.9.0 interaction supplement (current)

Protocol 19, Local\PassthroughPractice_link19_. Connection remains 288 bytes,
world PTWORLD8/397456 and scene layout unchanged. Position bytes 272-279 are u64
interaction sequence, 280-287 u64 boot-relative issue milliseconds. pt_mc_frame7
appends one u64 sequence after world-ID halves. Native sender stamps the issue time
once per changed world/sequence; repeated render frames do not renew it. Java
publishes a press for <=200 ms, clears on inactive contexts, and consumes key clicks
without repeating while held. Skyrim rejects requests older than 250 ms, duplicates,
resume-barrier requests and calls within a 250 ms cooldown. This is a latest request
slot, not a reliable event queue; rapid presses may be dropped.

CommonLib PlayerCharacter.cpp documents ActivatePickRef AE relocation 40548,
void(PlayerCharacter*). CrosshairPickData singleton AE 401585 points to its flat
runtime structure; target ObjectRefHandle is u32 at +4. Native logs the handle,
then calls regular activation on the player-update thread after lineage/origin
checks and pose updates. No direct TESObjectREFR activation bypass or synthetic
keyboard injection. Dispatch/return are not gameplay success signals.

UI.h counters at 0x160 (pauses), 0x164 (item menus), 0x170 (cursor retention), 0x178
(application menus), and modal byte 0x17C gate bridge control. DialogueMenu.h lists
kDontHideCursorWhenTopmost, which feeds 0x170. Recheck menus/load flags/cell after
activation and release before further collision export. Release invalidates input;
resume requires observing idle sequence=0 before a new request is dispatched.

Minecraft Options.load HEAD mixin appends KeyMapping before vanilla reads saved
settings, preserving normal rebind/save handling without adding Fabric API.
Local javap confirms keyMappings is assigned before the constructor calls load().
Default InputConstants.KEY_R, KeyMapping.Category.MISC; en_us localization supplies
the name. Runtime mixin/gameplay behaviour remains unverified until user testing.

# 0.8.9 persisted-origin supplement (current)

Protocol 18, Local\PassthroughPractice_link18_. Connection record is now 288 bytes,
with unchanged layout version 2 and size header 288. Existing position and render
fields retain their offsets; expected lineage is at 240–255, Minecraft world ID
at 256–271, 272–287 reserved. C pt_mc_frame6 appends two u64 Minecraft UUID halves
after the two lineage halves. Java derives UUID from skyrim-bridge-<UUID> folder;
it transmits most/least-significant UUID longs as little-endian bytes. These are
opaque identity bytes and are distinct from the PBID lineage encoding.

World magic PTWORLD8; size remains 397456. Header u32 at 124 is origin hold reason:
0 none; 1 missing Minecraft UUID; 2 invalid/unavailable origin; 3 different Minecraft
world; 4 different Skyrim cell/worldspace; 5 restored target >140 Skyrim units from
current player; 6 origin-state lock unavailable. Rejected origin replies set
admitted=false and contain no geometry. Java logs SCENE_ORIGIN_STATUS and holds
through the existing absent-coverage path. SCENE_ADMISSION allowed=true refers
to lineage only; SCENE_ALIGNMENT origin_ready confirms origin restoration too.

SKSE serialization plugin PSPT adds PBOR version 1, 72 bytes:
- 0–15: Minecraft world ID bytes.
- 16–19: Skyrim cell form ID (u32 LE).
- 20–23: worldspace form ID (u32 LE; zero for none).
- 24–47: Minecraft origin XYZ (three f64 LE).
- 48–71: Skyrim origin XYZ (three f64 LE).

PBID is unchanged. Validate finite bounded coordinates, nonzero cell/world UUID,
record length/version/uniqueness; resolve cell/worldspace through SKSE ResolveFormId
on load. Unknown record kinds are ignored. Revert clears in-memory origin. A legacy
missing PBOR allows first initialization; malformed or unresolved PBOR blocks
alignment and writes a one-byte rejected PBOR marker on later saves (if SKSE
WriteRecord succeeds), rather than turning into an absent record. Normal saves
write the unchanged origin. No cross-game atomic checkpoint or disk-durability
proof; write failures are logged. One origin per Skyrim lineage state is supported.

On each anchor reattachment, reuse persisted coordinates but bind the new sender/
world session IDs. Restore requires exact Minecraft ID and cell/worldspace equality,
plus target distance <=140 Skyrim units. This is a proximity guard, not rollback
validation. Creation/save/load/restoration log all coordinates. Older sections
below describe historical protocols; do not use their offsets for this build.

# 0.8.8 wire-layout correction

Current protocol 17, namespace Local\PassthroughPractice_link17_. World remains
PTWORLD7/397456 bytes. Expected lineage ID is at connection bytes 240–255, NOT
216–231. The latter overlap render_ms (216–223) and render_session (224–231);
render_ready occupies 232–235. Record size remains 256 and pt_mc_frame5 is unchanged.
The 0.8.7 layout below is historical and broken; use this corrected layout.

# 0.8.7 admission supplement

Protocol 16, Local\PassthroughPractice_link16_, world magic PTWORLD7. World
size remains 397456 bytes. Position reserved bytes 216–231 carry the stored
Minecraft pairing ID. New C symbol pt_mc_frame5 appends two u64 ID halves in
little-endian byte order after the active flag. World u32 at 120 is admitted
(0 or 1). Identity-only replies are transport-active with admitted=0, radius=4,
no boxes/NPCs, and the current Skyrim lineage tail. They are never collision
coverage. Skyrim compares nonzero lineage equality before touching puppet/look
or enabling scene alignment. Java observes the reply before discarding rejected
geometry. Matching snapshots carry admitted=1; fresh coverage controls movement
as before. Save revision equality and restoration are not part of admission.

# 0.8.6 pairing transport supplement

Protocol 15 uses Local\PassthroughPractice_link15_; world magic PTWORLD6.
World record size increases from 397440 to 397456 bytes. The trailing 16 bytes
at offset 397440 contain Skyrim's raw PBID lineage bytes; zero means unavailable.
Existing header/box/NPC offsets, position layout and scene format remain unchanged.
Export only after successful SKSE WriteRecord or valid load. An SKSE callback
success does not independently prove the full companion save reached disk.
Minecraft displays bytes as uppercase hex, matching SCENE_SAVE_ID. Its world-root
passthrough-pairing.txt is ASCII PTPAIR1 + LF + 32 uppercase hex digits + LF.
CREATE_NEW prevents overwriting. A partial write is treated as invalid on reopening;
no automatic repair. Metadata is cached per integrated-server session, offline only.
Pairing is observational and does not yet participate in native movement admission.

# 0.8.5 SKSE save identity supplement

No wire change: protocol 14. Uses QueryInterface(3), serialization interface >=4,
with the field order documented in local CommonLib SKSE/Impl/Stubs.h. Plugin
unique ID is little-endian PSPT, record type PBID, record version 1, payload 16
bytes from Windows CoCreateGuid (GUID integer fields serialized little-endian).
Callbacks are C ABI, protected from Rust unwinding, and access shared state via
Mutex. Revert clears identity; load accepts exactly one nonzero version-1 record;
save lazily generates and writes identity through SKSE WriteRecord. Unsupported,
malformed or duplicate identity records block this plugin's record writing for
that loaded state; this does not abort Skyrim's overall save or preserve bad
record bytes. Missing interface logs interface_unavailable without unloading
already registered callbacks. No player origin or game assets are serialized.
The ID represents a save lineage, not a save revision or globally paired session.

# 0.8.3 location identity supplement

Current protocol: 14, namespace `Local\PassthroughPractice_link14_`.
World magic is PTWORLD5; world buffer size remains 397440 bytes. Header offsets
112 and 116 now hold little-endian u32 cell and worldspace form IDs respectively.
All other world fields and the position/scene record layouts are unchanged.
Zero worldspace represents no associated worldspace, commonly an interior.
CommonLib TESForm formID is at +0x14; TESObjectCELL worldSpace is at +0x128
on the supported AE 1.7.104 runtime (versioned layout >=1.6.629). Read these
on Skyrim's existing player-update thread. Minecraft logs changes with the
snapshot epoch and sequence. These load-order-dependent IDs are diagnostic
location identifiers, not persistent save keys. Both adapters must be upgraded.
Historical protocol descriptions below describe their corresponding versions.

# Native interface notes

### 0.8.2 predictive scan placement

No wire change (protocol 13/PTWORLD4). Exporter estimates blocks/second from
distinct rendered frames and their sent_ms, accepting intervals 1..250 ms with
matching sender/world and increasing frame. Reject nonfinite or teleport-like
velocity (horizontal >32, vertical magnitude >40). Reset velocity history on the
existing exporter reset. Scan-start horizon is prior completed scan milliseconds
plus 150, clamped to 200..500 ms; horizontal lead length <=1.5 blocks, vertical
lead magnitude <=.75. Quantize predicted centre downward to half-block coordinates
and add the existing +1 vertical body offset. Complete that fixed volume before
starting another. Radius, ray grid, budgets, lifetime and scan discard rules stay
unchanged. No stale-volume merging or artificial terrain is introduced.

SCENE_WORLD_SCAN reports centre, chosen lead/horizon, final velocity, elapsed time,
ray count, retained boxes and active/discard status. SCENE_BRIDGE_MOTION separates
freshness/session failures from outside_bounds, logs requested AABB and snapshot
coverage plus hold_ms on changes and once/second while waiting. These durations
measure the hold-check state transitions, not end-to-end rendered latency. The
existing movement/fall-distance hold rules are retained. Gameplay improvement is
not tested; no performance gain is inferred from build success.

### Current 0.8.1 save-resume increment

Native protocol stays 13 (world PTWORLD4, scene PTSCE012). Resume bridge uses
LevelStorageSource.findLevelCandidates/loadLevelSummaries, selects the newest
marked name by lastPlayed, then shows a confirmation with the save name/folder.
WorldOpenFlows.openWorld retains vanilla validation and loading. Async callbacks
are posted to Minecraft's executor and ignored if the user left the parent screen.
New-world confirmation does not convert/copy inventory. This is a Minecraft-save
shortcut, not automatic Skyrim-save pairing.

PlayerList.placeNewPlayer RETURN and PlayerList.save RETURN log server-side bridge
inventory summaries. SCENE_BRIDGE_INVENTORY includes phase, normalized save folder,
player UUID, inventory slot item IDs/counts, six player equipment slots and XP.
Equipment may overlap inventory slots and is not added to inventory_count. No
item-component hash, ender-chest dump or on-disk integrity claim is made. The
save_return event means the vanilla save method returned, not that a separate
disk-read verification succeeded. No independent inventory file or save writer is
introduced. Runtime hook application and persistence round trip are not tested.

The game adapter is written in Rust and supports Windows x64 MSVC and Skyrim
1.7.104.0 only. It uses a small subset of the public SKSE and CommonLib interface
definitions. CommonLib is a source reference, not a linked runtime dependency.
The reference checkout is ignored under `.tools/CommonLibSSE-NG`.

## Sources used

- [SKSE plugin interface](https://github.com/ianpatt/skse64/blob/master/skse64/PluginAPI.h):
  loader structures, messaging interface 5, messages, and plugin metadata.
- [SKSE load result dispatch](https://github.com/ianpatt/skse64/blob/master/skse64/Hooks_SaveLoad.cpp):
  post-load success is encoded as a non-null pointer value, not a bool to dereference.
- CommonLib reference commit `94faaed0c60eddd8347767f2d4d29a97c93bde8c`:
  [player vtable ID](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/Offsets_VTABLE.h),
  [actor interface](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/A/Actor.h),
  [reference position](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/T/TESObjectREFR.h),
  [controller](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/B/bhkCharacterController.h),
  [address format](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/REL/IDDB.h).
- SkyCraft's local `skse/src/Game.cpp` uses the player update virtual slot `0xAD`
  and `Actor::SetPosition(position, true)`. Its Minecraft render callback uses
  `getGameTimeDeltaPartialTick(false)` and `player.getPosition(partial)`.

## Skyrim boundary

| Item | Value for this runtime |
| --- | --- |
| Packed runtime | `0x01070680` |
| Plugin metadata size | `0x350` bytes |
| Player vtable Address Library ID | `208040` |
| UI singleton pointer Address Library ID | `400327` |
| Player update virtual slot | `0xAD`; call original before applying position |
| Actor SetPosition virtual slot | `0xA9`; pass `true` to update its controller |
| Reference position | `0x54`, three floats |
| Actor process pointer | `0xF8` for post-1.6.629 layout |
| Process middle-high pointer | `0x08` |
| Middle-high character controller pointer | `0x250` |
| Controller set-velocity virtual slot | `7`, pointer to an aligned four-float vector |
| Controller fall-start height / fall time / gravity | `0x240` / `0x244` / `0x248` |
| UI pause counter | `0x160` |

The adapter reads the matching format-5 Address Library at runtime. It checks
format, version, pointer size, file length and image-relative address bounds.
It changes no saved control-enable flags. While following Minecraft, it clears
controller velocity and fall timers and saves/suspends controller gravity. It
restores gravity on release when that same controller still exists. A newly
created controller gets its own saved gravity value.

Earlier tests covered metadata layout, synthetic address-library validation, and
simulated actor/controller objects. Subsequent live logs confirmed loading and
position read-back; the user confirmed movement, wall/NPC blocking, jumping and
landing. The user subsequently confirmed the step 4 look adapter turns Skyrim's
camera when Minecraft's camera turns.

## Position transport

Protocol 13 uses the `Local\PassthroughPractice_link13_` namespace and retains the
256-byte position record and its layout-2 header. Integers and
floating-point bit patterns are stored little-endian, protected by the existing
named mutex. Bytes 0–127 retain the connection fields; bytes 128 onward hold:

| Offset | Field |
| --- | --- |
| 128 / 136 / 144 / 152 | sender generation / world generation / frame / boot-relative send time, each u64 |
| 160 | active u32; 164–167 reserved |
| 168 | partial-tick fraction, f64 |
| 176 / 184 / 192 | interpolated Minecraft X / Y / Z, each f64 |
| 200 / 208 | rendered player yaw / pitch in degrees, f64 |
| 216 / 224 | renderer heartbeat boot time / Skyrim generation, each u64 |
| 232 | renderer ready, u32; 236–255 reserved |

Only Minecraft can publish positions. Skyrim requires a mutual handshake,
matching protocol and source generation, finite coordinates, and a sample no
older than 250 ms. Inactive samples clear movement authority immediately on
the next Skyrim update. Frame IDs must increase within each sender generation.

Fabric calls `pt_mc_init()` and then `pt_mc_frame4(world, frame, partial, x, y, z,
yaw, pitch, active)` on the render thread using Java 25's foreign-function API. No position
is calculated in a 20-Hz client-tick callback. Rendering rates can differ, so the
receiver consumes the newest frame instead of replaying a backlog. Tests use
their own named sessions and keep their logs in `logs/step2-fixtures`.

## Step 3 world export

The same pinned CommonLib checkout supplies these additional definitions:
`TESObjectCELL.cpp`, `hkpWorld.h`, `bhkWorld.h`, `BSAtomic.cpp`,
`ProcessLists.h/.cpp`, `Misc.cpp`, `BSHandleRefObject.cpp`, `NiRefObject.h`,
`hkpWorldRayCastInput.h`, `hkpWorldRayCastOutput.h`, and `hkpCollidable.h`.
No Skyrim binaries or assets are copied into source control.

| Interface | AE Address Library ID or offset |
| --- | --- |
| TESObjectCELL::GetbhkWorld | 18995 |
| hkpWorld::CastRay | 61399 |
| Havok world scale float | 188105 |
| ProcessLists singleton | 400315 |
| LookupReferenceByHandle | 12332 |
| BSReadWriteLock read lock/unlock | 68233 / 68239 |
| Reference parent cell / form ID | 0x60 / 0x14 |
| bhkWorld::GetWorld1 virtual slot | 0x27 |
| bhkWorld worldLock | 0xC598 |
| ProcessLists highActorHandles data / count | 0x30 / 0x40 |
| TESObjectREFR BSHandleRefObject base / reference count | 0x20 / base+8 |
| NiRefObject::DeleteThis virtual slot | 1 on the adjusted base pointer |
| Ray input size / alignment | 0x30 / 16 |
| Ray output size / alignment | 0x60 / 16 |
| Ray fraction / root collidable | 0x10 / 0x50 |
| Collidable filter info | 0x2C, layer mask 0x7F |

The exporter runs on the existing Skyrim player-update thread, holds the Havok
read lock during a sampling batch, and releases each acquired actor reference.
It keeps numeric copied data between frames, not Havok object pointers. NPCs
come from the same cell as the player. Shapes come from the cell's Havok world.
Layer 30 is used for ray filtering, and returned layers 1, 2, 3, 9, 10, 13, 17,
27, and 31 are included. Up to eight closest-hit casts continue through each ray.
The sampled boxes are an approximation; no Havok shape decoder is implemented.

The separate `_world_memory` mapping has 397,440 bytes, under its own mutex:

| Offset | Contents |
| --- | --- |
| 0 | Eight bytes `PTWORLD4` |
| 8 / 16 / 24 | Skyrim generation / recipient Minecraft generation / Minecraft world, u64 |
| 32 / 40 / 48 | origin epoch / snapshot sequence / Windows boot-relative send milliseconds, u64 |
| 56 / 60 / 64 | active / box count / NPC count, u32 |
| 72 | Java copy only: remaining snapshot lifetime in milliseconds, u64 |
| 80 / 88 / 96 | Scan centre XYZ in Minecraft coordinates, f64 |
| 104 | Scan radius, f64; active snapshots require 4.0 |
| 128 | 8,192 slots of six f64 values: min XYZ, max XYZ (48 bytes each) |
| 393344 | 128 NPC slots: form ID u64, position XYZ f64 (32 bytes each) |

Only Skyrim publishes snapshots. Minecraft checks the connection handshake,
generations, active state, world and age before copying through
`pt_mc_world(pointer, capacity)`. Both receivers reject oversized counts or
invalid coordinates. Java constructs a spatially indexed immutable copy and
publishes it to the integrated server through a volatile reference. It restricts
queries to the current client level or its own integrated server and dimension.
Snapshots expire rather than retaining geometry indefinitely after a failure.

The common `BlockCollisions.computeNext` mixin supplies extra elements before
letting the vanilla iterator continue. It uses the existing result provider and
world-space shapes, skips suffocation-only queries, and does not change movement
resolution. Minecraft 26.3 method signatures and iterator behavior were inspected
from the ignored build cache. Source compilation passed; the new native ABI,
mixin runtime application was subsequently observed in live logs. The user has
confirmed basic wall/NPC blocking, jumping and landing; broad coverage remains
**not tested**.

## Step 4 look direction

Minecraft uses `player.getViewYRot(partial)` and `getViewXRot(partial)` with the
same partial tick used for position. Yaw is normalized to [-180,180) degrees;
pitch is bounded to [-90,90]. The decoder rejects nonfinite/out-of-range values.
The new native export name `pt_mc_frame4` prevents the older Java argument list
from silently calling the changed native signature. Protocol 4 isolates old
adapters with a new shared-memory namespace; the world-snapshot format stays 3.

Skyrim follows SkyCraft's `McYawToHeading` convention: heading equals yaw minus
180 degrees, normalized to [0,360), then converted to radians. Pitch retains its
sign and is converted to radians. The look adapter writes only player
`data.angle.x` (0x48) and `data.angle.z` (0x50), after the normal player update.
Skyrim's next normal first-person camera update consumes the new angles. This
does not override camera matrices or duplicate the camera update.

Additional published CommonLib interfaces, from the existing pinned checkout:

| Item | ID/offset |
| --- | --- |
| PlayerCamera singleton | AE ID 400802 (`PlayerCamera.cpp`) |
| ForceFirstPerson / ForceThirdPerson | AE IDs 50790 / 50796 |
| TESCamera currentState | 0x28 (`TESCamera.h`) |
| TESCameraState ID | 0x18 (`TESCameraState.h`) |
| First-person / ordinary third-person state IDs | 0 / 9 |
| TESCameraState GetRotation virtual slot | 4, writes NiQuaternion (w,x,y,z) |

`LOOK_RECV` logs the source angles and converted target/read-back angles, plus
camera-state quaternion. The quaternion is explicitly sampled before the next
camera update and is not claimed to be this frame's final rendered view.
The comparison tool's `-RequireLook` checks the source values bit-for-bit and
the conversion/read-back within 0.00001 radians. It does not prove rendered
camera alignment. Automated angle checks have not been rerun; camera following
was subsequently confirmed by the user.

## Step 5a image transport and composition

The Minecraft render mixin checks readiness before `GameRenderer.render` and
captures after it, following SkyCraft's `FrameExporter` / `LevelRendererMixin`.
The level-render mixin clears color to transparent and reverse-Z depth to zero,
then cancels level drawing only while linked and locally active. Hand/HUD
rendering continues. Source format must be `GpuFormat.RGBA8_UNORM`. Three
asynchronous GPU buffers use MAP_READ | COPY_DST (9); callbacks mark completed
copies, and only the newest completed image is mapped/published. Old completions
never replace a later published frame. Capture is capped at 30 Hz, 2560 x 1440.

New FFI exports: `pt_mc_overlay_ready() -> u64` returns boot-relative time when
Skyrim's renderer is ready, otherwise zero. `pt_mc_overlay(world:u64, frame:u64,
captured:u64, width:u32, height:u32, rgba:*const u8, bytes:u32) -> i32` copies the
buffer before returning (1 published, 0 skipped, -1 failure). Position's
`pt_mc_frame4` signature stays unchanged. Version 0.5.0 requires all three new
artifacts; protocol 5 isolates older installations.

The image mailbox uses the connection namespace with session suffix `_overlay`,
a separate mutex, and 80 + 2560*1440*4 bytes. Little-endian fields:

| Offset | Field |
| --- | --- |
| 0 | `PTIMAGE5` magic (8 bytes) |
| 8 / 16 / 24 / 32 | Minecraft generation / world / capture sequence / captured boot time, u64 |
| 40 / 44 / 48 / 52 | width / height / byte count / flags=1 (vertical flip), u32 |
| 56 / 64 | FNV-1a 64-bit pixel checksum / nonzero-alpha pixel count, u64 |
| 72–79 | reserved |
| 80 | tightly packed RGBA8 pixels, width*height*4 bytes |

Skyrim rejects wrong generation/world, invalid dimensions/byte counts, images
older than 500 ms, and checksum/count mismatch. Drawing additionally requires
a currently accepted movement sample no older than 250 ms; release clears that
permission. Renderer readiness includes Skyrim generation and expires in 500 ms.
No image is read or drawn while the player adapter has released control.

Published CommonLib `Renderer::GetCurrentRenderWindow` uses AE ID **411349**,
a global `RendererWindow*`; its `swapChain` member is at **0x18**. On DataLoaded,
the plugin hooks that chain's `IDXGISwapChain::Present` vtable slot **8**. Other
chains and `DXGI_PRESENT_TEST` pass straight through. Initialization, upload and
drawing all run on the Present thread. Overlay failures leave the original
Present call and the position/collision adapters running.

An RGBA8 D3D11 texture is uploaded for each new image. The fullscreen triangle
flips Y as SkyCraft does and uses premultiplied blending (ONE / INV_SRC_ALPHA),
with depth disabled. A separate D3D11 context state holds the overlay's graphics
settings. A scope guard restores Skyrim's state even on errors, using Microsoft's
[CreateDeviceContextState](https://learn.microsoft.com/en-us/windows/win32/api/d3d11_1/nf-d3d11_1-id3d11device1-createdevicecontextstate)
and [SwapDeviceContextState](https://learn.microsoft.com/en-us/windows/win32/api/d3d11_1/nf-d3d11_1-id3d11devicecontext1-swapdevicecontextstate).
The original device's single-threaded flag is propagated. Back-buffer views are
created per draw and unbound before restoring state; none are retained across
Present/ResizeBuffers. Windows crate 0.62.2 provides the public COM interfaces.
HLSL shaders compile on first live renderer initialization using D3DCompile.

OVERLAY_SEND/RECV record identity, dimensions, checksum and alpha count; DRAW
records submission. CPU timings are diagnostics, not GPU timestamps. The user
confirmed first-person arms/punching, held items, HUD and inventory appear inside
Skyrim's window. Numerical transport comparison, detailed transparency/orientation,
color matching and state-restoration checks are **not tested**. World/depth
composition is 5b.

## Step 5b: nearby block meshes (0.6.0, not tested)

SkyCraft's current `WorldExporter.java` and `WorldRender.cpp` use block geometry
and textures rendered through Skyrim's camera, rather than reprojecting a flat
Minecraft world screenshot. This implementation follows that division with
bounded complete snapshots instead of its section-message queue. See
[SkyCraft's world renderer](https://github.com/chasmlol/SkyCraft/blob/master/skse/src/WorldRender.cpp)
and [world exporter](https://github.com/chasmlol/SkyCraft/blob/master/fabric/src/client/java/dev/skycraft/client/render/WorldExporter.java).

Minecraft calls `ModelBlockRenderer.tesselateBlock` on a 16-cubed region centered
on the player's integer block position, at most every 100 ms. Work is spread
over frames with a 3 ms tessellation budget. A snapshot taking over one second
is discarded. Opaque/cutout `BakedQuad` objects become triangles (indices
0,1,2,0,2,3). Vertices contain relative XYZ, atlas UV and RGBA tint/shading with
simplified sky/block light brightness. Translucent faces are counted and skipped.
It scans loaded chunks only and does not change blocks, collision or saves.

`AtlasAccess` reads TextureAtlas width/height/texturesByName; `SpriteAccess`
reads SpriteContents.originalImage. These fields and the tessellation signatures
were inspected in the cached Minecraft 26.3 JAR. Atlas pixels are copied from
the first sprite frame at runtime, uploaded only on world/atlas identity changes,
and never saved. Resource reload creates a new atlas identity. Atlas and mesh
IDs must agree before Skyrim draws. Bounds/finite values are checked natively.

`pt_mc_scene(pointer, bytes:u32) -> i32` returns 1 on publish, 0 on contention or
no native link, -1 on error. Java owns the input until the synchronous copy
returns. Current packages pair the 0.8.2 DLL with its JAR. Protocol 13 isolates old
shared-memory namespaces; the earlier overlay and collision payloads are retained.

Two mutex-protected mailboxes use session suffixes `_mesh` and `_atlas`.
Header size is 128; capacity is 128 + 262140*24 and 128 + 4096*4096*4 respectively.
All fields are little-endian:

| Offset | Field |
| --- | --- |
| 0 | `PTSCE012` magic |
| 8 / 16 / 24 / 32 | sender generation / world / snapshot / publish boot time, u64 |
| 40 | kind u32: 1 mesh, 2 atlas |
| 44 | u32 vertex count (mesh) or width (atlas) |
| 48 | u32 payload byte count |
| 52 | u32 height (atlas), zero for mesh |
| 56 | FNV-1a 64-bit payload checksum |
| 64 | atlas generation, u64 |
| 72 / 80 / 88 | mesh origin XYZ in Minecraft coordinates, f64 |
| 96 | u32 crack vertex count, zero for atlas; trailing subset of mesh vertices |
| 100 | u32 opaque/cutout item vertex count, zero for atlas; subset of solid prefix |
| 104 | u32 item entity count, zero for atlas |
| 108 | u32 translucent vertex count, zero for atlas |
| 112 | u32 3D dropped-model count, subset of item entity count |
| 116 | u32 translucent item vertex count, subset of offset 108 |
| 120 | opaque fluid vertices, u32; subset of static opaque vertices |
| 124 | translucent fluid vertices, u32; subset of offset 108 excluding item vertices |
| 128 | mesh: 24 bytes/vertex (five f32 XYZUV, four u8 RGBA); atlas: RGBA8 |

The receiver reads new generations/sequences only, validates shape/checksum and
logs SCENE_RECV. Meshes expire after two seconds; atlas data is retained until
replaced, but never drawn for the wrong world/generation. Fresh movement
permission and its matching Anchor are copied from the player-update thread.
That Anchor converts the mesh origin into Skyrim coordinates. Vertex offsets
use X, -Z, Y times 70 units. World drawing runs in the existing isolated Present
context before hands/HUD, and its failure disables only this new pass.

Published CommonLib interfaces (same pinned source commit as earlier steps):

| Item | ID/offset |
| --- | --- |
| BSGraphics::Renderer singleton (object, not pointer slot) | AE ID 411393 |
| Main::WorldRootCamera function | AE ID 36609 |
| Renderer depthStencil array, runtime >=1.6.1130 | 0x2018; MAIN index 0 |
| DepthStencilData.depthSRV | 0x88 |
| NiCamera world transform rotation / translation | 0x7C / 0xA0 |
| NiCamera worldToCam projection matrix | 0x110, 4x4 floats |
| NiCamera frustum near / far / orthographic flag | 0x160 / 0x164 / 0x168 |
| NiCamera viewport left/right/top/bottom | 0x174, four floats |

Use the actual Skyrim matrix, rebasing its translation around the camera with
double-precision arithmetic. A point 500 units ahead must project near screen
center with positive W, otherwise skip and log. Only a full [0,1,1,0] viewport
and perspective camera are supported. Log the projection check, near/far and
mapped mesh origin in SCENE_DRAW. These are diagnostic consistency checks,
not independently verified game offsets or rendered alignment.

A 4x4 screen grid plus one centre sample is read from Skyrim's main depth SRV into a 17x1 R32_FLOAT
texture. Copy it to staging and poll with `Map(DO_NOT_WAIT)` on later frames;
no intentional GPU wait. Every 500 ms, log minimum/median/maximum, rejecting
nonfinite/out-of-range/uniform samples. As in SkyCraft's fallback, median below
0.5 infers reversed depth. This is a heuristic, not a proof for arbitrary scenes
or graphics mods. Invalid/unknown depth suppresses world drawing. The pixel
shader converts sampled depth into view distance using Skyrim's near/far,
discards Minecraft pixels behind it (one Skyrim unit tolerance), and alpha-tests
cutout pixels. A private D32 buffer resolves Minecraft-against-Minecraft depth,
using the convention measured from Skyrim's projection matrix. Skyrim's depth
buffer itself is read only; no reference to it is retained across Present.

In 0.6.2 the atlas includes a 16-pixel-high strip for ten damage textures,
loaded through Minecraft's resource manager and resampled to 16x16 in memory.
Block V coordinates are scaled to the enlarged atlas height. Total dimensions
must remain at most 4096x4096; block atlas width must be at least 160. A block
atlas already 4096 high cannot fit this strip and is rejected with SCENE_DISABLED.
Minecraft's public ClientLevel.destructionProgress() supplies the highest damage
stage per position. Only MODEL blocks with solid rendering and full-cube collision
shapes are included, at most 32 per snapshot, within the existing export region.
Each emits 36 vertices on a cube inflated by 0.002 blocks; unsupported shapes or
capacity limits are counted as skipped. The crack count must be a multiple of 36,
no greater than 1152 or the total vertex count. Atlas packets must have zero.
Solid vertices precede cracks. Skyrim draws solids normally, then cracks with
depth testing but no depth writes. The pixel shader converts grayscale darkness
times texture alpha to the opacity of black; SRC_ALPHA/INV_SRC_ALPHA blending
darkens existing block color. Colored resource-pack crack textures become
grayscale. The layer still undergoes the Skyrim depth comparison. All state is
within the existing isolated context; the hands/HUD pass follows. Counts are
logged in SCENE_SEND/RECV and SCENE_CRACK_DRAW; client stages in SCENE_CRACK.
No new FFI entry point is needed. This rendering path is not tested in game.

0.6.3 supersedes the atlas layout above: block and item atlases are packed
vertically, or horizontally if the vertical layout exceeds the height limit;
the crack strip follows both. Combined dimensions remain at most 4096x4096.
Both U and V of block meshes are rescaled; sprite identity maps supply item UVs.
A change of either source sprite map rebuilds the combined runtime-only atlas.
The exporter visits at most 4096 loaded entities, attempts up to 64 nearby items,
and checks a 3 ms deadline between entities. One expensive model resolution can
exceed that budget; remaining items are omitted rather than delaying publication.
SCENE_ITEMS records skipped/errors/scanned/budget_limited counts. Each ItemEntity
uses getPosition(partial), getAge, bobOffs and getSpin; ItemModelResolver with
GROUND context supplies a particle sprite. The mesh contains a half-block-high
spinning plane, six vertices; culling is disabled so both sides draw. Color is
white (no Minecraft lighting/tint/glint), alpha cutout uses the existing shader,
and icons participate in the solid depth-writing pass before cracks. A stack
uses one icon irrespective of its count. Item vertices must be divisible by six,
at most 384, and items plus cracks must not exceed total vertices. All are zero
for atlas packets. Geometry order is blocks, items, cracks. No native entity
simulation or pickup logic is added; removal is reflected by the next complete
snapshot. Existing stale/world permission checks still apply. Dynamic positions
are sampled at publication time but updates remain roughly 10 Hz; this first
version does not interpolate between received mesh snapshots. Full 3D block/item
models, animated textures and custom material atlases remain unsupported.
SCENE_ITEM logs ID/type/stack/position/bob/spin; SCENE_SEND/RECV/ITEM_DRAW log
item_vertices for correlation. This increment is not tested in game.

0.6.4 extends the item subset to full opaque block models: a default BlockState
must have MODEL rendering, solid rendering and full-cube collision. The model
must provide exactly one quad per direction, no unculled quads, bounded finite
positions and known, nontranslucent atlas sprites. Otherwise the item follows
the previous icon path. Each supported drop has 36 vertices transformed to 0.25
block scale, centred on its interpolated X/Z with bob plus 0.02 blocks below its
bottom face. Each quad's original UVs are mapped into its copied sprite rectangle;
default block tint is applied per tinted quad. Item-specific block-state/model
components are not applied. A thrown model error rolls back the item's vertices
and logs SCENE_ITEM_ERROR; other items continue.

The item entity count is now explicit at offset 104. Validation requires N<=64,
6*N<=item_vertices<=36*N and (item_vertices-6*N) divisible by 30. Thus each item
is either a six-vertex icon or a 36-vertex model, at most 2304 item vertices.
Both counts are zero for atlases; item plus crack vertices must fit the mesh.
SCENE_ITEM_DRAW derives models=(item_vertices-6*N)/30 and icons=N-models;
SCENE_SEND/RECV include both item_vertices and items. Protocol 9 separates older
readers that assume six vertices per item. Existing atlas, depth and budget rules
are retained. This new 3D path is not tested in game.

0.6.5 collects translucent MODEL quads separately instead of skipping them.
One bounded direct buffer holds their 24-byte vertices, with a centroid per quad.
At publication, sort a copy of the quad list by descending squared distance to
the current interpolated Minecraft eye relative to the mesh origin; copy each
six-vertex quad intact into the outgoing mesh. Log quads/vertices, relative eye
and sort/copy microseconds. Geometry order is opaque/cutout blocks, items,
translucent faces, cracks. Reserve translucent capacity before adding items;
the existing total limit remains 262140 vertices. Offset 108 is divisible by
six and bounded by that limit; cracks+items+translucent must fit the total.
All non-applicable counts are zero for atlas packets. Protocol 10 separates old
readers that would draw these faces as opaque.

Skyrim draws the opaque/item prefix, then cracks at their trailing offset, then
the sorted translucent slice. The shader uses depthParams.w=2 to return sampled
RGBA without the opaque alpha cutoff. It reuses SRC_ALPHA/INV_SRC_ALPHA blending
and the no-depth-write state from cracks; Skyrim depth rejection and private
opaque depth testing stay enabled. The output alpha is preserved by the existing
blend state. Cracks draw before glass so foreground glass also blends over cracks.
SCENE_SEND/RECV and SCENE_TRANSLUCENT_DRAW include translucent_vertices. The
pipeline retains disabled face culling; this can affect the opacity of closed
translucent models. Sorting uses Minecraft's eye and publication rate, not exact
per-pixel ordering or Skyrim's per-frame camera. No fluids, refraction or colored
lighting are added. Sorting has a vertex bound and timing log, not a separate
time budget. Runtime shader behavior and gameplay are not tested for this build.

### 0.6.6 pre-HUD world stage

The late world pass caused geometry to cover the compass, as reported by the
user after confirming 0.6.5 transparency. Resolve HUDMenu's primary vtable with
AE Address Library ID 215362 (CommonLib Offsets_VTABLE.h); replace virtual slot
6, inherited IMenu::PostDisplay. CommonLib IMenu.cpp calls uiMovie->Display in
that method. The hook signature is void(IMenu*), the Windows x64 C ABI. Store
and chain the original slot, including an existing hook; restore page protection
after patching, logging any restoration failure. Install on SKSE data-loaded,
alongside the existing Present hook. No new protocol fields are needed.

On the same thread as Present, use the initialized overlay renderer to acquire
the current swap-chain back buffer and a temporary RTV, swap into its isolated
D3D11 context state, draw the existing world pass, and restore Skyrim's state
before invoking the original HUD display. Present is now hands/HUD-only. The
first HUD invocation may precede renderer initialization and therefore skip one
frame. No back-buffer reference is kept between calls. Pre-HUD errors disable
only that stage; Present overlay and movement remain independently active.
SCENE_STAGE_HOOK records installation; SCENE_STAGE records stage invocation,
not proof that the inner renderer emitted pixels. SCENE_STAGE_WAIT at Present
reports no observed pre-HUD pass on that thread; SCENE_DRAW still identifies
actual mesh submissions. These events are included by collect-scene-logs.ps1.

The back buffer and render-thread timing at HUDMenu::PostDisplay have not been
verified in game for runtime 1.7.104. Missing hooks, suppressed HUD display,
different-thread invocation or unsupported multisampling may suppress world
geometry. There is intentionally no world fallback at Present. Other menus/UI
layers and third-party HUD hooks are not guaranteed by this narrow compass fix.

### 0.6.7 transparent and non-full dropped block models

Collect all directional and unculled quads from the default block MODEL state,
up to 96 quads per item. Validate sprite availability and finite vertices within
the unit block; other models fall back to icons. Scale/rotate/tint as before.
Translucent faces enter the same centroid-sorted collection as placed faces;
opaque/cutout faces enter the item prefix. Per-item failures roll back both
buffers. Publication retries truncate the translucent list to its static count
before regenerating drops. Combined geometry remains bounded by the original
total limit, including reserved translucent space.

Offset 100 now counts only opaque/cutout item vertices. Offset 116 counts dropped
translucent vertices already included in offset 108: do not add them twice to the
mesh total. Their sum is the total dropped-item vertex count. Offset 112 replaces
the old inferred model count. Validation requires 0<=models<=items<=64, counts
divisible by six, at least six vertices per item, exactly six per icon and at
most 576 per model in aggregate. Opaque item vertices must accommodate every
icon, and translucent item vertices must fit the translucent subset. All such
counts are zero for atlas packets. Receive/draw logs carry both item vertex
counts and explicit models/items for comparison. Protocol 11 isolates old
six/36-vertex readers. No shader, HUD-hook or item-gameplay change is required.
Default-state geometry may differ from Minecraft's dedicated item display;
special block-entity renderers, item components and exact alpha ordering remain
unsupported. Gameplay is not tested for this increment.

Microsoft references: [depth as a texture and separate depth targets](https://learn.microsoft.com/en-us/windows/win32/direct3d11/d3d10-graphics-programming-guide-depth-stencil)
and [nonblocking Map](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11devicecontext-map).

Rust/Fabric source builds pass. HLSL compiles on first in-game use; this new
shader compilation, mesh/atlas transport, actual camera alignment and occlusion,
reload/resizing/lifecycle behavior and performance are **not tested**. The pass
does not participate in Skyrim's earlier HDR/post-processing or shadow passes.
Fluids, translucent blocks, dropped items, block entities and animated entities
are not implemented in this first world-mesh build.

### 0.6.8 fluid surfaces

FluidCapture implements FluidRenderer.Output and its complete-vertex callback.
Minecraft's renderer supplies flow heights, faces, tint, light, layer and atlas
UVs. Its section-local positions are rebased by `(blockCoordinate & ~15) - origin`
on each axis, including negative coordinates. Four vertices become two triangles;
block-atlas UVs are remapped into the combined runtime atlas. Waterlogged cells
export both their fluid and their ordinary block model.

Opaque fluid faces enter the static prefix; translucent faces join the existing
quad-centre sorting collection. The total vertex limit and scan deadline remain.
Offsets 120/124 count fluid vertices already included in the mesh, never extra
vertices. Validation requires multiples of six, zero counts on atlas packets,
opaque fluids within the non-item solid prefix and translucent fluids within the
non-item translucent slice. Protocol 12 and PTSCE012 isolate older adapters.
SCENE_FLUID logs cells/water/lava and vertex counts; SCENE_SEND/RECV and
SCENE_FLUID_DRAW carry the same fluid counts for snapshot comparison.

No new native shader or render stage is needed. Static texture frames, approximate
transparency sorting and roughly 10 Hz geometry updates remain. No underwater
effects, Skyrim-fluid import, or fluid reshaping against Skyrim geometry is added.
Rust and Fabric builds succeeded; runtime fluid behavior is **not tested**.

### 0.7.0 terrain placement

No native wire changes: retain protocol 12. SkyCollision tags the existing terrain
boxes separately from synthesized NPC body boxes. Terrain ray queries use the
same fresh, immutable, dimension/session-scoped snapshot as movement. The client
Minecraft.pick tail compares terrain against vanilla's completed block/entity hit
only while holding a BlockItem. Map the actual surface to a whole outward grid
cell. The packet click point is the ray's entry into that cell, keeping it inside
vanilla's packet bounds check; log the actual surface separately.

BlockItem.useOn validates air targets on both sides of an unpublished integrated
server. Trace from the current eye through the click point to normal reach and
require the same cell, face and nearby cell-entry point. Reject intervening vanilla
geometry and terrain occupying the destination cell. Then vanilla owns placement,
item consumption, entity obstruction and support rules. The server uses its latest
fresh snapshot, which can differ from the rendered one and reject changed targets.
No saved support blocks, bucket override or fluid simulation change is introduced.
SCENE_TARGET, SCENE_PLACE and SCENE_PLACE_RESULT are collected by the existing
scene log script. Runtime mixin application and gameplay are **not tested**.

### 0.7.1 grid targeting correction

Intersect the original aiming ray with the outward snapped grid plane, then take
its tangential cell coordinates from that intersection. The earlier choice kept
the terrain contact's tangential cell, which a shallow ray could miss after normal
axis rounding. Keep the actual terrain contact for reach/occlusion and keep packet
points on the ray within the chosen cell. Both client and server use this same
mapping. Normal server validation, protocol 12 and collision sampling are unchanged.
SCENE_TARGET adds decision/eye/reach/surface_distance; throttled SCENE_TARGET_MISS
records missing targets and grid-intersection failures. Collision coverage remains
four blocks per axis. These logs do not prove a particular vanilla placement
failure's cause. Gameplay of the correction is **not tested**.

### 0.7.2 contact-preserving offline intent

Restore tangential destination coordinates to floor(actual contact); snap only
the normal axis outward. Clamp a separate packet point to [cell+.001,cell+.999].
The packet point is not a ray origin/direction source. Client pick retains a
500 ms candidate with client level/server/player and terrain sender/epoch. At
client BlockItem.useOn, match the synthetic hit, independently revalidate, and
register an immutable intent before vanilla's predicted action returns its packet.
The inspected 26.3 startPrediction path sends that packet after performUseItemOn.

Use a synchronized deque, max 16 entries, each expiring in 500 ms. Match server,
level/dimension, player UUID, hand, item identity, cell, face and packet position
(with squared tolerance 1e-8 for serialization). Consume once on the server before
validation. Clear on bridge reset; prune expiry/server changes on enqueue/consume.
Server traces from its eye toward the saved actual contact and requires fresh
sender/epoch, cell/face agreement and contact within .1 block, normal reach plus
.1 tolerance, no nearer vanilla block, and no terrain inside the cell. Vanilla
still controls inventory, support rules, entity obstruction, prediction and ack.
No second placement operation, custom packet, native protocol change or remote
server support. This mailbox is only valid for an unpublished integrated server.
SCENE_PLACE logs intent identity; SCENE_TARGET uses mapping=contact_cell. Source
build and runtime behavior must be reported separately; gameplay is not tested.

### 0.7.3 filled terrain buckets

TerrainPlacement accepts vanilla WATER_BUCKET and LAVA_BUCKET for synthetic air
targets. BucketItem inherits Item.useOn in 26.3; TerrainBucketMixin intercepts only
those two items, unpublished integrated-server sessions and air cells. Ordinary
Minecraft targets and empty/special buckets keep vanilla paths. Reuse the 0.7.2
intent validation; then call BucketItem.emptyContents at the exact destination,
with null hit to suppress adjacent-cell fallback. Check mayInteract/mayUseItemAt,
invoke extra-content/placed-block criteria and use getEmptySuccessItem plus
ItemUtils.createFilledResult for the returned held stack. ItemStack.useOn awards
item-use statistics and applies component side effects; the normal client/server
item-use path handles stack replacement. No direct setBlock or second use packet.
SCENE_BUCKET records side, decision, cell, item, fluid state and result, alongside
SCENE_PLACE intent diagnostics. Protocol 12 and scene format are unchanged.
This does not add fluid containment, Skyrim water pickup or special bucket content.
Gameplay/prediction/stack behavior is not tested.

### 0.8.0 empty bridge world and local coverage gate

Create a unique save through WorldOpenFlows.createFreshLevel, with LevelSettings
name marker Skyrim Bridge [v1], survival/commands, WorldOptions structures/chest
false and an overworld FlatLevelSource: 128 AIR layers, THE_VOID biome, explicit
empty structure overrides, decoration/lakes false. Spawn height derives from the
layer count (64). Marker plus generator signature gates active passthrough and
world suppression. The first sixteen generated chunks per generator log non-air
counts after buildTerrain. Marked saves disable natural mobs/patrols/phantoms/traders
and set respawn radius zero at server load. Other saves are not converted.

Protocol 13 uses the same world mapping size but PTWORLD4 and header coverage
fields 80..111. Exporter publishes its actual existing scan centre/radius. Rust
checks finite centres/radius and active radius 4. Java retains coverage with the
immutable snapshot and requires freshness (remaining lifetime and 250 ms render
heartbeat), matching level/server/dimension and full containment of the player's
body plus requested movement and .5 block margin. Initial readiness also requires
a terrain collision tile within one block below the feet. Readiness is per player
object and retained for later jumps; missing coverage still holds movement.

Entity.move is cancelled for held bridge players on both sides; velocity and fall
distance are reset. ServerGamePacketListenerImpl.handleMovePlayer additionally
rejects held moves only on the server thread, allowing vanilla's network-thread
rescheduling first. Active stationary position samples continue so Skyrim can
establish alignment/export terrain while movement is held. Overlay gating leaves
normal worlds rendered normally. SCENE_BRIDGE_WORLD/CHUNK/COVERAGE/MOTION logs are
captured by the existing scene collector. No geometry, assets or world saves are
written by the build; the user creates the new save through Minecraft at runtime.

Scene magic remains PTSCE012, position/world mapping sizes remain unchanged.
No proactive scan expansion, height-window remap, stable save pairing or complete
travel lifecycle is claimed. UI/mixin runtime application, initial spawn, movement
hold/resume and empty-world gameplay are not tested.


