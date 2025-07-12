# Todos

## Example dir structure

``` txt
11/
    11-21/
        11-21-31/
        11-21-32.txt
    11-22.txt
    11-23.md
12
```

## Blacklist all nodes except those matching `./**/*.txt`

``` gitignore
*
!*/
!**/*.txt
```

## Blacklists all nodes except those matching `./*.txt`

``` gitignore
*
!*.txt
```

## Blacklists nodes subordinated to `./foo`

``` gitignore
/foo
```

# Blacklists nodes subordinated to `./**/foo`

``` gitignore
foo
```

``` gitignore
**/foo
```

## Blacklists anything

``` gitignore
# Blacklists nodes subordinated to `./*`.
/*
```

``` gitignore
# Blacklists nodes subordinated to `./**/*`.
*
```

## Blacklisting directories

``` gitignore
Blacklists directories subordinated to `./**/foo`.
In this regard, symbolic links are not directories.
foo/
```

``` gitignore
Blacklists nodes (even directories) subordinated to `./**/*.d`.
*.d
```

## Parent Directory

The specification of at least one parent directory induces git to interpret the pattern relative to the root directory.

``` gitignore
Blacklists nodes subordinated to `./**/foo/bar`.
**/foo/bar
```

``` gitignore
Blacklists nodes subordinated to `./foo/bar`.
foo/bar
```

Contrast this with:

``` gitignore
Blacklists nodes subordinated to `./**/bar`.
bar
```
