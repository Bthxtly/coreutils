# https://just.systems

[working-directory: '02_echor']
echo *TEXT:
  @cargo run --quiet -- {{TEXT}}

[working-directory: '03_catr']
cat *FILES:
  @cargo run --quiet -- {{FILES}}

default:
    echo 'Hello, world!'
