# encoding: utf-8
# flake8: noqa: E501
#
# ============================================================================
# VENDORED COPY -- DO NOT EDIT IN PLACE.
#
# Source: dathere/datapusher-plus @ main
#         ckanext/datapusher_plus/jinja2_helpers.py
#
# This file is reused by `qsv profile` (issue dathere/qsv#1705) so the same
# Jinja2 filters/globals DP+ uses to compute CKAN metadata (formula /
# suggestion_formula keys in scheming YAML specs) also work when running
# qsv standalone, without CKAN.
#
# The three CKAN imports below (``ckanext.datapusher_plus.config``,
# ``...datastore_utils``, and ``ckan.plugins.toolkit``) are resolved at
# runtime against the stub modules installed by ``qsv_ckan_stubs.install()``
# (see ``py_engine.rs``). The SQL-requiring helpers (``temporal_resolution``,
# ``guess_accrual_periodicity``) gracefully short-circuit to ``None`` outside
# a real CKAN environment because the stubs report no datastore index.
#
# To refresh: re-vendor from upstream, leave this header intact.
# ============================================================================

from __future__ import annotations

import logging
import os
from datetime import datetime
from collections import Counter
from typing import Any, Dict, Optional

from jinja2 import DictLoader, FileSystemBytecodeCache, pass_context
from jinja2.sandbox import SandboxedEnvironment

import ckanext.datapusher_plus.config as conf
import ckanext.datapusher_plus.datastore_utils as dsu
import ckan.plugins.toolkit as tk

log = logging.getLogger(__name__)
if not log.handlers:
    log.addHandler(logging.StreamHandler())

JINJA2_FILTERS = []
JINJA2_GLOBALS = []
USES_SQL = []


def _register_unique(registry: list, func):
    """Register ``func`` in ``registry``, replacing any existing entry that
    shares the same qualname.

    The decorators below run at import time; a naive ``append`` would
    accumulate duplicates on module reload (test isolation, hot reload, etc.)
    and bloat the Jinja env. Skipping on qualname-collision avoids duplicates
    but pins the registry to the *old* function object, so fixes in the
    reloaded module would silently never take effect. Replacing the existing
    entry instead gives idempotent behaviour AND honours reloads.
    """
    qualname = getattr(func, "__qualname__", func.__name__)
    for i, existing in enumerate(registry):
        if getattr(existing, "__qualname__", existing.__name__) == qualname:
            registry[i] = func
            return
    registry.append(func)


def jinja2_filter(func):
    """Decorator to register a function as a Jinja2 filter."""
    _register_unique(JINJA2_FILTERS, func)
    return func


def jinja2_global(func):
    """Decorator to register a function as a Jinja2 global."""
    _register_unique(JINJA2_GLOBALS, func)
    return func


def uses_sql(func):
    """Decorator to register a function as using SQL."""
    _register_unique(USES_SQL, func)
    return func


def detect_lat_lon_fields(resource_fields_stats):
    """Return ``(lat_field, lon_field)`` for a stats dict, or ``(None, None)``.

    A field is treated as a latitude / longitude candidate when its qsv
    ``stats.type`` is ``Float`` and its ``min``/``max`` fall within the
    valid WGS84 range ([-90,90] for latitude, [-180,180] for longitude).
    The candidate set comes from ``conf.LATITUDE_FIELDS`` /
    ``conf.LONGITUDE_FIELDS`` (comma-separated, case-insensitive); the
    first matching column wins. Either return value can be ``None`` if
    no candidate matched.

    Extracted from ``FormulaProcessor.__init__`` so non-formula stages
    (currently ``MetadataStage``, which persists ``dpp_spatial_extent``
    for CSVs with lat/lon columns) can run the same detection without
    constructing a full ``FormulaProcessor``.
    """
    latitude_fields = [f.strip() for f in conf.LATITUDE_FIELDS.split(",")]
    longitude_fields = [f.strip() for f in conf.LONGITUDE_FIELDS.split(",")]

    def _match(candidates, min_bound, max_bound):
        lowered = {k.lower(): k for k in resource_fields_stats.keys()}
        for field in candidates:
            orig = lowered.get(field.lower())
            if orig is None:
                continue
            stats = resource_fields_stats.get(orig, {}).get("stats", {})
            if stats.get("type") != "Float":
                continue
            try:
                lo = float(stats["min"])
                hi = float(stats["max"])
            except (KeyError, TypeError, ValueError):
                continue
            if lo >= min_bound and hi <= max_bound:
                return orig
        return None

    lat_field = _match(latitude_fields, -90.0, 90.0)
    lon_field = _match(longitude_fields, -180.0, 180.0)
    return lat_field, lon_field


class FormulaProcessor:
    def __init__(
        self,
        scheming_yaml,
        package,
        resource,
        resource_fields_stats,
        resource_fields_freqs,
        dataset_stats,
        logger,
    ):

        # FIRST, INFER LATITUDE AND LONGITUDE COLUMN NAMES
        logger.trace(f"Latitude Fields: {conf.LATITUDE_FIELDS}")
        logger.trace(f"Longitude Fields: {conf.LONGITUDE_FIELDS}")

        dpp = {}
        lat_field, lon_field = detect_lat_lon_fields(resource_fields_stats)
        dpp["LAT_FIELD"] = lat_field
        dpp["LON_FIELD"] = lon_field
        dpp["NO_LAT_LON_FIELDS"] = lat_field is None or lon_field is None

        # now, check if any date fields are present in the resource_fields_stats
        # if found, set the dpp["DATE_FIELDS"] to the field name
        dpp["DATE_FIELDS"] = []
        for field in resource_fields_stats.keys():
            if resource_fields_stats[field]["stats"]["type"] == "Date":
                dpp["DATE_FIELDS"].append(field)

        # if no date/datetime fields are found, set dpp["NO_DATE_FIELDS"] to True
        if not dpp["DATE_FIELDS"]:
            dpp["NO_DATE_FIELDS"] = True
        else:
            dpp["NO_DATE_FIELDS"] = False

        # now, check if any datetime fields are present in the resource_fields_stats
        # if found, set the dpp["DATETIME_FIELDS"] to the field name
        dpp["DATETIME_FIELDS"] = []
        for field in resource_fields_stats.keys():
            if resource_fields_stats[field]["stats"]["type"] == "DateTime":
                dpp["DATETIME_FIELDS"].append(field)

        # if no datetime fields are found, set dpp["NO_DATETIME"] to True
        if not dpp["DATETIME_FIELDS"]:
            dpp["NO_DATETIME_FIELDS"] = True
        else:
            dpp["NO_DATETIME_FIELDS"] = False

        # add dataset_stats to dpp
        dpp["dataset_stats"] = dataset_stats

        self.scheming_yaml = scheming_yaml
        self.package = package
        self.resource = resource
        self.resource_fields_stats = resource_fields_stats
        self.resource_fields_freqs = resource_fields_freqs
        self.dpp = dpp
        self.logger = logger

    def process_formulae(
        self, entity_type: str, fields_key: str, formula_type: str = "formula"
    ):
        """
        Generic formula processor for both package and resource fields

        Args:
            entity_type: 'package' or 'resource'
            fields_key: Key in scheming_yaml for fields ('dataset_fields' or 'resource_fields')
            formula_type: Type of formula ('formula' or 'suggestion_formula')
        """
        formula_fields = [
            field for field in self.scheming_yaml[fields_key] if field.get(formula_type)
        ]

        if not formula_fields:
            return

        self.logger.info(
            f"Found {len(formula_fields)} {entity_type.upper()} field/s with {formula_type} in the scheming_yaml"
        )

        jinja2_formulae = {}
        for schema_field in formula_fields:
            field_name = schema_field["field_name"]
            template = schema_field[formula_type]
            jinja2_formulae[field_name] = template

            self.logger.debug(
                f'Jinja2 {formula_type} for {entity_type.upper()} field "{field_name}": {template}'
            )

        context = {
            "package": self.package,
            "resource": self.resource,
            "dpps": self.resource_fields_stats,
            "dppf": self.resource_fields_freqs,
            "dpp": self.dpp,
        }
        self.logger.trace(f"Environment Context: {context}")
        jinja2_env = self.create_jinja2_env(jinja2_formulae)

        updates = {}
        for schema_field in formula_fields:
            field_name = schema_field["field_name"]
            try:
                formula = jinja2_env.get_template(field_name)
                rendered_formula = formula.render(**context)
                
                # Coerce Jinja2's stringified empty/none outputs back to
                # actual Python ``None`` so ``dpp_suggestions`` JSON serializes
                # ``null`` and the frontend can grey out the suggestion button
                # (issue #261). We ``.strip()`` first so whitespace-only renders
                # (the ``" "`` / ``"\n"`` output of ``{% if %}`` blocks missing
                # ``-`` trim markers) are caught too — roborev #2289 LOW.
                #
                # Gated on ``formula_type == "suggestion_formula"`` because
                # this method also handles direct ``formula`` updates (see
                # ``jobs/stages/formula.py:290,321``) that write straight into
                # the package/resource dict. Coercing ``""`` → ``None`` on
                # those would change patch semantics for fields where the
                # CKAN validator treats empty string and null differently
                # (caught by Copilot review on PR #322).
                #
                # KNOWN LIMITATION for the suggestion path: a formula that
                # legitimately renders the literal string ``"None"`` (e.g. a
                # license field) will also coerce to ``None``. Deliberate —
                # ``scheming-suggestions.js`` relies on ``null`` (not
                # ``"None"``) to distinguish "no suggestion" from a real
                # value. Scheming YAMLs that need to emit the literal text
                # "None" should pick a different label (``"none"`` lowercase,
                # ``"N/A"``, etc.).
                if (
                    formula_type == "suggestion_formula"
                    and rendered_formula.strip() in ("", "None")
                ):
                    rendered_formula = None

                updates[field_name] = rendered_formula

                self.logger.debug(
                    f'Evaluated jinja2 {formula_type} for {entity_type.upper()} field "{field_name}": {rendered_formula}'
                )
            except Exception as e:
                formula_error_msg = f'#ERROR!: {formula_type} for {entity_type.upper()} field "{field_name}": {str(e)}'
                self.logger.error(formula_error_msg)
                updates[field_name] = formula_error_msg

        return updates

    def create_jinja2_env(self, context: Dict[str, Any]) -> SandboxedEnvironment:
        """Create a configured Jinja2 environment with all filters and globals.

        Uses ``SandboxedEnvironment`` because formula templates originate from
        scheming YAML authored by sysadmins/schema editors. Without the sandbox,
        a template like ``{{ ''.__class__.__mro__[1].__subclasses__() }}`` can
        escape to ``subprocess``, the filesystem, etc. — i.e. RCE on the worker.
        """

        # We use a bytecode cache to speed up the rendering of the Formulas
        # we do not use temp_dir defined in jobs.py because we want the cache
        # to be persistent across datapusher runs
        cache_dir = conf.JINJA2_BYTECODE_CACHE_DIR
        os.makedirs(cache_dir, exist_ok=True)
        bytecode_cache = FileSystemBytecodeCache(cache_dir)

        env = SandboxedEnvironment(
            loader=DictLoader(context), bytecode_cache=bytecode_cache
        )

        datastore_sqlsearch_enabled = tk.config.get(
            "ckan.datastore.sqlsearch.enabled", False
        )

        # Register filters
        for func in JINJA2_FILTERS:
            if func in USES_SQL and not datastore_sqlsearch_enabled:
                self.logger.warning(
                    f"Datastore SQL Search is disabled, skipping filter {func.__name__}"
                )
            else:
                env.filters[func.__name__] = func
        # Register globals
        for func in JINJA2_GLOBALS:
            if func in USES_SQL and not datastore_sqlsearch_enabled:
                self.logger.warning(
                    f"Datastore SQL Search is disabled, skipping global {func.__name__}"
                )
            else:
                env.globals[func.__name__] = func
        return env


# ------------------
# DP+ Jinja2 filters & functions
# that can be used in scheming formulas
@jinja2_filter
def truncate_with_ellipsis(text, length=50, ellipsis="..."):
    """Truncate text to a specific length and append ellipsis.

    Example:
    {{ package.description | truncate_with_ellipsis(10) }} -> "Hello, wo..."
    """
    if not text or len(text) <= length:
        return text
    return text[:length] + ellipsis


@jinja2_filter
def format_number(value, decimals=2):
    """Format numbers with thousands separator and decimal places

    Example:
    {{ dpps.population.stats.sum | format_number }} -> 1,234,567.89
    """
    try:
        return f"{float(value):,.{decimals}f}"
    except (TypeError, ValueError):
        return value  # Return as-is if not a number


@jinja2_filter
def format_bytes(bytes):
    """Format byte sizes into human readable format

    Example:
    {{ dpp.ORIGINAL_FILE_SIZE | format_bytes }} -> 1.5 GB
    """
    try:
        bytes = float(bytes)
    except (TypeError, ValueError):
        return bytes  # Return as-is if not a number

    for unit in ["B", "KB", "MB", "GB", "TB"]:
        if bytes < 1024:
            return f"{bytes:.1f} {unit}"
        bytes /= 1024
    return f"{bytes:.1f} PB"  # For very large numbers


@jinja2_filter
def format_date(value, format="%Y-%m-%d"):
    """Format dates in specified format. This function takes a date value and formats it according to the specified format string.

    The function handles three cases:
    1. If the input is None, returns None
    2. If the input is already a datetime object, formats it directly using strftime()
    3. If the input is a string, attempts to parse it as an ISO format date first

    If parsing fails in case 3, returns the original value unchanged.

    Args:
        value: A datetime object, ISO format date string, or None
        format: A strftime format string (default: "%Y-%m-%d")

    Returns:
        str: The formatted date string
        None: If input was None
        original value: If parsing failed

    Example:
    {{ dpps.created_date.stats.max | format_date("%B %d, %Y") }} -> January 1, 2024
    """
    if value is None:
        return None
    try:
        # value is already a datetime object
        return value.strftime(format)
    except AttributeError:
        # Try to parse string to date
        try:
            dt = datetime.fromisoformat(value)
            return dt.strftime(format)
        except Exception:
            return value  # Return as-is if parsing fails


@jinja2_filter
def calculate_percentage(part, whole):
    """Calculate percentage

    Example:
    {{ calculate_percentage(dpps.id.stats.nullcount, dpp.dataset_stats.RECORD_COUNT) }} -> 12.5
    """
    try:
        part = float(part)
        whole = float(whole)
        if whole == 0:
            raise ValueError("Whole value is zero")
        else:
            return (part / whole) * 100
    except (TypeError, ValueError, ZeroDivisionError):
        raise ValueError("Error calculating percentage")


@jinja2_filter
def format_range(min_val, max_val, separator=" to "):
    """Format a range of values

    Example:
    {{ format_range(dpps.temperature.stats.min, dpps.temperature.stats.max) }} -> "-10 to 35"
    """
    return f"{min_val}{separator}{max_val}"


@jinja2_filter
def format_coordinates(lat, lon, precision=6):
    """Format coordinates nicely

    Example:
    {{ format_coordinates(dpps.latitude.stats.mean, dpps.longitude.stats.mean) }}
    -> "40.7128°N, 74.0060°W"
    """
    lat_dir = "N" if lat >= 0 else "S"
    lon_dir = "E" if lon >= 0 else "W"
    return f"{abs(lat):.{precision}f}°{lat_dir}, {abs(lon):.{precision}f}°{lon_dir}"


# ------------------
# Jinja2 Global Functions
# that can be used in scheming formulas
@jinja2_global
@pass_context
def calculate_bbox_area(
    context: dict,
    min_lon: float = None,
    min_lat: float = None,
    max_lon: float = None,
    max_lat: float = None,
) -> Optional[float]:
    """Calculate approximate area of bounding box in square kilometers

    Args:
        context: The context of the template
                 (automatically passed by Jinja2 using @pass_context decorator)
        min_lon: Minimum longitude coordinate
        min_lat: Minimum latitude coordinate
        max_lon: Maximum longitude coordinate
        max_lat: Maximum latitude coordinate

    If the min/max coordinates are not provided, the spatial extent of the resource will be used if available
    Otherwise, the min/max coordinates will be calculated from the latitude and longitude fields

    Returns:
        float: Area of the bounding box in square kilometers

    Example:
    {{ calculate_bbox_area(dpp.spatial_extent.min_lon, dpp.spatial_extent.min_lat, dpp.spatial_extent.max_lon, dpp.spatial_extent.max_lat) }}
    -> 1234.56
    """
    from math import radians, cos, pi

    earth_radius = 6371  # km

    bbox = context.get("resource").get("dpp_spatial_extent")
    if bbox:
        # get the min/max coordinates from the spatial extent
        # which is in BoundingBox format
        if bbox.get("type") != "BoundingBox":
            # validate the spatial extent is a BoundingBox first
            raise ValueError("Spatial extent is not a BoundingBox")
        else:
            bbox_coords = bbox.get("coordinates")
            min_lon = bbox_coords[0][0]
            min_lat = bbox_coords[0][1]
            max_lon = bbox_coords[1][0]
            max_lat = bbox_coords[1][1]
    elif context.get("dpp").get("NO_LAT_LON_FIELDS"):
        raise ValueError("No latitude or longitude fields found")
    else:
        lon_field = context.get("dpp").get("LON_FIELD")
        lat_field = context.get("dpp").get("LAT_FIELD")
        min_lon = context.get("dpps").get(lon_field).get("stats").get("min")
        min_lat = context.get("dpps").get(lat_field).get("stats").get("min")
        max_lon = context.get("dpps").get(lon_field).get("stats").get("max")
        max_lat = context.get("dpps").get(lat_field).get("stats").get("max")

    # Convert degree differences to radians for accurate area calculation
    width = abs(max_lon - min_lon) * (pi / 180) * cos(radians((min_lat + max_lat) / 2))
    height = abs(max_lat - min_lat) * (pi / 180)
    return width * height * (earth_radius**2)


@jinja2_global
@pass_context
def spatial_extent_wkt(
    context: dict,
    min_lon: float = None,
    min_lat: float = None,
    max_lon: float = None,
    max_lat: float = None,
) -> Optional[str]:
    """Convert min/max WGS84 coordinates to WKT polygon format.

    Args:
        context: The context of the template
                 (automatically passed by Jinja2 using @pass_context decorator)
        min_lon: Minimum longitude coordinate
        min_lat: Minimum latitude coordinate
        max_lon: Maximum longitude coordinate
        max_lat: Maximum latitude coordinate

    If the min/max coordinates are not provided, the spatial extent of the resource will be used if available
    Otherwise, the min/max coordinates will be calculated from the latitude and longitude fields

    Returns:
        str: WKT polygon string representing the spatial extent

    Example:
        >>> spatial_extent_wkt(-180, -90, 180, 90)
        'POLYGON((-180 -90, -180 90, 180 90, 180 -90, -180 -90))'
        >>> spatial_extent_wkt()
        'POLYGON((-180 -90, -180 90, 180 90, 180 -90, -180 -90))'
    """
    bbox = context.get("resource").get("dpp_spatial_extent")
    if bbox:
        # get the min/max coordinates from the spatial extent
        # which is in BoundingBox format
        if bbox.get("type") != "BoundingBox":
            # validate the spatial extent is a BoundingBox first
            raise ValueError("Spatial extent is not a BoundingBox")
        else:
            bbox_coords = bbox.get("coordinates")
            min_lon = bbox_coords[0][0]
            min_lat = bbox_coords[0][1]
            max_lon = bbox_coords[1][0]
            max_lat = bbox_coords[1][1]
    elif context.get("dpp").get("NO_LAT_LON_FIELDS"):
        raise ValueError("No latitude or longitude fields found")
    else:
        lon_field = context.get("dpp").get("LON_FIELD")
        lat_field = context.get("dpp").get("LAT_FIELD")
        min_lon = context.get("dpps").get(lon_field).get("stats").get("min")
        min_lat = context.get("dpps").get(lat_field).get("stats").get("min")
        max_lon = context.get("dpps").get(lon_field).get("stats").get("max")
        max_lat = context.get("dpps").get(lat_field).get("stats").get("max")
    # Create WKT polygon string from coordinates
    wkt = f"SRID=4326;POLYGON(({min_lon} {min_lat}, {min_lon} {max_lat}, {max_lon} {max_lat}, {max_lon} {min_lat}, {min_lon} {min_lat}))"
    return wkt


@jinja2_global
@pass_context
def spatial_extent_feature_collection(
    context: dict,
    name: str = "Inferred Spatial Extent",
    bbox: list[float] = None,
    feature_type: str = "inferred",
) -> str:
    """Convert a bounding box to a namedGeoJSON feature collection.

    Args:
        name: Name of the feature
        bbox: List of floats representing the bounding box [min_lon, min_lat, max_lon, max_lat]
              If the bbox is not provided, the spatial extent of the resource will be used if available
              Otherwise, the bbox will be calculated from the latitude and longitude fields
        feature_type: Type of the feature, defaults to "inferred"

    Returns:
        str: GeoJSON feature collection string

    Example:
        >>> spatial_extent_feature_collection()
        '{"type": "FeatureCollection", "features": [{"type": "Feature", "properties":{"name":"Inferred Spatial Extent","type":"inferred"}, "geometry": {"type": "Polygon", "coordinates": [[[-180, -90], [-180, 90], [180, 90], [180, -90], [-180, -90]]]}, "properties": {}}]}
        >>> spatial_extent_feature_collection("User Provided Bounding Box", [-180, -90, 180, 90])
        '{"type": "FeatureCollection", "features": [{"type": "Feature", "properties":{"name":"User Provided Bounding Box","type":"calculated"}, "geometry": {"type": "Polygon", "coordinates": [[[-180, -90], [-180, 90], [180, 90], [180, -90], [-180, -90]]]}, "properties": {}}]}
        >>> spatial_extent_feature_collection("Custom Name")
        '{"type": "FeatureCollection", "features": [{"type": "Feature", "properties":{"name":"Custom Name","type":"inferred"}, "geometry": {"type": "Polygon", "coordinates": [[[-180, -90], [-180, 90], [180, 90], [180, -90], [-180, -90]]]}, "properties": {}}]}
    """
    if bbox:
        feature_type = "calculated"
        # validate bbox
        if len(bbox) != 4:
            raise ValueError("Invalid bounding box")
        else:
            bbox = [float(coord) for coord in bbox]
    else:
        extent = context.get("resource").get("dpp_spatial_extent")
        if extent:
            # ``dpp_spatial_extent`` is persisted as a GeoJSON-ish
            # BoundingBox: ``{"type": "BoundingBox", "coordinates":
            # [[min_lon, min_lat], [max_lon, max_lat]]}``. Flatten to
            # the [min_lon, min_lat, max_lon, max_lat] order this
            # function's format-string expects. Same shape that
            # ``spatial_extent_wkt`` consumes.
            if extent.get("type") == "BoundingBox":
                coords = extent.get("coordinates")
                bbox = [
                    float(coords[0][0]),
                    float(coords[0][1]),
                    float(coords[1][0]),
                    float(coords[1][1]),
                ]
            else:
                raise ValueError("Spatial extent is not a BoundingBox")
        else:
            if context.get("dpp").get("NO_LAT_LON_FIELDS"):
                raise ValueError("No latitude or longitude fields found")
            else:
                lat_field = context.get("dpp").get("LAT_FIELD")
                lon_field = context.get("dpp").get("LON_FIELD")
                if lat_field and lon_field:
                    bbox = [
                        context.get("dpps").get(lon_field).get("stats").get("min"),
                        context.get("dpps").get(lat_field).get("stats").get("min"),
                        context.get("dpps").get(lon_field).get("stats").get("max"),
                        context.get("dpps").get(lat_field).get("stats").get("max"),
                    ]
                else:
                    raise ValueError("No latitude or longitude fields found")

    return f'{{"type": "FeatureCollection", "features": [{{"type": "Feature", "properties": {{"name": "{name}", "type": "{feature_type}"}}, "geometry": {{"type": "Polygon", "coordinates": [[[{bbox[0]},{bbox[1]}], [{bbox[0]},{bbox[3]}], [{bbox[2]},{bbox[3]}], [{bbox[2]},{bbox[1]}], [{bbox[0]},{bbox[1]}]]]}}}}]}}'


@jinja2_global
@pass_context
def get_frequency_top_values(
    context: dict,
    field: str,
) -> list[dict]:
    """Get the top values for a field from the frequency data.

    Args:
        context: The context of the template
                 (automatically passed by Jinja2 using @pass_context decorator)
        field: The field to get the top values for

    Returns:
        List of dictionaries containing value, count and percentage for top values

    Example:
    {{ get_frequency_top_values("field_name") }} -> [{'value': '<ALL_UNIQUE>', 'count': 1000000, 'percentage': 100.0}]
    """
    dppf = context.get("dppf")

    if not dppf:
        raise ValueError("No frequency data found")

    if field not in dppf:
        raise ValueError("Field not found in frequency data")

    # The data is already sorted by frequency in descending order from qsv frequency
    return dppf[field]


@jinja2_global
@uses_sql
@pass_context
def temporal_resolution(context, date_field=None):
    """Compute the minimum interval between sorted unique dates in a date field.

    This function analyzes a date field in a resource to determine the smallest time interval
    between consecutive dates. It returns the interval in ISO 8601 duration format.

    NOTE: This function uses the CKAN DataStore SQL API to fetch all the unique dates for
    the given date field. This is OK even for large datasets as DP+ automatically adds an
    index on date fields BY DEFAULT (configurable with AUTO_INDEX_DATES).
    If AUTO_INDEX_DATES is set to False, this function will log a warning to alert
    the user that the resource will be slow to process.

    Args:
        context (dict): The Jinja2 template context containing resource info and stats
        date_field (str, optional): Name of the date field to analyze. If not provided,
            uses the first date field found in the resource.

    Returns:
        str: ISO 8601 duration string representing the minimum interval between dates:
            - "PT1H" for sub-daily intervals
            - "P1D" for daily intervals
            - "PnD" for n-day intervals up to 31 days
            - "PnM" for n-month intervals up to 12 months
            - "PnY" for n-year intervals
            - None if no valid intervals found

    Raises:
        ValueError: If no date fields found, not enough records, or error processing dates

    Example:
        {{ temporal_resolution("date_column") }} -> "P1D" for daily data
    """
    dpp = context.get("dpp", {})
    resource = context.get("resource", {})

    if not date_field:
        date_fields = dpp.get("DATE_FIELDS", [])
        datetime_fields = dpp.get("DATETIME_FIELDS", [])
        if datetime_fields:
            date_fields.extend(datetime_fields)
        if not date_fields:
            raise ValueError("No date or datetime fields found")
        # if there are DATE_FIELDS, but no date_field is provided,
        # use the first one
        date_field = date_fields[0]

    # Get unique values for the date field from the DataStore
    resource_id = resource.get("id")
    if not resource_id:
        raise ValueError("No resource ID found")

    # check if the resource has an index on the date field
    # if not, log a warning
    if not dsu.index_exists(resource_id, date_field):
        log = context.get("logger", logging.getLogger(__name__))
        log.warning(
            f"Resource {resource_id} does not have an index on the date field {date_field}. This will slow down the temporal resolution calculation."
        )

    sql = f'SELECT DISTINCT "{date_field}" FROM "{resource_id}" WHERE "{date_field}" IS NOT NULL ORDER BY "{date_field}"'
    try:
        records = dsu.datastore_search_sql(sql)
        values = [
            r[date_field] for r in records.get("records", []) if r.get(date_field)
        ]
        if len(values) < 2:
            raise ValueError("Not enough records to get temporal resolution")
    except Exception as e:
        raise ValueError(f"Error getting temporal resolution: {e}")

    # Parse and sort dates
    try:
        dates = [datetime.fromisoformat(v) for v in values if v]
        intervals = [(dates[i + 1] - dates[i]).days for i in range(len(dates) - 1)]
        if not intervals:
            return None
        min_days = min(intervals)
        if min_days < 1:
            return "PT1H"  # fallback for sub-daily
        elif min_days == 1:
            return "P1D"
        elif min_days <= 31:
            return f"P{min_days}D"
        elif min_days <= 366:
            return f"P{min_days//30}M"
        else:
            return f"P{min_days//365}Y"
    except Exception:
        raise ValueError("Error getting temporal resolution")


@jinja2_global
@uses_sql
@pass_context
def guess_accrual_periodicity(context, date_field=None):
    """Guess the accrual periodicity of a dataset based on date intervals.

    Analyzes the intervals between dates in a date field to determine how frequently
    the dataset is updated. Returns an ISO 8601 duration pattern with repeating interval.

    NOTE: This function uses the CKAN DataStore SQL API to fetch all the unique dates for
    the given date field. This is OK even for large datasets as DP+ automatically adds an
    index on date fields BY DEFAULT (configurable with AUTO_INDEX_DATES).
    If AUTO_INDEX_DATES is set to False, this function will log a warning to alert
    the user that the resource will be slow to process.

    Args:
        context (dict): Context dictionary containing resource and field information
        date_field (str, optional): Name of the date field to analyze. If not provided,
            will use the first detected date field.

    Returns:
        str: ISO 8601 repeating duration pattern, e.g.:
            - "R/P1D" for daily
            - "R/P7D" for weekly
            - "R/P1M" for monthly
            - "R/P1Y" for yearly

    Raises:
        ValueError: If no date fields found, not enough records to analyze,
            or other errors occur during processing.

    Example:
        {{ guess_accrual_periodicity("updated_date") }} -> "R/P1D"
    """
    dpp = context.get("dpp", {})
    if not date_field:
        date_fields = dpp.get("DATE_FIELDS", [])
        datetime_fields = dpp.get("DATETIME_FIELDS", [])
        if datetime_fields:
            date_fields.extend(datetime_fields)
        if not date_fields:
            raise ValueError("No date or datetime fields found")
        # if there are DATE_FIELDS, but no date_field is provided,
        # use the first one
        date_field = date_fields[0]

    resource_id = context.get("resource", {}).get("id")
    if not resource_id:
        raise ValueError("No resource ID found")

    # check if the resource has an index on the date field
    # if not, log a warning
    if not dsu.index_exists(resource_id, date_field):
        log = context.get("logger", logging.getLogger(__name__))
        log.warning(
            f"Resource {resource_id} does not have an index on the date field {date_field}. This will slow down the accrual periodicity calculation."
        )

    try:
        sql = f'SELECT DISTINCT "{date_field}" FROM "{resource_id}" WHERE "{date_field}" IS NOT NULL ORDER BY "{date_field}"'
        try:
            records = dsu.datastore_search_sql(sql)
        except Exception as e:
            raise ValueError(f"Error getting accrual periodicity: {e}")
        values = [
            r[date_field] for r in records.get("records", []) if r.get(date_field)
        ]
        if len(values) < 2:
            raise ValueError("Not enough records to guess accrual periodicity")
        dates = [datetime.fromisoformat(v) for v in values if v]
        intervals = [(dates[i + 1] - dates[i]).days for i in range(len(dates) - 1)]
        if not intervals:
            raise ValueError("No intervals found")
        # Use the most common interval
        most_common = Counter(intervals).most_common(1)[0][0]
        if most_common == 1:
            return "R/P1D"
        elif most_common <= 31:
            return f"R/P{most_common}D"
        elif most_common <= 366:
            return f"R/P{most_common//30}M"
        else:
            return f"R/P{most_common//365}Y"
    except Exception:
        raise ValueError("Error guessing accrual periodicity")


@jinja2_global
@pass_context
def map_tags_to_themes(context):
    """
    Map CKAN tags to DCAT theme URIs using a lookup table.
    """
    # Example mapping, extend as needed
    # we can possibly point to a reference resource in CKAN
    # or point to a remote JSON resource
    # TODO: add more mappings ...
    tag_to_theme = {
        "climate": "https://data.gov/themes/climate",
        "health": "https://data.gov/themes/health",
        "transportation": "https://data.gov/themes/transportation",
    }
    tags = context.get("package", {}).get("tags", [])
    if isinstance(tags, str):
        tags = [t.strip() for t in tags.split(",")]
    themes = [
        tag_to_theme.get(tag.lower()) for tag in tags if tag.lower() in tag_to_theme
    ]
    return themes or None


@jinja2_global
@pass_context
def spatial_resolution_in_meters(context, bbox=None):
    """
    Compute the diagonal of the bounding box in meters.

    Args:
        context: The template context containing spatial data
        bbox: Optional BoundingBox tuple (min_lat, max_lat, min_lon, max_lon)
              If not provided, will try to get coordinates from context
    """
    from math import radians, cos, sin, sqrt, asin

    if bbox:
        min_lat, max_lat, min_lon, max_lon = bbox
    else:
        dpp = context.get("dpp", {})
        dpps = context.get("dpps", {})
        if dpp.get("NO_LAT_LON_FIELDS"):
            raise ValueError("No latitude or longitude fields found")
        lat_field = dpp.get("LAT_FIELD")
        lon_field = dpp.get("LON_FIELD")
        if not lat_field or not lon_field:
            raise ValueError("No latitude or longitude fields found")
        min_lat = dpps[lat_field]["stats"]["min"]
        max_lat = dpps[lat_field]["stats"]["max"]
        min_lon = dpps[lon_field]["stats"]["min"]
        max_lon = dpps[lon_field]["stats"]["max"]

    # Haversine formula for diagonal
    R = 6371000  # meters
    phi1, phi2 = radians(float(min_lat)), radians(float(max_lat))
    dphi = radians(float(max_lat) - float(min_lat))
    dlambda = radians(float(max_lon) - float(min_lon))
    a = sin(dphi / 2) ** 2 + cos(phi1) * cos(phi2) * sin(dlambda / 2) ** 2
    c = 2 * asin(sqrt(a))
    d = R * c
    return d


@jinja2_global
@pass_context
def get_column_null_percentage(context, column_name):
    """
    Compute the percentage of nulls in a field.
    Usage: {{ get_column_null_percentage("column_name") }}
    """
    if not column_name:
        raise ValueError("Column name is required")
    field_stats = context.get("dpps", {}).get(column_name, {}).get("stats", {})
    nullcount = field_stats.get("nullcount", 0)
    record_count = context.get("dpp", {}).get("RECORD_COUNT", 0)
    if record_count == 0:
        return 0
    else:
        return (nullcount / record_count) * 100


@jinja2_global
@pass_context
def get_column_stats(context, column_name, stat_name=None):
    """Get statistics for a column in the datastore.
    This function retrieves statistics for a specified column from the datastore stats.
    Available statistics typically include: min, max, sum, mean, stddev, variance,
    nullcount, cardinality, and type.

    Args:
        context: The context of the template
                 (automatically passed by Jinja2 using @pass_context decorator)
        column_name (str): Name of the column of the current resource to get stats for
        stat_name (str|list, optional): Specific stat(s) to return. Can be:
            - None: Returns all available stats (default)
            - str: Returns value for a single stat
            - list: Returns dict of values for specified stats

    Returns:
        dict|float: Statistics for the column. Returns:
            - All stats as dict if stat_name is None
            - Single stat value if stat_name is string
            - Dict of requested stats if stat_name is list
            - 0 if stat not found

    Examples:
        Get all stats:
        {{ get_column_stats("temperature") }}

        Get specific stats:
        {{ get_column_stats("temperature", ["min", "max"]) }}

        Get single stat:
        {{ get_column_stats("temperature", "avg") }}
    """
    if not column_name:
        raise ValueError("Column name is required")

    field_stats = context.get("dpps", {}).get(column_name, {}).get("stats", {})
    if not field_stats:
        raise ValueError(f"No stats found for column {column_name}")
    if stat_name:
        if isinstance(stat_name, list):
            return {k: field_stats.get(k, 0) for k in stat_name}
        else:
            return field_stats.get(stat_name, 0)
    else:
        return field_stats
