# Player
A player have multiple data asociated.

* max_movements: 1, increments with cards and abilities.
* movements: 1, decreases for every movement of the player, it reset to max_movements at turn start.
* max_mana: 2, increments with cards, and every 5 turns.
* mana: 2, decreases when used, increses by 1 up to max_mana at turn start.
* id: String
* starting_hand: Hand
* starting_deck: Deck
* current_hand: Hand
* current_deck: Deck
* central_deck: Deck

# Board
Main container of the game. It should update everything accordingly.

* board: List of Squares
* death_pile: DeathPile
* common_deck: Deck
* cards_on_board: PlayedCard
* Players: List of Player
* Direction: List of Directions for the patterns to use (like Up, Down)
* time: ChessTime
* rng: ChessRandom
* events: List of Event

# ChessTime
Its composed by 3 numbers: rounds, turn, movement. On a normal 1 vs 1 there is a lot of redundancy but the system is made for abstraction, not on this specific case. Most things are counted in rounds, but there are exceptions. All time elements (cooldown, cast time, etc) are in this ChessTime and should be updated accordingly. The board will contain a function that's called on_movement, on_turn and on_round, for simplicity's sake.

* round: 0, adds 1 every round
* turn: 0, adds 1 every turn, reset to 0 at round end
* movement: 0, adds 1 every movement, reset to 0 at turn end

# ChessRandom
Its composed by 3 numbers, round_rng, turn_rng and movement_rng. They are updated accordingly on the board.

* round_rng: Rng
* turn_rng: Rng
* movement_rng: Rng

# Card
It contains the info of the card. The id is used to get its Name, Description and other localization stuff.

* id: String
* mana_cost: int

# PlayedCard
It contains a Card, and the id of the player that played it.

* card: Card
* player_id: String

# Deck and Hand
They are the same struct, but named different for natural intuition. You should be able to take, see, put, remove cards individually or in a collection. It should also have a method to see if it has a specific card or a group of cards.

* cards: List of Cards

# Square
The chess square.

* magic: bool, if it is magic or not (for portal building allowance), default false
* pos: Position, the position on the board, given at constructing it, needed
* piece: Optional of Piece, the piece on the Square, default None

# DeathPile
A list of death pieces. Every dead piece should go to here.

* pieces: List of Pieces (with color in it)

# Event
An event to occur in the board, it will be expanded later.

* player_id: String
* when: ChessTime, it is reduced on tick
* event_function: EventFunctionEnum

# EventFunctionEnum
Lazy way to implement (not so) generic functions and not worrying about lifetimes. Currently I only care about these things

* Summon(Piece)
* AddMana
* AddMovement
* ApplyEffect(Position)

# Position
just x, y coordinates. It can be constructed from Axis, Dir and SubDir

* x: int
* y: int

# Axis
1 X
2 Y

# Directions (Dir for short)
1 Up    (x: 0, y: +1) (Axis: Y)
2 Right (x: +1, y: 0) (Axis: X)
3 Down  (x: 0, y: -1) (Axis: Y)
4 Left  (x: -1, y: 0) (Axis: X)

# SubDirection (SubDir for short)
1 Up        (Up)
2 UpRight   (Up + Right)
3 Right     (Right)
4 DownRight (Down + Right)
5 Down      (Down)
6 DownLeft  (Down + Left)
7 Left      (Left)
8 UpLeft    (Up + Left)

Any non Direction Subdirection is considered Diagonal

# Patterns

Every Pattern receives a StartPoint and an EndPoint and returns true if the Points Matches the Pattern. The patterns can ask for extra data, and they might need the board state to check if there are any píeces in the way. The patterns can include or exclude the last square containing a piece depending if the pattern is used to check an Action::Take (includes) or an Action::Move (excludes). Notice that attack also includes pieces in the pattern matching.

The description is: (number_of_squares, can_go_through_pieces, Axis/Dir/SubDir/Diagonal/All, other notes). 
if number_of_squares is -1, then is infinite

1 SubdirectionalPattern (n = int as parameter)
  * KingPattern             (n=1  no SubDir)
2 DirectionalPattern    (n = int as parameter)
  * RookPattern             (n=-1 no  Dir)
  * StructureMovePattern    (n=1  no  Dir)
  * MagicianPattern         (n=2  yes Dir)
  * BalistaAttackPattern    (n=6  yes Dir)
3 DiagonalPattern       (n = int as parameter)
  * BishopPattern           (n=-1 no Diagonal)
  * LeechTakePattern        (n=1  no Diagonal)
4 DirectionalPattern    (Forward = Direction as parameter)
  * PawnMovePattern         (2 no Forward)
  * PawnTakePattern         (1 no ForwardDiagonals, no en passant)
  * SuperPawnMovePattern    (2 no Forward | ForwardDiagonals)
  * SuperPawnTakePattern    (1 no Forward | ForwardDiagonals)
5 ShapePattern          (n = int as parameter)
  * CirclePattern           (distance <= n, yes, All)
    * ArcherAttackPattern       (n = 4)
    * GargoylePattern           (n = 5)
  * SquarePattern           (dx <= n & dy <= n, yes, All)
    * CannonAttackPattern       (n = 3)
6 OtherPattern          (They dont consider Number of Squares, They go through Pieces, dx = int, dy = int as params)
  * KnightPattern           ((dx = 2 & dy = 1) | (dx = 1 & dy = 2))
7 RandomizablePattern   (movement_rn = float, turn_rn = float, round_rn = float, randomly generated, once per movement, turn, and round, respectively)
  * CrazyPawnPattern        (n=2  no  SubDir from turn_rn)
0 CompositePattern      (Merge Patterns, params exists if exists in Child Pattern)
  * QueenPattern            (Rook | Bishop)
  * MermaidPattern          (Knight | King)
  * OniPattern              (Knight | Rook)
  * WitchPattern            (Rook | King)
  * ArcherMovePattern       (Magician | King)

# Actions
Actions that can be done. Some of them can also be received.

* Piece Actions
  1 Move
  2 Attack
  3 Take
  4 Ability
  5 Die
  6 Kill
* Card Actions
  7 Take
  8 PutOnBoard
  9 Play
  10 Discard
* Effect Actions
  11 Tick
  12 Expire
* Board Actions
  13 Movement
  14 Turn
  15 Round
* Other Actions
  16 Summon 

# Properties
Each piece can have a set of properties, most of them dont. If it has a default value, then the piece will act as having that default value. If the property has a default value of NONE, then the game only ask of this property knowing that is an especial one.

1 Life: ammount of times they need to be taken or attacked to actually die (DEFAULT: 1)
2 Current Life: starts being the `Life` property ammount, and goes down everytime they are tried to be taken or attacked (DEFAULT: 1)
3 Attack Damage: damage to life that this piece do on attack (DEFAULT: 1)
4 Take Damage: damage to life that this piece do on take (DEFAULT: 1)
5 Piece List: list of pieces, used in necromancer piece (DEFAULT: NONE)

# Piece Tag
Each piece have tags that define how some actions interact with them. They can be removed, added or modified during gameplay. They can do something on an associated stage of a PieceAction. They are usually written with an abreviated form.

1. Biologic:        (BIO) tag to biological units
2. Structure:       (STR) tag to structures
3. Transportable:   (TRANS) tag to transportable pieces
4. Impenetrable:    (IMP) some effects cannot go through these pieces (balista attack, ram ability, etc)
5. Immune:          (IMM) Cannot receive ability effects
6. Heroic:          (HERO) Cannot receive attacks
7. Demonic:         (DEM) Gives 1 mana back to the owner (on_dead)
8. Dead:            (DEAD) tag to necromancer's controlled units, (on_dead skip any other on_dead of this unit and do nothing)

# Pieces
Every Piece can have its own MovePattern, TakePattern, AttackPattern and Ability

## Classic

Classical Chess Pieces, they can move, take, and use an ability (they could attack if given the capacity). The ability has movement cost (in player rounds), mana cost and a cooldown to use it again. They also have Tags.

1 Pawn
  * tags: BIO, TRANS
  * move: PawmMovePattern
  * take: PawnTakePattern
  * ability: Crowning, it transform into another piece when on the end on the board
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 0
2 Bishop
  * tags: BIO, TRANS
  * move: BishopPattern
  * take: BishopPattern
  * ability: It moves to one direction
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 2
3 Knight
  * tags: BIO, TRANS
  * move: KnightPattern
  * take: KnightPattern
  * ability: Summons two Pawns, one to the left and one to the right
    * Movement cost: 1
    * Mana cost: 1
    * Cooldown: 10
4 Rook
  * tags: STR
  * move: RookPattern
  * take: RookPattern
  * ability: Moves all connected towers into one direction, until it takes a piece or touches the border
    * Movement cost: 2
    * Mana cost: 0
    * Cooldown: 10
5 Queen
  * tags: BIO, HERO
  * move: QueenPattern
  * take: QueenPattern
  * ability: It moves like a knight
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 2
6 King
  * tags: BIO, IMM, HERO
  * move: KingPattern
  * take: KingPattern
  * ability: It teleports in a circle of radius 5
    * Movement cost: 0
    * Mana cost: 0
    * Cooldown: once per game

## Starting

These are the starting pieces on the board that doesn't appear on a classical chess game. They have an added attack pattern, and a cast time for the ability, period in which they cannot play and the ability acts when it finishes casting. If any of the actions is none, they cannot do that action unless granted by another ability, card, or other way.

7 Archer
  * tags: BIO, TRANS
  * move: ArcherMovePattern
  * take: None
  * attack: ArcherAttackPattern
  * ability: None
8 Balista
  * tags: STR
  * move: StructureMovePattern
  * take: None
  * attack: BalistaAttackPattern
  * ability: None
9 Builder
  * tags: BIO, TRANS
  * move: MagicianMovePattern
  * take: LeechTakePattern
  * attack: None
  * ability: Puts 3 walls in the given Direction
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 10
    * Cast time: 0
10 Cannon
  * tags: STR
  * move: StructureMovePattern
  * take: None
  * attack: CannonAttackPattern
  * ability: None
11 Catapult
  * tags: STR
  * move: StructureMovePattern
  * take: None
  * attack: None
  * ability: Throws a TRANS piece into a direction on a maximum of 10 distance
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 4
    * Cast time: 0
12 CrazyPawn
  * tags: BIO, TRANS
  * move: CrazyPawnPattern
  * take: CrazyPawnPattern
  * attack: CrazyPawnPattern
  * ability: Dies and gives 2 cards to the player owner of this piece
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 0
    * Cast time: 0
13 Magician
  * tags: BIO, HERO, IMM, TRANS
  * move: MagicianMovePattern
  * take: None
  * attack: None
  * ability: Given by MagicianAbilityCards
    * Movement cost: 1
    * Mana cost: 2
    * Cooldown: 6
    * Cast time: 0
14 Paladin
  * tags: HERO, IMM
  * move: QueenPattern
  * take: QueenPattern
  * attack: None
  * ability: Given by PaladinAbilityCards
    * Movement cost: 1
    * Mana cost: 2
    * Cooldown: 4
    * Cast time: 0
15 Ram
  * tags: STR
  * move: StructureMovePattern
  * take: None
  * attack: None
  * ability: Rams into one direction, it takes the first unit which it has contact with, and rams behind with a depth of distance_traveled / 5. For every health taken, or square moved, it reduces the depth by one, until reaches 0, or the end of the board. Impacting an IMP unit, drains all remaining depth and damages both units by 1 hp.
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 4
    * Cast time: 1
16 ShieldBearer
  * tags: BIO, TRANS
  * move: PawnMovePattern
  * take: PawnTakePattern
  * attack: None
  * ability: gives IMP to pieces to the left and to the right for the rest of the game.
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 15
    * Cast time: 0
17 Ship
  * tags: STR
  * move: MagicianMovePattern
  * take: KingPattern
  * attack: None
  * ability: do 1 damage to units on all SubDirections, except up and down
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 12
    * Cast time: 0
18 SuperPawn
  * tags: BIO, TRANS
  * move: SuperPawnMovePattern
  * take: SuperPawnTakePattern
  * attack: None
  * ability: Adds IMM and IMP to itself
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 10
    * Cast time: 0
19 TeslaTower
  * tags: STR
  * move: MagicianMovePattern
  * take: StructureMovePattern
  * attack: None
  * ability: apply deactivate effect to STR pieces on a CannonAttackPattern for 6 rounds
    * Movement cost: 2
    * Mana cost: 0
    * Cooldown: 10
    * Cast time: 1
20 Wall
  * tags: STR, IMP
  * move: None
  * take: None
  * attack: None
  * ability: Deconstruct itself, dying, its free to use.
    * Movement cost: 0
    * Mana cost: 0
    * Cooldown: 0
    * Cast time: 0
21 Warlock
  * tags: TRANS, DEM, IMM
  * move: MagicianMovePattern
  * take: None
  * attack: None
  * ability: Casts for 3 rounds, and creates a portal, refunds 2 mana on end
    * Movement cost: 1
    * Mana cost: 3
    * Cooldown: 5
    * Cast time: 3

## Portal
Portal Units have an associated cost, in rounds, mana, and a requirement of number of portals

22 Portal
  * tags: STR, IMP, IMM, HERO
  * move: None
  * take: None
  * attack: None
  * ability: Summons an unit after a cast time, depending on the unit its the cost and if it needs more portals to work. Needs a SubDirection to put the piece
    * Movement cost: 1
    * Mana cost: depending on the unit
    * Cooldown: 0
    * Cast time: depending on the unit
23 Basilisk
  * tags: DEM, IMM
  * move: BishopPattern
  * take: BishopPattern
  * attack: None
  * ability: deactivate units on a bishop pattern for 4 rounds
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 20
    * Cast time: 0
  * requirements: 1 portal
    * rounds: 5
    * mana: 1
24 Dragon
  * tags: IMM, HERO, BIO, DEM
  * move: MermaidPattern
  * take: MermaidPattern
  * attack: None
  * ability: It applies the fire effect on a 3*5 area on a direction on all enemy units for 4 rounds
    * Movement cost: 2
    * Mana cost: 1
    * Cooldown: 2
    * Cast time: 0
  * requirements: 2 portal
    * rounds: 15
    * mana: 3
25 Gargoyle
  * tags: DEM
  * move: GargoylePattern
  * take: GargoylePattern
  * attack: None
  * ability: None
  * requirements: 1 portal
    * rounds: 5
    * mana: 1
26 Golem
  * tags: HERO, DEM, IMM, STR, IMP
  * property: 4 life
  * move: ArcherMovePattern
  * take: ArcherMovePattern
  * attack: None
  * ability: None
  * requirements: 1 portal
    * rounds: 5
    * mana: 0
27 Imp
  * tags: DEM, BIO
  * move: KnightPattern
  * take: None
  * attack: None
  * ability: choose one: take one card from your PersonalDeck into your Hand, or, put a card from the CentralDeck into your PersonalDeck and shuffle
    * Movement cost: 1
    * Mana cost: 2
    * Cooldown: 5
    * Cast time: 0
  * requirements: 1 portal
    * rounds: 5
    * mana: 2
28 Mandragora
  * tags: DEM, BIO
  * move: KingPattern
  * take: None
  * attack: None
  * ability: adds 1 to max mana this and next turn (2 max mana in total)
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 10
    * Cast time: 0
  * requirements: 1 portal
    * rounds: 5
    * mana: 2
29 Mermaid
  * tags: DEM, BIO
  * move: MermaidPattern
  * take: MermaidPattern
  * attack: None
  * ability: Reduces cast time by 2 rounds of nearby portals
    * Movement cost: 1
    * Mana cost: 2
    * Cooldown: 5
    * Cast time: 0
  * requirements: 2 portal
    * rounds: 3
    * mana: 0
30 Necromancer
  * tags: DEM
  * property: Piece List of controlled units (on die -> all units die)
  * move: MagicianMovePattern
  * take: None
  * attack: None
  * ability: kills and controls an enemy unit, removes all tags and add DEAD tag.
    * Movement cost: 2
    * Mana cost: 1
    * Cooldown: 4
    * Cast time: 0
  * requirements: 1 portal
    * rounds: 7
    * mana: 2
31 Ogre
  * tags: DEM, IMP
  * property: 2 hp
  * move: ArcherMovePattern
  * take: ArcherMovePattern
  * attack: None
  * ability: pushes a unit in any subdirection. If it lands on a unit, kill that unit, if that unit is immune, kill the pushed unit, if the square is empty, leave the pushed unit in that square.
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 4
    * Cast time: 0
  * requirements: 1 portal
    * rounds: 4
    * mana: 0
32 Oni
  * tags: DEM, BIO, TRANS
  * move: OniPattern
  * take: OniPattern
  * attack: None
  * ability: it teleports to the enemy magician, it ask in which subdirection relative to the magician will teleport to
    * Movement cost: 1
    * Mana cost: 1
    * Cooldown: 7
    * Cast time: 0
  * requirements: 2 portals
    * rounds: 3
    * mana: 1
33 Spider
  * tags: DEM, TRANS, BIO
  * move: KnightPattern
  * take: KnightPattern
  * attack: None
  * ability: puts a SpiderEgg up or down of this piece
    * Movement cost: 1
    * Mana cost: 1
    * Cooldown: 12
    * Cast time: 0
  * requirements: 1 portal
    * rounds: 5
    * mana: 0
34 SpiderEgg
  * tags: DEM, BIO
  * move: None
  * take: None
  * attack: None
  * ability: it transform into a spider after 5 rounds (autocast)
    * Movement cost: 0
    * Mana cost: 0
    * Cooldown: 0
    * Cast time: 5
35 Succubus
  * tags: BIO, DEM, TRANS
  * move: BishopPattern
  * take: BishopPattern
  * attack: None
  * ability: deactivate a piece (at any range) for 6 rounds
    * Movement cost: 1
    * Mana cost: 1
    * Cooldown: 10
    * Cast time: 0
  * requirements: 1 portal
    * rounds: 12
    * mana: 1
36 Witch
  * tags: DEM
  * move: WitchPattern
  * take: WitchPattern
  * attack: None
  * ability: attacks for 2 hp all units in a KingPattern, including allies
    * Movement cost: 1
    * Mana cost: 2
    * Cooldown: 8
    * Cast time: 2
  * requirements: 1 portal
    * rounds: 12
    * mana: 1


## Other units
These other units need to have al the info shown up on here.

37 Swamp
  * tags: STR
  * move: None
  * take: None
  * attack: None
  * ability: Can summon Swamp Units
    * Movement cost: 1
    * Mana cost: 0
    * Cooldown: 5
    * Cast time: given by the unit rounds info
  * spawn from: Cards
38 Leech
  * tags: BIO
  * move: KingPattern
  * take: LeechTakePattern
  * attack: None
  * ability: attacks for 2 hp all units in a KingPattern, including allies
    * Movement cost: 1
    * Mana cost: 2
    * Cooldown: 8
    * Cast time: 2
  * requirements: 1 swamp
    * rounds: 1
    * mana: 0

# Effects
Effects have multiple functions (on_action_done, on_action_received, can_action_be_done, can_action_be_received, on_tick, on_expire, on_apply). They are empty functions by default (can return true), and they are overriten by each case.
* Fire              (on_expire -> die) (on_apply -> if is STR, divide time by 2)
* Ice               (on_tick -> setmoved = true) (on_apply -> if is STR, divide time by 2)
* Deactivate        (on_tick -> setmoved = true) (on_apply -> if is not STR, divide time by 2)
* Invulnerability   (can_action_be_received -> false)

# Decks
From closest to furthest

* Hand:         (accesible only to you, you can play cards from here)
* PersonalDeck: (accesible only to you)
* CentralDeck:  (accesible for both, altough both players have one)
* CommonDeck:   (only one per board)

## Card Holders
This are cards holders, include decks but also includes other things

* StartingHand: cards that always start in your hand, they are not taken from anywhere
* Hand: cards that you can play
* BoardCards: cards that have been played and have permament effects, like MagicianAbilityCards
* StartingDeck: cards that you start with on your PersonalDeck
* PersonalDeck: cards that you can take onto your hand
* CentralDeck: cards that you can take onto your hand or PersonalDeck
* CommonDeck: cards that all players can take
* InfinityPool: all cards in infinity ammount, this exists as a technichality more than anything playable

## StartingHand
This is personalizable for each user. But the default is

* AddMovement
* AddMovement
* Transform
* SummonKnight
* SummonWarlock

## StartingDeck
This is personalizable for each user. But the default is

* 2xAddMovement
* 4xAddMana
* 2xSummonWarlock
* 1xSummonSwamp
* 4xTransform
* 2xSummonRook
* 1xFireCard
* 1xIceCard
* 1xAttackToDemonicCard

# Cards

## SummonCards
They summon units, the valid square are any square near an ally piece (in any subdirection), and a CannonPattern around the king. In parenthesis is mana cost.
* SummonKnight  (2)
* SummonSwamp   (2)
* SummonRook    (2)
* SummonWarlock (2)

## UtilityCards
They are utility cards used for various thingss. In parenthesis is mana cost.

* AddMovement: Adds 1 movement per turn permanently         (0)
* AddMana: adds 1 max mana permanently                      (0)
* Transform: it transform into a card of your PersonalDeck  (1)

## MagicianAbilityCards
Gives Magician piece abilities. They also provide a passive ability to the magician. They activate all at once. They are all OnBoardCards.

* FireCard: gives fire effect for 6 rounds to every piece on a ArcherMovePattern area   (2)
  * Passive: Cancels the ice effect to every piece on a ArcherMovePattern area
* IceCard: gives ice effect for 4 rounds to every piece on a ArcherMovePattern area     (2)
  * Passive: Cancels the fire effect to every piece on a ArcherMovePattern area

## PaladinAbilityCards
Gives Paladin piece abilities. Paladin can only activate one of them at a time. They are all OnBoardCards.

* AttackToDemonicCard: to enemy, attacks to other piece if it has the demonic tag, on a CannonAttackPattern     (2)
* InvulnerabilityCard: to ally, adds invulnerability effect for 3 rounds to them, on a ArcherMovePattern        (3)
* ReviveCard: on an empty square, revive a dead ally unit (it needs to be on the deathPile), on a KingPattern   (3)
