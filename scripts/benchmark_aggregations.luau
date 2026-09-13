-- version 1.2.0
-- this luau script is used by benchmarks.sh to compute records per second (recs_per_sec)
-- for each benchmark. It does this by retrieving the rowcount from the environment var 
-- QSVBM_ROWCOUNT (which is set by benchmarks.sh by running "qsv count") and dividing it 
-- by the mean value of the elapsed time for for each benchmark's runs (default: 3 runs).
--
-- A few benchmarks read a SUBSET of the benchmark data rather than the full dataset, so
-- dividing by the full rowcount would overstate their throughput (qsv issue #4598).
-- For those, benchmarks.sh exports QSVBM_ROWCOUNTS, a flat "name=rows;name=rows;" string
-- keyed on the benchmark name - which is already a column in the results CSV, so no new
-- column is needed. Benchmarks absent from that map use the full rowcount as before.
-- It also computes the total mean in the END block, which is returned to stderr.
-- The total mean is reported by benchmarks.sh at the end of each run. 
-- The total mean gives an idea of how long the benchmark suite took to run, 
-- regardless of the number of runs per benchmark.

-- this is the BEGIN block, which is executed once before the main loop
BEGIN {
    -- we initialize the variables here
    total_mean = 0;
    rounded_mean = 0;
    benchmark_data_rowcount=tonumber(qsv_getenv("QSVBM_ROWCOUNT"));

    -- parse the per-benchmark rowcount overrides into a lookup table keyed by benchmark
    -- name. NOTE: every CSV header is set as a luau global on EVERY row, so this table
    -- must NOT be named after any column in the results CSV
    -- (version,tstamp,name,mean,stddev,median,user,system,min,max).
    -- This parser must be incapable of raising: qsv luau fails the whole run on any
    -- error, and benchmarks.sh captures this script's stderr as total_mean.
    -- qsv_getenv returns "" (never nil) when unset, so an unset/empty/malformed value
    -- just yields an empty table and every row falls back to benchmark_data_rowcount.
    bm_rowcounts = {};
    local bm_rowcounts_spec = qsv_getenv("QSVBM_ROWCOUNTS");
    for bm_key, bm_val in string.gmatch(bm_rowcounts_spec, "([^;=]+)=([^;=]+)") do
        local bm_num = tonumber(bm_val);
        if bm_num and bm_num > 0 then
            bm_rowcounts[bm_key] = bm_num;
        end
    end

    -- and setup helper functions
    -- this one is to round a number to a given number of decimal places
    function round(num, numDecimalPlaces)
        return string.format("%." .. (numDecimalPlaces or 0) .. "f", num)
    end
}!

-- this is the main loop, which is executed for each row in the CSV
-- "mean" is the name of one of the columns in the CSV. That's right,
-- we can "magically" access the columns in the CSV directly as luau variables!
-- for each row, the luau var "mean" is set to the current value of the column "mean"
-- and the column "recs_per_sec" for that row is "map"ped to the value returned by the main loop
--
-- we round the mean to 3 decimal places and return it as a string
-- we do so as we display the rounded mean in the output
-- and computing recs/sec with the unrounded mean would show slightly 
-- different results for the same rounded_mean
rounded_mean = round(mean, 3);
-- for clarity, instead of this "magic" parsing of CSV data,
-- we could have also written:
--    rounded_mean = round(col.mean , 3); 
-- or
--    rounded_mean = round(col['mean'] , 3);
-- or using a 1-based index, where the first column is 1:
--    rounded_mean = round(col[4] , 3);
-- this is especially useful if the column has embedded spaces or special characters
-- or if the column name is a luau keyword (like "end" or "function")

-- use this benchmark's own rowcount if it reads a subset of the benchmark data,
-- otherwise use the full benchmark data rowcount.
-- reading a table with a nil key returns nil in luau (only WRITING a nil key raises),
-- so this lookup cannot error even if "name" is somehow unset.
bm_rowcount = bm_rowcounts[name];
if not bm_rowcount then
    bm_rowcount = benchmark_data_rowcount;
end

work_var = bm_rowcount / rounded_mean; 
total_mean = total_mean + rounded_mean; 

recs_per_sec=string.format("%.0f",work_var);
return recs_per_sec;

-- this is the end block, which is executed once after the main loop
END {
    -- we return the total_mean here
    -- note that this value is returned to stderr
    return round(total_mean, 3);
}!
