use lsm_tree::{AbstractTree, Config, SequenceNumberCounter};

#[test_log::test]
fn blob_gc_flush_tombstone() -> lsm_tree::Result<()> {
    let folder = tempfile::tempdir()?;

    let tree = Config::new(&folder).open_as_blob_tree()?;

    let seqno = SequenceNumberCounter::default();

    tree.insert("a", "neptune".repeat(10_000), seqno.next());
    tree.insert("b", "neptune".repeat(10_000), seqno.next());
    tree.flush_active_memtable(0)?;

    tree.remove("b", seqno.next());

    tree.gc_scan_stats(seqno.get())?;
    assert_eq!(2.0, tree.blobs.space_amp());

    let strategy = value_log::SpaceAmpStrategy::new(1.0);
    tree.apply_gc_strategy(&strategy, seqno.next())?;
    assert_eq!(1, tree.blobs.segment_count());

    tree.gc_scan_stats(seqno.get())?;
    assert_eq!(1.0, tree.blobs.space_amp());

    tree.flush_active_memtable(0)?;
    assert_eq!(1, tree.blobs.segment_count());

    tree.gc_scan_stats(seqno.get())?;
    assert_eq!(1.0, tree.blobs.space_amp());

    Ok(())
}