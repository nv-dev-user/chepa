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
- Renderer
- Input
- Action
- Zone
- Rewards

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
- Logique des input actions
- Implémentation des logiques de calculs (drop items, gain exp, etc...)
- Gestion des probas de spawn + lancement d'un combat

### Nicolas
- Réalisation des models :
    - Zone
    - PNJ (ensuite transformé en LivingEntity et NPC par Paul)
- Réalisation des controllers :
    - LivingEntityController
- Réalisation de services :
    - Rendu de déplacement (commencé)
    - Jsonparser (refonte complète)
- Réalisation de la structure du jeu (setup/run(update/render))
- Déplacement du joueur

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
 
- Actions :
      - Affichage
      - Gestion

- Services :
    - Zone
    - Input

### Martin

- Modèle :
    - Metadata

- Services :
    - JsonParser (commencé)
    - Render de déplacement
    - Render de Combat
    - Zone (ajout)
- Système de combat :
      - relier interface
