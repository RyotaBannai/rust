### reference

- http://carol-nichols.com/2017/04/20/rust-profiling-with-dtrace-on-osx/
- https://qiita.com/nirasan/items/88663cdb1694bd937d88
- https://docs.oracle.com/cd/E24845_01/html/E22189/toc.html

### rust profile setting

```toml
# cargo toml
[profile.release]
debug = true
```

- dtrace profiling: `sudo dtrace -c './target/release/your_binary' -o out.stacks -n 'profile-997 /pid == $target/ { @[ustack(100)] = count(); }'`
- create svg file: `FlameGraph/stackcollapse.pl out.stacks | FlameGraph/flamegraph.pl > graph.svg`
- get `stackcollapse.pl` from [here](git clone https://github.com/brendangregg/FlameGraph)
