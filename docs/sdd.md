- View
- AppState
- Screen

# Buried Peace - Design Document

## 1. Overview

## 2. Game

## 3. Engine

### 3.1 Overview

#### 3.1.1 Goals

Key attributes of the engine include the following:

- Systemic Gameplay
- Player Agency
- Simplicity
- Development Ease

#### 3.1.2 Engine Overview - Cycle

The engine consists of four foundational parts.

- Player
- World
- Input
- Output

#### 3.1.3 Cycle Concept

All four of these make up a cycle.

    Player ----( Input  )----> World

    World  ----( Output )----> Player

#### 3.1.4 Definitions

Let us define what each one does.

Component   | Definition
------------|----------------------
Player      | The player and their preferences / settings.
World       | The game and its logic
Input       | The input received from the user
Output      | Presents the world to the user

#### 3.1.5 Examples

Each can have sub-components, of which some examples include:

Component   | Examples
------------|------
Player      | Name, Platform, Saved Profile
World       | Physics, Game Logic, Characters, Terrain
Input       | Keyboard, Mouse, Touch, Controller, Buttons, Checkboxes
Output      | Visual (Graphics), Audio, Tactile, Tone, Views

#### 3.1.6 Islands

Two components are "islands": separate concepts we want to build bridges to.

- Player - The person (or machine in some cases) playing the game
- World - The virtual game environment, with logic, systems and laws

#### 3.1.7 Bridges

Two components are "bridges": one-way means of both islands communicating with one another.

- Input - **Player** sends messages to the *world*
- Output - **World** sends messages to the *player*

### 3.2 Player

#### 3.2.1 Overview



### 3.3 World

### 3.4 Input

### 3.5 Output