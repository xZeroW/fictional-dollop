# 🎮 Game Design Document

## Core Concept
- **Genre:** 2D Pixel Art Roguelite
- **Perspective:** Top-down
- **Session Structure:** Timed rounds (5 minutes each)
- **Core Twist:** Player progression happens by upgrading **monsters**, not the hero

---

## 🗺️ World & Maps

### Map Structure
- Rectangular maps
- Built using **Tiled**
- Integrated via `bevy_ecs_tiled`

### Map Selection
- A random **themed map** is chosen each 5 rounds

### Map Themes
- Each map has a distinct **theme** (e.g. Forest, Crypt, Factory, Void)
- Each theme defines:
  - Exclusive item drops
  - Monster types
  - Environmental flavor and mechanics

---

## ⏱️ Rounds & Game Flow
- Each round lasts **5 minutes**

### Round Loop
1. Enter themed map
2. Fight monsters and collect loot
3. End of round:
   - Choose monster buffs
   - Access loot and crafting
4. Transition to next map

---

## 👹 Monster Progression System

### Monster Buffs
Instead of upgrading the hero, the player selects **monster buffs** each round.

Possible buffs include:
- Increased speed
- Increased health
- Increased damage
- Critical chance / critical damage
- Special effects (poison, bleed, shields, etc.)

### Risk–Reward Bonuses
Each monster buff grants the player a **bonus**, such as:
- Increased item drop quantity
- Increased item rarity
- Temporary player buffs (round-based or map-based)

---

## 🎒 Loot & Inventory System

### Loot
- Monsters drop items during rounds
- Items have rarity tiers
- Some items are **exclusive to specific map themes**

### Inventory Types

#### Temporary Inventory
- Holds items collected during the current run
- Completely lost on death
- Lose what is not sent to safe inventory (Bank)

#### Safe Inventory (Bank)
- Persistent storage across runs
- Player must manually move items here
- Limited capacity to enforce meaningful decisions

### Death Penalty
- On death:
  - Lose all dropped items
  - Lose temporary inventory
  - Safe inventory remains intact

---

## 🛠️ Crafting System
- Available **between rounds**
- Player can:
  - Inspect collected loot
  - Craft new items
  - Combine materials
- Crafting may consume:
  - Temporary inventory items
  - Safe inventory items (with risk)

---

## 🌳 Progression Systems

### Classes
- Multiple playable classes
- Each class has:
  - Unique mechanics
  - Distinct identity
  - Own passive tree

### Passive Trees

#### Class Passive Tree
- Long-term progression
- Inspired by **Grim Dawn**
- Defines builds and playstyles

#### Map Passive Tree
- Influences:
  - Map theme appearance chance
  - Exclusive drop frequency
  - Potential map modifiers

---

## 🧠 Design Pillars
- Risk vs Reward
- Indirect power scaling
- Strategic planning across multiple rounds
- Strong meta progression layered over short runs

---

# 💡 Extra / Optional Ideas

## Monster Buff Synergies
- Certain monster buffs interact:
  - Speed + Crit → higher rarity drops
  - Tanky monsters → more crafting materials

## Map Modifiers
- Maps can roll 1–2 modifiers:
  - Monsters explode on death
  - Increased elite spawn rate
  - Reduced crafting costs this round

## Corruption System
- Buffing monsters increases corruption
- Higher corruption:
  - Better loot
  - Stronger enemies
  - Boss invasions

## Boss Maps
- Special themed maps
- No timer
- Single powerful boss
- High-risk / high-reward rewards

