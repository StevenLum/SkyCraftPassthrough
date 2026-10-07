# Passthrough design

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
| Minecraft to Skyrim, later rendering work | World, hand, and interface images plus depth information so objects hide behind the correct surfaces. |

Use Windows named shared memory for state and messages, as SkyCraft does. Step 1
uses a named mutex (a lock shared by the two processes) to protect its small
connection record. Later steps add event queues and frame data as needed; the
rendering step adds shared GPU textures. Give this project its own memory names
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
6. **Connect interactions.** Add NPC stand-ins in Minecraft, damage in both
   directions, block placement, and Skyrim NPC collision with those blocks.
7. **Handle world changes and saves.** Add doors/interiors, water, time/weather,
   and matched saves so loading Skyrim restores the matching Minecraft state.

Step 2 is implemented; the user's live logs now confirm successful Skyrim loading.
Step 3 adds sampled collision export
and NPC position/body boxes. It does not add camera control, rendering
composition, combat, or save sync. Live logs confirm matching collision/NPC
transport and collision-query matches on both Minecraft sides. The user verified
movement stops at a Skyrim wall and NPC, and jumping/landing work. General
coverage still needs verification. This change implements step 4 look-direction
transfer using Minecraft's window and controls. Skyrim-window input forwarding
is a separate later part. New camera behavior is not tested in the games yet.
