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
use opentelemetry_otlp::{LogExporter, OTEL_EXPORTER_OTLP_LOGS_PROTOCOL};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::logs::SdkLoggerProvider;

use crate::otlp::OtlpExporterConfig;

pub(crate) fn init_logger_provider(
    otlp_config: &OtlpExporterConfig,
    resource: Resource,
) -> anyhow::Result<SdkLoggerProvider> {
    let log_exporter = if otlp_config.protocol_env_var_is_set(OTEL_EXPORTER_OTLP_LOGS_PROTOCOL) {
        LogExporter::builder().build()
    } else {
        LogExporter::builder().with_tonic().build()
    }
    .context("failed to initialize OTLP logs exporter")?;
    Ok(SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(log_exporter)
        .build())
}
