# https://just.systems

[working-directory: '02_echor']
echo *TEXT:
  @cargo run --quiet -- {{TEXT}}

default:
    echo 'Hello, world!'
