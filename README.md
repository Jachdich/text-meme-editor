# Overview

Note: this is highly unstable. It reads and writes to `mem1.txt` only currently.

Use [tiv](https://github.com/stefanhaustein/TerminalImageViewer) to convert a meme template to a .txt file. Then make sure you have rust and cargo installed, and run `cargo run` in this directory. It doesnt render the screen 'till you press a key.

# Controls

 - Tab switches between image focus and colour palette focus
 - Ctrl+e switches between image focus and character sheet focus
 - Ctrl+s to save
 - Ctrl+c to quit (doesn't save or prompt to save)
 - In image focus:
    - current tool displayed at top left
    - pen down status is displayed below current tool
    - p to switch to pen tool (draws with selected character)
    - o to switch to paint tool (changes colour of chars)
    - t to switch to text tool (type characters directly)
    - g to set the current foreground and background colour to the colour of the currently selected character
    - h to set colours like g and also set the pen char to the currently selected character
    - enter to toggle pen down
    - with the pen down, no controls work except pen up. This is so you can type in text mode.
 - In colour palette focus:
    - enter selects foreground colour
    - backspace selects background colour
    - arrow keys to navigate
    - r to toggle foreground default mode (does not change existing char colour)
    - shift+r to toggle background default mode
    - # to enter a custom hex value for the foreground colour
    - shift+# to enter a custom hex value for the backgroubnd colour
 - In character sheet focus:
    - arrows to select character to use for the pen
    - Ctrl+e to switch back

