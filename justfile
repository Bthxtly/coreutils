# https://just.systems

[working-directory: '02_echor']
echo *TEXT:
  @cargo run --quiet -- {{TEXT}} 2>/dev/null

[working-directory: '03_catr']
cat *FILES:
  @cargo run --quiet -- {{FILES}} 2>/dev/null

[working-directory: '04_headr']
head *FILES:
  @cargo run --quiet -- {{FILES}} 2>/dev/null

[working-directory: '05_wcr']
wc *FILES:
  @cargo run --quiet -- {{FILES}} 2>/dev/null

[working-directory: '06_uniqr']
uniq *FILES:
  @cargo run --quiet -- {{FILES}} 2>/dev/null

[no-cd]
test *OPTIONS:
  @cargo test {{OPTIONS}}
