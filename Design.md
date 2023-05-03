# Unamed Game

Unamed Game is a game about destroying the GDP of a fictional nation-state in 
the throes of late-stage capitalism. It is a top-down, tile-based, 2D rogue-lite.
The adventurer will be given a set number of rounds to annihilate the economy by 
wandering the map and placing tiles, via cards. 

A round ends when the player is captured by the corpo-fascist State and 
imprisoned. Similarly, the player can choose to end a round by returning to their
base of operations.

Between rounds there may be some meta-progression. This is TBD.

## Characters

The player controls one character on a quest to eradicate GDP. The game is 
initially limited to a single class, though additional classes may be added 
Classes will have customized card sets and unique abilities.

The player may encounter helpful NPCs in their wanderings of the procedurally
generated map.

Examples of friendlies:
* Hippy chick
* Crypto-anarchist

The player will also encounter the agents of oppressive order who will seek to
capture and imprison them.

Examples of enemies:
* Neo-neo-nazis
* Killer drones

## Story

The year is 2157. A global economic and political system run by authoritarian
corporations oppresses the masses. "Buy or die" is the law of the land. You,
with the help of a few comrades-in-arms, set out to destroy this system of
economic oppression and restore hope to humanity.

The game takes place in a near-future world.   

## Progression

## Gameplay

The player is given a fixed number of "rounds" in which to navigate and 
and interact with the map, while evading or defeating State Actors. Being captured
results in a new "round" beginning after some prison time is served by the player.

The core mechanic for interacting with the world are cards. Cards are a 
randomization and gearing element, but they can also affect the map state. 
The contents of the player deck is fixed at the start of the game, but mechanics
within the game may alter those contents to make them more or less favorable.

There are NPCs and events scattered around the map. The player can interact with
these via their cards, or in some cases directly. Some NPCs will sell cards to
the player, some will offer tasks for the player to complete. 

The map will also feature random events which the player can interact with or 
disrupt. For example, if a player encounters a "Brand Launch" event, they could 
place a Pipe Bomb card which would cause the event to fail, wreaking economic 
destruction. 

## Goals
* Reduce the State GDP to zero
* per round:
  * play as many cards as possible to maximized effect
  * interact with as many events and npcs without being captured
  * discover powerups which alter gameplay in subsequent levels
  
## Player Skills
* tactical gameplay of cards to maximize effect
* long and short term resource management?

## Game Mechanics
TBD.

There are probably 5-ish politico-economic tracks/factions that should be tracked.

## Losing
The game ends after a fixed number of rounds. If the State GDP is more than 0, 
the player has lost. A score board of the lowest scores will be maintained. 

## Technical Description
The game will be written in Rust, using Bevy as the game engine. It will support
PC, Mac and Linux, with WebAssembly for browser-based play being a stretch goal.