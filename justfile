# https://just.systems

[working-directory: '02_echor']
echo *TEXT:
  @cargo run --quiet -- {{TEXT}}

[working-directory: '03_catr']
cat *FILES:
  @cargo run --quiet -- {{FILES}}

[working-directory: '04_headr']
head *FILES:
  @cargo run --quiet -- {{FILES}}

[working-directory: '05_wcr']
wc *FILES:
  @cargo run --quiet -- {{FILES}}

[working-directory: '06_uniqr']
uniq *FILES:
  @cargo run --quiet -- {{FILES}}

[working-directory: '07_findr']
find *FILES:
  @cargo run --quiet -- {{FILES}}

[working-directory: '08_cutr']
cut *FILES:
  @cargo run --quiet -- {{FILES}}

[working-directory: '09_grepr']
grep *FILES:
  @cargo run --quiet -- {{FILES}}

[no-cd]
test *OPTIONS:
  @cargo test {{OPTIONS}}
