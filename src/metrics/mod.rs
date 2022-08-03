pub mod metric {
    pub enum MetricName {
        StoryView,
        PageView,
        ButtonClick,
        Share,
        Error,
        Base,
    }

    pub struct Metric {
        pub metric_type: MetricName,
        pub subfield: String,
        pub value: u8,
    }
}
