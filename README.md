## Zudjn
Zudjn is a small esolang that has 2d cell based memory.

### syntax

| Symbol | Description | Implemented |
|:------:|:-----------|:-----:|
| `<` | go one memory cell left | yes |
| `>` | go one memory cell right | yes |
| `^` | go one memory cell up | yes |
| `v` | go one memory cell down | yes |
| `+` | add one to the current memory cell, put a number (0-9) before it for faster writing | yes |
| `-` | substract one from the current memory cell, put a number (0-9) before it for faster writing | yes |
| `x` | set the width of the memory cell grid, has to be at the start (example: set the width to 4: 4x) | yes |
| `y` | set the height of the memory cell grid, has to be at the start (example: set the height to 4: 4y) | yes |
| `@` | output the current cell value as a number | yes |
| `&` | output the current cell value as an ascii character | yes |
| `?` | if the current memory cell value equals a number, do the code in the brackets, (example: if the current cell value is equal to 5, print it as an ascii character: 5?&; ) | yes |
| `;` | ends an if statement container | yes |
| `\|` | output the cell grid as a whole (this is mainly used for debugging, but it can be used in other ways) | yes |
| `/` | if you put two, any text in between them will be ignored | yes |

### examples

Hello World (not in one line): 
```
3x3y
9+9+9+9+9+9+9+9+>
9+9+9+5+>
9+9+9+9+9+9+9+9+9+9+9+2+v
9+9+9+9+9+9+9+9+9+9+9+9+9+2+<
9+9+9+9+9+9+9+9+9+9+9+9+3+<
9+9+9+9+9+9+9+9+9+9+9+9+v
9+9+9+9+9+9+9+9+9+9+9+9+6+>
9+9+9+6+>
9+9+9+9+9+9+9+9+9+9+9++
^^<<&>>&v<<&&>&^&v>&<&<v&^&v>>&<&

```

Hello World (in one line):
```
3x3y9+9+9+9+9+9+9+9+>9+9+9+5+>9+9+9+9+9+9+9+9+9+9+9+2+v9+9+9+9+9+9+9+9+9+9+9+9+9+2+<9+9+9+9+9+9+9+9+9+9+9+9+3+<9+9+9+9+9+9+9+9+9+9+9+9+v9+9+9+9+9+9+9+9+9+9+9+9+6+>9+9+9+6+>9+9+9+9+9+9+9+9+9+9+9++^^<<&>>&v<<&&>&^&v>&<&<v&^&v>>&<&
```

### history
Zudjn was made in 2025 by [@DevThatCodes](https://github.com/DevThatCodes) (me). The name zudjn was chosen when i made a random "word" generator and one of the "words" that it generated was zudjn.

### Links
[esolangs.org page](https://esolangs.org/wiki/Zudjn)
