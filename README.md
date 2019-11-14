# sltest
Demonstrates potential memory leak in SLED.

```
#cd sltest 
#cargo build --release 
#./target/release/sltest > x.log
# top -p `pgrep sltest`
```
watch RES (Resident Set Size) grow unbounded
