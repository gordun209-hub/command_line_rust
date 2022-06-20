#!/usr/bin/env bash
# define a variable for output dir
OUTDIR="tests/expected"
# test if the output dir does not exist and create if needed
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# one argument with two words
echo "Hello there" > $OUTDIR/hello1.txt
# two args seperated more than one space
echo "Hello"  "there" > $OUTDIR/hello2.txt
# one argument with two spaces and no newline
echo -n "Hello  there" > $OUTDIR/hello1.n.txt
# two args with no newline
echo -n "Hello"  "there" > $OUTDIR/hello2.n.txtt
