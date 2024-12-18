# Rust program to print ASCII table/values

```
$ asciir -h
Print ASCII table/values

Usage: asciir [VAL]...

Arguments:
  [VAL]...  Input file(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Run with no arguments to print the ASCII table:

```
$ asciir
 33: !   52: 4   71: G   90: Z  109: m
 34: "   53: 5   72: H   91: [  110: n
 35: #   54: 6   73: I   92: \  111: o
 36: $   55: 7   74: J   93: ]  112: p
 37: %   56: 8   75: K   94: ^  113: q
 38: &   57: 9   76: L   95: _  114: r
 39: '   58: :   77: M   96: `  115: s
 40: (   59: ;   78: N   97: a  116: t
 41: )   60: <   79: O   98: b  117: u
 42: *   61: =   80: P   99: c  118: v
 43: +   62: >   81: Q  100: d  119: w
 44: ,   63: ?   82: R  101: e  120: x
 45: -   64: @   83: S  102: f  121: y
 46: .   65: A   84: T  103: g  122: z
 47: /   66: B   85: U  104: h  123: {
 48: 0   67: C   86: V  105: i  124: |
 49: 1   68: D   87: W  106: j  125: }
 50: 2   69: E   88: X  107: k  126: ~
 51: 3   70: F   89: Y  108: l  127: DEL
```

Run with one or more values to convert between codepoints and characters:

```
$ asciir 35 %
 35 = #
  % = 37
```

Arguments that cannot be parsed as a `u8` in the range 33-126 or strings that are not exactly one character or are not ASCII values will be printed to `STDERR`:

```
$ asciir Hey "" 😁 0 127
Input "Hey" must be a single character
Input "" must be a single character
😁 is not an ASCII value
Codepoint 0 not in the range 33-126
Codepoint 127 not in the range 33-126
```

## Author

Ken Youens-Clark <kyclark@gmail.com>
