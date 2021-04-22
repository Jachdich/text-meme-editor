# Overview

Note: this is highly unstable. It reads and writes to `mem1.txt` only currently.

Use [tiv](https://github.com/stefanhaustein/TerminalImageViewer) to convert a meme template to a .txt file. Then make sure you have rust and cargo installed, and run `cargo run` in this directory. It doesnt render the screen 'till you press a key.

# Controls

 - Tab switches between image focus and colour palette focus
 - Ctrl+e switches between image focus and character sheet focus
 - In image focus:
    - current tool displayed at top left
    - pen down status is displayed below current tool
    - p to switch to pen tool (draws with selected character)
    - o to switch to paint tool (changes colour of chars)
    - t to switch to text tool (type characters directly)
    - enter to toggle pen down
    - with the pen down, no controls work except pen up. This is so you can type in text mode.
 - In colour palette focus:
    - enter selects foreground colour
    - backspace selects background colour
    - arrow keys to navigate
    - d to toggle foreground default mode (does not change existing char colour)
    - ctrl+d to toggle background default mode
 - In character sheet focus:
    - arrows to select character to use for the pen
    - Ctrl+e to switch back

