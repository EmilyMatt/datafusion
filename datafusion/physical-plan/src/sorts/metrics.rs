use crate::metrics::{ExecutionPlanMetricsSet, MetricBuilder, Time};

#[derive(Clone, Debug)]
pub struct LexSortMetrics {
    pub time_evaluating_sort_columns: Time,

    pub time_calculating_lexsort_indices: Time,

    pub time_taking_indices_in_lexsort: Time,
}

impl LexSortMetrics {
    pub fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self {
        Self {
            time_evaluating_sort_columns: MetricBuilder::new(metrics)
                .subset_time("time_evaluating_sort_columns", partition),
            time_calculating_lexsort_indices: MetricBuilder::new(metrics)
                .subset_time("time_calculating_lexsort_indices", partition),
            time_taking_indices_in_lexsort: MetricBuilder::new(metrics)
                .subset_time("time_taking_indices_in_lexsort", partition),
        }
    }
}
