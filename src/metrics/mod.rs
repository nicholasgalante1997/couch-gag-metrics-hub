pub mod metric {

    use crate::url::url::ReqUrl;

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
        pub target: String,
        pub value: u8,
    }

    impl Metric {
        pub fn get_metric(m_type: MetricName, s: String, t: String, v: u8) -> Metric {
            Metric {
                metric_type: m_type,
                subfield: s,
                target: t,
                value: v
            }
        }

        pub fn get_metric_type_as_string(m_type: MetricName) -> String {
            let metric_type_string: String = match m_type {
                MetricName::Base => String::from("couch-gag-base-metric-health-ping"),
                MetricName::ButtonClick => String::from("couch-gag-button-click"),
                MetricName::Error => String::from("couch-gag-error"),
                MetricName::PageView => String::from("couch-gag-page-view-hit"),
                MetricName::Share => String::from("couch-gag-share-story"),
                MetricName::StoryView => String::from("couch-gag-story-view"),
                _ => String::from("couch-gag-missed-metric-type")
            };

            metric_type_string
        }

        pub fn get_metric_subfield_off_query_params(req_url: &ReqUrl) -> String {
            let mut msg = String::new();
            for req_param in req_url.query_parameters.clone() {
                if req_param.0.contains("subfield") {
                    msg = String::from(req_param.1);
                }
            }
            msg
        }

        pub fn get_val_off_query_params(req_url: &ReqUrl) -> u8 {
            let mut val = 0;
            for req_param in req_url.query_parameters.clone() {
                if req_param.0.contains("value") {
                    val = req_param.1.parse::<u8>().unwrap();
                }
            }
            val
        }

        pub fn get_target_string_off_query_params(req_url: &ReqUrl) -> String {
            let mut target = String::new();
            for req_param in req_url.query_parameters.clone() {
                if req_param.0.contains("target") {
                    target = req_param.1
                }
            }
            target
        }
    
        pub fn get_metric_type_off_query_param(req_url: &ReqUrl) -> MetricName {
            let mut metric_type: MetricName = MetricName::Base;
    
            for req_param in req_url.query_parameters.clone() {
                if req_param.0.contains("metric") {
                    metric_type = {
                        if req_param.1.eq("story-view") {
                            MetricName::StoryView
                        } else if req_param.1.eq("page-view") {
                            MetricName::PageView
                        } else if req_param.1.eq("share") {
                            MetricName::Share
                        } else if req_param.1.eq("button-click") {
                            MetricName::ButtonClick
                        } else if req_param.1.eq("base") {
                            MetricName::Base
                        } else {
                            MetricName::Error
                        }
                    }
                }
            }
    
            metric_type
        }
    }
}
