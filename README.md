# sltest
Demonstrates potential memory leak in SLED.

```
#cd sltest 
#cargo build --release 
#./target/release/sltest > x.log
#top -p `pgrep sltest`
```

Expected: constant RES (Resident Set Size)

Observed:  RES (Resident Set Size) grows unbounded.
