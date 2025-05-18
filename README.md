# Dopamine
A fast, lightweight and (almost) feature-complete internal gamehack for Counter-Strike: Source (recently for 64-bit)

<details>
  <summary>Content table <i>(click to expand)</i></summary>

  - [Features](#features)
  - [Planned](#planned)
  - [Known problems](#known-problems)
</details>

<details>
  <summary>Screenshots <i>(click to expand)</i></summary>

  ![Allies](/assets/allies.jpg)
  ![Enemies](/assets/enemies.jpg)
</details>

## Features
<ul>
  <li><b>Misc</b></li>
  <ul>
    <li>Bunnyhop</li>
  </ul>
  <li><b>Visuals</b></li>
  <ul>
    <li>Stream-proof no-scope crosshair</li>
    <li>Fov changer</li>
    <li>Viewmodel origin changer</li>
  </ul>
  <li><b>Chams</b></li>
  <ul>
    <li>Modes:</li>
    <ul>
      <li>Visible</li>
      <li>Occluded</li>
    </ul>
    <li>Material features:</li>
    <ul>
      <li>Color & alpha modulation</li>
      <li>Wireframe</li>
    </ul>
    <li>Materials:</li>
    <ul>
      <li>Regular</li>
      <li>Flat</li>
    </ul>
    <li>Layer system (up to <b>4</b> for both visible and occluded models layers simultaneously!)</li>
  </ul>
  <li><b>Glow</b></li>
  <ul>
    <li>Color & alpha modulation</li>
    <li>Fade out when spotted with ability to edit fading rate</li>
  </ul>
  <li>
    <b>
      Stream-proof menu powered by
        <a href="https://github.com/ocornut/imgui/">ImGui</a>
    </b>
  </li>
  <li><b>Unload at any moment</b></li>
</ul>

## Planned
- There's no particular plans (yet might be in the future) because I work on this project only when I have kind of an <i>inspiration</i>.

## Known problems
- Intentional memory leak in <b>KeyValues</b> struct
- If enabled at least one ignore-z layer along with a glow then ignore-z chams shall be visible even if model isn't occluded
