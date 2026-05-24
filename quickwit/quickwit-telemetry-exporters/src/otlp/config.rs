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

use opentelemetry_otlp::OTEL_EXPORTER_OTLP_PROTOCOL;
use quickwit_common::get_bool_from_env;

pub const QW_ENABLE_OPENTELEMETRY_OTLP_EXPORTER_ENV_KEY: &str =
    "QW_ENABLE_OPENTELEMETRY_OTLP_EXPORTER";

pub(crate) struct OtlpExporterConfig {
    enabled: bool,
}

impl OtlpExporterConfig {
    pub(crate) fn load_from_env() -> Self {
        OtlpExporterConfig {
            enabled: get_bool_from_env(QW_ENABLE_OPENTELEMETRY_OTLP_EXPORTER_ENV_KEY, false),
        }
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn protocol_env_var_is_set(&self, signal_protocol_env_key: &str) -> bool {
        std::env::var_os(signal_protocol_env_key).is_some()
            || std::env::var_os(OTEL_EXPORTER_OTLP_PROTOCOL).is_some()
    }
}
