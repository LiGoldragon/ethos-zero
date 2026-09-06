# Ethos-zero Nexus upgrades

The structural Signal runtime stores schema version 2. It cannot open a
retired version-1 store: v1 archived generated Signal values, whereas v2
stores scalar configuration and assembly facts and uses current typed Datom
for its source manifest.

Stop the v1 Nexus before migrating. The source store is opened with the
native redb writer lock, so an active Nexus or another opener makes the
converter refuse. Never point the target at the old store.

```sh
ethos-zero-migrate-v1 \
  "$XDG_STATE_HOME/ethos-zero-nexus/ethos-zero-nexus.sema" \
  "$XDG_STATE_HOME/ethos-zero-nexus/ethos-zero-nexus-v2.sema"
```

The converter reads the old store and its retired Datomic source map, then
creates two new sibling outputs: the supplied v2 `.sema` store and
`<v2-store>.sources.datom`. The migrated configuration points at the latter.
It does not modify the v1 store or the old manifest. It refuses an existing
target, uses temporary files, and publishes a completed manifest before the
completed store. If it fails, leave the old Nexus stopped, correct the stated
source or target problem, and rerun with a fresh target path.

After the converter reports `Migrated`, configure the structural Nexus with
the emitted v2 store. Retain the v1 store for the old version's inspection
route until its history is no longer needed; do not run the new serving Nexus
against it.
