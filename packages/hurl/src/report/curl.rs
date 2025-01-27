/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use crate::report::ReportError;
use crate::runner::HurlResult;
use crate::util::redacted::Redact;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Creates a curl export from a list of `hurl_results`.
///
/// `secrets` strings are redacted from this export.
pub fn write_curl(
    hurl_results: &[&HurlResult],
    filename: &Path,
    secrets: &[&str],
) -> Result<(), ReportError> {
    // We ensure that parent folder is created.
    if let Some(parent) = filename.parent() {
        match std::fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(err) => return Err(ReportError::from_error(err, filename, "Issue curl export")),
        }
    }
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .append(false)
        .open(filename)?;
    let mut cmds = hurl_results
        .iter()
        .flat_map(|h| &h.entries)
        .map(|e| e.curl_cmd.to_string().redact(secrets))
        .collect::<Vec<_>>()
        .join("\n");
    cmds.push('\n');
    file.write_all(cmds.as_bytes())?;

    Ok(())
}
