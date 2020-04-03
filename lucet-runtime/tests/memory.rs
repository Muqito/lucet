use lucet_runtime_tests::memory_tests;

cfg_if::cfg_if! {
    if #[cfg(feature = "uffd")] {
        memory_tests!(
            mmap => lucet_runtime::MmapRegion,
            uffd => lucet_runtime::UffdRegion
        );
    } else {
        memory_tests!(mmap => lucet_runtime::MmapRegion);
    }
}
