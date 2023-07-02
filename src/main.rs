mod hashmap;
mod linklist;

fn main() {
}

type DeriveIndex = u32;

struct Storage {
    lru: LRU,
    mem_map: MemMap,
    sync_map: SyncMap,
    key_map: KeyMap,

}

struct LRU {}

struct SyncMap {}

struct KeyMap {}

struct MemMap {}