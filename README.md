# pgen

Simple password generator supporting various formats and entropy estimates

## Example:

```
$ ./pgen 20
  m9>OSizZ$h6?LM=mRAV~ (entropy: 128.8 bits)
  Y8BQWSVBJYWS32H9YJVF (entropy: 102.600006 bits)
  YY7YB-4MKIE-PLGGJ-THIRI (entropy: 103.4 bits)
```

## How it works

The command is invoked with an argument specifying the desired password length in characters.  Several random passwords are generated in different formats, using bytes from `/dev/random`.  Entropy estimates are provided for each.

Some common difficult-to-read characters (such as 'l' and '1') are excluded from the generated passwords.  This is taken into account in the entropy calculation.

## How to build

Clone the repo and run `make`
