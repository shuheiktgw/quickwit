// Copyright 2021-Present Datadog, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Context;
use opentelemetry::metrics::MeterProvider;
use opentelemetry_otlp::{MetricExporter, OTEL_EXPORTER_OTLP_METRICS_PROTOCOL};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::SdkMeterProvider;

use super::metrics_exporter::OtlpMetricsRecorder;
use crate::otlp::OtlpExporterConfig;

pub(crate) fn build_recorder(
    otlp_config: &OtlpExporterConfig,
    resource: Resource,
) -> anyhow::Result<(OtlpMetricsRecorder, SdkMeterProvider)> {
    let metric_exporter =
        if otlp_config.protocol_env_var_is_set(OTEL_EXPORTER_OTLP_METRICS_PROTOCOL) {
            MetricExporter::builder().build()
        } else {
            MetricExporter::builder().with_tonic().build()
        }
        .context("failed to initialize OTLP metrics exporter")?;
    let metrics_provider = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_periodic_exporter(metric_exporter)
        .build();
    let meter = metrics_provider.meter("quickwit");

    let recorder = OtlpMetricsRecorder::new(meter);
    for (name, buckets) in quickwit_metrics::histogram_buckets() {
        recorder.set_histogram_bounds(&metrics::KeyName::from(name), buckets);
    }
    Ok((recorder, metrics_provider))
}
