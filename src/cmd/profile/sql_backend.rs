//! Polars-SQL backend for `qsv profile`'s SQL-requiring formula helpers.
//!
//! DP+'s `jinja2_helpers.py` ships two helpers — `temporal_resolution`
//! and `guess_accrual_periodicity` — that need to fetch all distinct
//! values of a date column. In a CKAN deployment they hit
//! `datastore_search_sql`. In qsv there is no CKAN datastore; instead
//! we read the input CSV via Polars' `LazyCsvReader`, register it as a
//! SQL table named `data`, and run the same kind of query directly
//! against the file.
//!
//! Each call rebuilds a fresh `LazyFrame` + `SQLContext`. This is fine
//! for the qsv profile use case (helpers are invoked at most once per
//! formula in a scheming spec) and keeps the backend `Send + Sync` so
//! it can be cloned across minijinja closures via `Arc`.

use std::path::PathBuf;

use polars::{prelude::*, sql::SQLContext};

use crate::{CliError, CliResult};

/// Owns the path to the input CSV plus the parsing options that the
/// rest of the `profile` pipeline applies (delimiter, header presence)
/// so SQL-backed helpers see the same columns the stats / frequency
/// passes did. Inexpensive to clone.
#[derive(Debug, Clone)]
pub struct SqlBackend {
    csv_path:    PathBuf,
    delimiter:   u8,
    has_header:  bool,
    all_strings: bool,
}

impl SqlBackend {
    /// Defaults match Polars' `LazyCsvReader` defaults — comma-separated
    /// with a header row. Use the `with_*` builders to override.
    pub fn new(csv_path: impl Into<PathBuf>) -> Self {
        Self {
            csv_path:    csv_path.into(),
            delimiter:   b',',
            has_header:  true,
            all_strings: false,
        }
    }

    /// Read every column as a string instead of inferring types, so
    /// values compare as the text the file holds (`001` != `1`).
    #[must_use]
    pub const fn with_all_strings(mut self) -> Self {
        self.all_strings = true;
        self
    }

    /// Override the field separator byte (e.g. `b'\t'` for TSV).
    #[must_use]
    pub const fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Set whether the first row carries column names. When `false`,
    /// Polars synthesizes `column_0`, `column_1`, … as field names —
    /// SQL queries must reference those generated names.
    #[must_use]
    pub const fn with_has_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }

    /// Run a SQL query against the input CSV. The CSV is registered as
    /// table `data`. Returns the collected `DataFrame`.
    pub fn query(&self, sql: &str) -> CliResult<DataFrame> {
        let path_str = self.csv_path.to_string_lossy();
        let plpath = PlRefPath::new(&*path_str);
        let mut reader = LazyCsvReader::new(plpath)
            .with_skip_lines(crate::config::sniffed_preamble_lines(&self.csv_path))
            .with_separator(self.delimiter)
            .with_has_header(self.has_header);
        if self.all_strings {
            // Schema inference over zero rows types every column String.
            reader = reader.with_infer_schema_length(Some(0));
        }
        let lf = reader
            .finish()
            .map_err(|e| CliError::Other(format!("SqlBackend: read CSV: {e}")))?;
        let mut ctx = SQLContext::new();
        ctx.register("data", lf);
        ctx.execute(sql)
            .and_then(LazyFrame::collect)
            .map_err(|e| CliError::Other(format!("SqlBackend: SQL execute: {e}")))
    }

    /// Fetch distinct, sorted, non-null values from `date_field` as
    /// strings. Used by both `temporal_resolution` and
    /// `guess_accrual_periodicity`. Casting to Utf8 in SQL keeps the
    /// caller agnostic to whether Polars inferred the column as Date,
    /// Datetime, or String.
    pub fn distinct_sorted_date_strings(&self, date_field: &str) -> CliResult<Vec<String>> {
        // Use ANSI SQL double-quote identifier escapes. We don't allow
        // arbitrary user SQL through this path — `date_field` comes from
        // the scheming YAML formula caller — but we still hard-fail on
        // an embedded double-quote to prevent identifier-escape attacks.
        if date_field.contains('"') {
            return Err(CliError::Other(format!(
                "SqlBackend: invalid date_field (contains \"): {date_field}"
            )));
        }
        let sql = format!(
            r#"SELECT DISTINCT CAST("{date_field}" AS VARCHAR) AS d FROM data WHERE "{date_field}" IS NOT NULL ORDER BY d"#
        );
        let df = self.query(&sql)?;
        let col = df
            .column("d")
            .map_err(|e| CliError::Other(format!("SqlBackend: missing result column: {e}")))?;
        let series = col
            .as_series()
            .ok_or_else(|| CliError::Other("SqlBackend: result column not a series".to_string()))?;
        let strs = series
            .str()
            .map_err(|e| CliError::Other(format!("SqlBackend: cast to string: {e}")))?;
        let mut out: Vec<String> = Vec::with_capacity(strs.len());
        for s in strs.iter().flatten() {
            out.push(s.to_string());
        }
        Ok(out)
    }

    /// Rows that exactly duplicate an earlier row (every field's text
    /// equal), in one streaming pass that keeps a 128-bit xxh3 hash per
    /// distinct row rather than the rows themselves: ~16-32 bytes a row
    /// where `SELECT DISTINCT *` held every full row (about 2.5x the
    /// file's size). Each field is hashed with its length, so `a,bc` and
    /// `ab,c` differ. Two distinct rows sharing a 128-bit hash would be
    /// counted as duplicates; at a billion rows the odds are ~1e-21.
    pub fn duplicate_row_count(&self) -> CliResult<u64> {
        use std::collections::HashSet;

        use xxhash_rust::xxh3::Xxh3;

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_header)
            .flexible(true)
            .from_path(&self.csv_path)
            .map_err(|e| CliError::Other(format!("SqlBackend: cannot read input: {e}")))?;
        let mut seen: HashSet<u128, foldhash::fast::RandomState> = HashSet::default();
        let mut record = csv::ByteRecord::new();
        let mut rows: u64 = 0;
        let mut hasher = Xxh3::new();
        while rdr
            .read_byte_record(&mut record)
            .map_err(|e| CliError::Other(format!("SqlBackend: cannot read row: {e}")))?
        {
            hasher.reset();
            for field in &record {
                hasher.update(&(field.len() as u64).to_le_bytes());
                hasher.update(field);
            }
            seen.insert(hasher.digest128());
            rows += 1;
        }
        Ok(rows - seen.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn write_csv(name: &str, body: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::Builder::new()
            .prefix(name)
            .suffix(".csv")
            .tempfile()
            .unwrap();
        f.write_all(body.as_bytes()).unwrap();
        f.flush().unwrap();
        f
    }

    #[test]
    fn distinct_sorted_date_strings_returns_sorted_unique_values() {
        let f = write_csv(
            "dates",
            "id,date\n1,2024-01-15\n2,2024-01-15\n3,2024-01-16\n4,2024-01-14\n5,\n",
        );
        let backend = SqlBackend::new(f.path());
        let dates = backend.distinct_sorted_date_strings("date").unwrap();
        assert_eq!(dates, vec!["2024-01-14", "2024-01-15", "2024-01-16"]);
    }

    #[test]
    fn distinct_sorted_date_strings_rejects_quote_in_field_name() {
        let f = write_csv("bad", "a,b\n1,2\n");
        let backend = SqlBackend::new(f.path());
        let err = backend
            .distinct_sorted_date_strings(r#"foo" UNION SELECT '"#)
            .unwrap_err();
        assert!(err.to_string().contains("invalid date_field"));
    }

    #[test]
    fn all_strings_keeps_textual_values_distinct() {
        let f = write_csv("txt", "z\n001\n1\n");
        let n = |b: &SqlBackend| {
            let df = b
                .query("SELECT COUNT(*) AS n FROM (SELECT DISTINCT * FROM data)")
                .unwrap();
            df.column("n").unwrap().get(0).unwrap().to_string()
        };
        // Inferred as integers, `001` and `1` collapse; as text they don't.
        assert_eq!(n(&SqlBackend::new(f.path())), "1");
        assert_eq!(n(&SqlBackend::new(f.path()).with_all_strings()), "2");
    }

    #[test]
    fn query_runs_arbitrary_sql() {
        let f = write_csv("q", "id,name\n1,a\n2,b\n3,c\n");
        let backend = SqlBackend::new(f.path());
        let df = backend.query("SELECT COUNT(*) AS n FROM data").unwrap();
        let n_col = df.column("n").unwrap();
        assert_eq!(n_col.len(), 1);
    }

    #[test]
    fn with_delimiter_handles_tsv() {
        let f = write_csv("tsv", "id\tdate\n1\t2024-01-01\n2\t2024-01-02\n");
        let backend = SqlBackend::new(f.path()).with_delimiter(b'\t');
        let dates = backend.distinct_sorted_date_strings("date").unwrap();
        assert_eq!(dates, vec!["2024-01-01", "2024-01-02"]);
    }

    #[test]
    fn with_has_header_false_uses_synthetic_names() {
        // No header row — Polars synthesizes column_0, column_1 (0-based since
        // pola-rs/polars#28146).
        let f = write_csv("hdrless", "1,2024-01-01\n2,2024-01-02\n3,2024-01-03\n");
        let backend = SqlBackend::new(f.path()).with_has_header(false);
        let dates = backend.distinct_sorted_date_strings("column_1").unwrap();
        assert_eq!(dates, vec!["2024-01-01", "2024-01-02", "2024-01-03"]);
    }

    #[test]
    fn duplicate_row_count_compares_each_fields_text() {
        let f = write_csv(
            "dups",
            "k,v\na,1\na,1\n\"a\",1\na,01\nab,c\na,bc\n,x\n\"\",x\n",
        );
        // `a,1` twice more (quoting does not change a value), and `"",x`
        // repeats `,x` (a quoted empty field is still empty). `a,01` is not
        // `a,1`, and `ab,c` is not `a,bc`: each field is hashed with its
        // length.
        assert_eq!(SqlBackend::new(f.path()).duplicate_row_count().unwrap(), 3);
    }

    #[test]
    fn duplicate_row_count_honors_delimiter_and_header() {
        // tab-separated, `a,b` and `"a,b"` are the same single field; read
        // as comma-separated they would be different rows
        let tsv = write_csv("dups_tsv", "k\na,b\n\"a,b\"\n");
        let backend = SqlBackend::new(tsv.path()).with_delimiter(b'\t');
        assert_eq!(backend.duplicate_row_count().unwrap(), 1);

        // without a header, the first row is data and is duplicated
        let raw = write_csv("dups_noheader", "a,1\na,1\nb,2\n");
        let with_header = SqlBackend::new(raw.path());
        assert_eq!(with_header.duplicate_row_count().unwrap(), 0);
        let no_header = SqlBackend::new(raw.path()).with_has_header(false);
        assert_eq!(no_header.duplicate_row_count().unwrap(), 1);
    }
}
