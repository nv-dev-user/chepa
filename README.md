## MODELS

- Entity
- Zone
- SpawnGroup
- Mob
- Item
- Weapon
- Armor
- PNJ
- Metadata

## SERVICES

- JsonParser


## ROLES

### Paul

- Réalisation des models :
    - Living entity
    - NPC
    - Player
    - Weapon
    - Mob (ensuite transformé en LivingEntity et NPC)
- Logique de gain d'expérience
- Logique de drop d'items

### Nicolas
- Réalisation des models :
    - Zone
    - PNJ (ensuite transformé en LivingEntity et NPC par Paul)
- Réalisation des controllers :
    - LivingEntityController



- Implémentation des logiques de calculs (drop items, gain exp, etc...)

### Thomas

- Modèle :
    - Equipement
- Modification :
    - Armor
    - Weapon

Système de combat :
- LivingEntityCombat
- Modification :
    - LivingEntity
    - Player
    - NPC

- Services :
    - Zone
    - Input

### Les autres

- M -> Metadata, JsonParser
- N -> Zone, PNJ
- T -> Item, Armor