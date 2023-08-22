Command line used to find this crash:

/Users/kevinvalerio/.local/share/afl.rs/rustc-1.73.0-nightly-474709a/afl.rs-0.13.3/afl/bin/afl-fuzz -c0 -Mmainaflfuzzer -i./output/ethfuzz/shared_corpus/ -pfast -ooutput/ethfuzz/afl -g500 -G1048576 -F./output/ethfuzz/shared_corpus -V79205 -l2 -x../magic.dict ./target/afl/debug/ethfuzz

If you can't reproduce a bug outside of afl-fuzz, be sure to set the same
memory limit. The limit used for this fuzzing session was 0 B.

Need a tool to minimize test cases before investigating the crashes or sending
them to a vendor? Check out the afl-tmin that comes with the fuzzer!

Found any cool bugs in open-source tools using afl-fuzz? If yes, please post
to https://github.com/AFLplusplus/AFLplusplus/issues/286 once the issues
 are fixed :)

