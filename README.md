## Zudjn
Zudjn is a small esolang that has 2d cell based memory.

### syntax

| Symbol | Description |
|:------:|:-----------|
| `<` | go one memory cell left |
| `>` | go one memory cell right |
| `^` | go one memory cell up |
| `v` | go one memory cell down |
| `+` | add one to the current memory cell, put a number (0-9) before it for faster writing |
| `-` | substract one from the current memory cell, put a number (0-9) before it for faster writing |
| `x` | set the width of the memory cell grid, has to be at the start (example: set the width to 4: 4x) |
| `y` | set the height of the memory cell grid, has to be at the start (example: set the height to 4: 4y) |
| `@` | output the current cell value as a number |
| `&` | output the current cell value as an ascii character |
| `?` | if the current memory cell value equals a number, do the code in the brackets, (example: if the current cell value is equal to 5, print it as an ascii character: 5?[&] ) |
| `\|` | output the cell grid as a whole (this is mainly used for debugging, but it can be used in other ways) |
| `/` | if you put two, any text in between them will be ignored |

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

### Links
[esolangs.org page](https://esolangs.org/wiki/Zudjn)
