use crate::display_options::DisplayOptions;

/// Represents options for displaying samples from a dataframe.
#[derive(Clone)]
pub struct SampleOptions {
  /// Size or number of elements of the sample set.
  pub sample_size: usize,
  /// Whether or not the retrieved samples are randomly selected.
  pub shuffle: bool,
  /// Display options for the elements of the sample set.
  pub display_options: DisplayOptions,
}

impl SampleOptions {
  /// Creates a new instance of `[SampleOptions]`.
  pub fn new() -> Self {
    Self::default()
  }

  /// Gets the builder for these sample options.
  pub fn builder() -> SampleOptionsBuilder {
    SampleOptionsBuilder::default()
  }
}

impl Default for SampleOptions {
  fn default() -> Self {
    Self {
      sample_size: usize::MAX,
      shuffle: false,
      display_options: DisplayOptions::new(),
    }
  }
}

/// Represents a builder for `[SampleOptions]`.
pub struct SampleOptionsBuilder {
  /// Size or number of elements of the sample set.
  pub sample_size: usize,
  /// Whether or not the retrieved samples are randomly selected.
  pub shuffle: bool,
  /// Display options for the elements of the sample set.
  pub display_options: DisplayOptions,
}

impl SampleOptionsBuilder {
  /// Creates a new instance of `[SampleOptionsBuilder]`.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the size of the samples set.
  pub fn sample_size(
    mut self,
    sample_size: usize,
  ) -> Self {
    self.sample_size = sample_size;
    self
  }

  /// Sets whether the set will have random samples.
  pub fn shuffle(
    mut self,
    shuffle: bool,
  ) -> Self {
    self.shuffle = shuffle;
    self
  }

  /// Builds the instance of `[SampleOptions]`.
  pub fn build(self) -> SampleOptions {
    SampleOptions {
      sample_size: self.sample_size,
      shuffle: self.shuffle,
      display_options: self.display_options,
    }
  }
}

impl Default for SampleOptionsBuilder {
  fn default() -> Self {
    Self {
      sample_size: usize::MAX,
      shuffle: false,
      display_options: DisplayOptions::default(),
  }
}
}