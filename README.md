# Pnguice to Encode, Decode and Remove message from PNG images

### Use Case

Run program with `pnguice --help` to see the available sub commands

A chunk is a 4 letter case sensitive alphabetic word used to decide where the message should be, different words have different messages, example:

```terminal
pnguice decode img.png ruSt
```

Will look for the message in a different place than

```terminal
pnguice decode img.png rust
```

### Encode

```terminal
pnguice encode img.png ruSt "This is a hidden message" output.png
```

the output is Optional, if it weren't used then the message is encoded into the original image

### Decode

```terminal
pnguice decode img.png ruSt
```

Will print out the image encoded in the chunk `ruSt` or nothing if it were not found

### Remove

```terminal
pnguice remove img.png oTHa
```

Will remove the message encoded into the chunk oTHa if it were found.
