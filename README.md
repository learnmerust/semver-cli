# semver-cli &middot; ![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![Build Status](https://travis-ci.org/learnmerust/semver-cli.svg?branch=master)](https://travis-ci.org/learnmerust/semver-cli) ![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)

A Rust CLI application for working with the [semver](https://semver.org/) specification

### Motivation
[node-semver](https://github.com/npm/node-semver) can be noticeably slow if you're doing many parses. How much faster could it be if we just used some existing glue from rust's ecosystem?

I wrote a script that would parse my git history (to generate a CHANGELOG.md from commits + tags), eg:  
```
bash changelog.sh 124cef4 57175e39d4e2c49a618668ac155f85b3585835b7 > CHANGELOG.md
```

time command execution on the above script (implemented with node-semver):
```
real    1m7.698s
user    0m45.112s
sys     0m14.046s
```
time command execution on the same script (using a debug version of this implementation):
```
real    0m13.831s
user    0m3.530s
sys     0m4.337s
```
time command execution on a `--release` build:
```
real    0m10.202s
user    0m2.527s
sys     0m3.627s
```
Fast!

### Installation
```
cargo install semver-cli
```

### Disclaimer
This is still very much a work in progress, see: [Todo](#todo)

### Usage
validate:
```
semver-cli 2.3.2
2.3.2
```
invalidate:
```
semver-cli 100
-> no output
```
increment:
```
semver-cli 2.3.1 --increment
2.3.2
semver-cli 2.3.1 --increment=minor
2.4.0
semver-cli 2.3.1 --increment=major
3.0.0
```
### Todo
- <strike>increment</strike>
- range
- preid
- loose
- include-prerelease
- coerce

(feature parity with https://github.com/npm/node-semver)
### License

[MIT licensed](./LICENSE)
