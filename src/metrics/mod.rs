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

    impl Metric {
        fn get_metric(m_type: MetricName, s: String, v: u8) -> Metric {
            Metric {
                metric_type: m_type,
                subfield: s,
                value: v
            }
        }
    }
}
