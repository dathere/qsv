# Python equivalent of dt_format.luau for the qsv benchmarks (issue #1351).
# Used by the `py_dateformat` benchmark to compare the Python interpreter against
# Luau on a non-trivial, per-row date-wrangling task.
#
# Unlike luau map (which can emit multiple columns in one pass), `py map` emits a
# single column, so this returns the day-of-week - the heaviest part of the luau
# date-format work (parse "Created Date", then format).
#
# Exceptions are caught and turned into "" so a few malformed/edge-case rows do
# not abort the whole `py map` run (qsv fails the command if any row errors).
import datetime

# NYC 311 "Created Date" looks like: 03/01/2023 12:00:00 AM
_FMT = "%m/%d/%Y %I:%M:%S %p"


def dtformat(created_date):
    if not created_date:
        return ""
    try:
        return datetime.datetime.strptime(created_date, _FMT).strftime("%a")
    except ValueError:
        return ""
