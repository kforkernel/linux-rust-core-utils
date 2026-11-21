# echo — Linux Rust Core Utils

## NAME
`echo` - display a line of text

## SYNOPSIS
```bash
echo [OPTION]... [STRING]...
```

## DESCRIPTION
Print the `STRING`(s) to standard output, separated by a single space and followed by a newline.

## OPTIONS
Flags must appear **before** any text arguments and cannot be repeated or combined.

* **`-n`**  
  Do not output the trailing newline.
* **`-e`**  
  Enable interpretation of backslash escapes (`\n`, `\t`, `\\`).
* **`--`**  
  End of options. Everything after `--` is printed literally as text.

## PARSING RULES
* **Single argument:** If only one argument is given (e.g., `echo -n`), it is printed literally as text.
* **First word stops flags:** Once a non-flag argument is encountered, all remaining arguments are treated as literal text.
* **No duplicate flags:** Each flag (`-n`, `-e`, `--`) can only be used once per command; any repeat is printed as text.

## EXAMPLES
```bash
# Print a standard message
$ echo Hello, world!
Hello, world!

# Print without a trailing newline
\$ echo -n "No newline here"
No newline here$

# Interpret escape sequences
$ echo -e "Line 1\nLine 2\tTabbed"
Line 1
Line 2	Tabbed

# Print a literal flag using --
echo -- -n
-n
```

## AUTHOR
Written by `@kforkernel`.

## LICENSE
Licensed under the kforkernel Use License (KUL) v1.0. See [LICENSE.md](../LICENSE.md) for details.
