# Python equivalent of turnaround_time.luau for the qsv benchmarks (issue #1351).
# Used by the `py_turnaround` benchmarks to compare the Python interpreter against
# Luau on a non-trivial, per-row date-math task.
#
# Computes the turnaround time (in days) between "Closed Date" and "Created Date"
# for the NYC 311 benchmark data, skipping rows where either date is empty - the
# same logic as turnaround_time.luau.
#
# Exceptions are caught and turned into "" so a few malformed/edge-case rows do
# not abort the whole `py map` run (qsv fails the command if any row errors).
import datetime

# NYC 311 dates look like: 03/01/2023 12:00:00 AM
_FMT = "%m/%d/%Y %I:%M:%S %p"


def turnaround(created_date, closed_date):
    if not created_date or not closed_date:
        return ""
    try:
        start = datetime.datetime.strptime(created_date, _FMT)
        end = datetime.datetime.strptime(closed_date, _FMT)
        # fractional days, mirroring luau date:diff():spandays()
        return (end - start).total_seconds() / 86400.0
    except ValueError:
        return ""
