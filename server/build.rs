macro_rules! check_exclusive {
  ($feat_a:literal, $feat_b:literal) => {
    #[cfg(all(feature = $feat_a, feature = $feat_b))]
    compile_error!(concat!(
      "Features '",
      $feat_a,
      "' and '",
      $feat_b,
      "' cannot be used together. Only one allocator is allowed."
    ));
  };
}

fn main() {
  #[cfg(not(any(feature = "stdalloc", feature = "mimalloc", feature = "jemalloc")))]
  compile_error!("One of the features 'stdalloc, 'mimalloc' or 'jemalloc' should be enabled");

  check_exclusive!("stdalloc", "mimalloc");
  check_exclusive!("stdalloc", "jemalloc");
  check_exclusive!("mimalloc", "jemalloc");
}
