# Enemy Ideas

## Behavior Implementation
- **Wandering** - Random movement in a direction, changes periodically
- **FollowAndAttack** - Chase player, stop to attack when in range
- **Coward** - Run away from player when within distance threshold

## Data/Scaling
- Enemy data expansion - Add `behavior` field to `EnemyData` in `.ron` so different enemy types have different default behaviors
- Scaling per round - Enemies get stronger as rounds progress (more health, damage, speed)
- XP/Level system - Enemies gain levels based on corruption/rounds

## Combat
- Attack cooldowns - Enemies can only attack every X seconds
- Attack range - Melee vs Ranged enemies (ranged could be a behavior)
- Damage types - Physical, Poison, Fire, etc.

## Variety
- Elite enemies - Special flag, stronger, different appearance
- Boss enemies - Multiple behaviors, phases, larger health pool
- Death effects - Explode on death (map modifier), spawn children, etc.

## AI/State Machine
- State machine - More complex transitions between Idle, Wandering, Chasing, Attacking, Fleeing
- Aggro system - Switch between player and other targets
- Group behaviors - Flocking/swarming (could be a new behavior)

## Drops/Loot
- Drop system - Enemies drop items based on rarity and map theme
- Loot table - Per enemy type or per map theme