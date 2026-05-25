# Stubs for the CKAN-side modules that DP+'s ``jinja2_helpers.py`` imports.
#
# The vendored ``jinja2_helpers.py`` in this directory is kept byte-for-byte
# close to the upstream copy at dathere/datapusher-plus@main so we can pick up
# upstream fixes via re-vendoring. The three CKAN imports it does at module
# load time -- ``ckanext.datapusher_plus.config``, ``...datastore_utils``, and
# ``ckan.plugins.toolkit`` -- are not available outside a CKAN environment and
# qsv does not need their full surface to evaluate the pure-Python helpers.
#
# This module is installed into ``sys.modules`` *before* ``jinja2_helpers`` is
# imported by the Rust side (``py_engine.rs``). Anything not stubbed here will
# surface as an ``AttributeError`` at evaluation time, which is the intended
# escape hatch: the SQL-requiring helpers (``temporal_resolution``,
# ``guess_accrual_periodicity``) need a CKAN datastore and are out of scope
# for ``qsv profile``.

from __future__ import annotations

import sys
import types
from typing import Any, Optional

# ---------------------------------------------------------------------------
# ckanext.datapusher_plus.config
# ---------------------------------------------------------------------------

# Defaults pulled from DP+'s config module. We only need the constants that
# ``jinja2_helpers`` reads at module-import time or that ``detect_lat_lon_fields``
# reads at call time.
LATITUDE_FIELDS = "lat,latitude,y,ycoord,y_coord"
LONGITUDE_FIELDS = "lon,lng,long,longitude,x,xcoord,x_coord"
JINJA2_BYTECODE_CACHE_DIR: Optional[str] = None
DATE_FIELDS = "date,due,open,close,created"
DATETIME_FIELDS = "datetime,timestamp"


def _make_module(name: str, attrs: dict[str, Any]) -> types.ModuleType:
    mod = types.ModuleType(name)
    for k, v in attrs.items():
        setattr(mod, k, v)
    sys.modules[name] = mod
    return mod


# ---------------------------------------------------------------------------
# ckanext.datapusher_plus.datastore_utils  (SQL helpers)
# ---------------------------------------------------------------------------


def _dsu_index_exists(resource_id: str, field: str) -> bool:  # noqa: ARG001
    """No CKAN datastore in qsv -- always report no index, so SQL-using
    helpers fall through their "cannot resolve" branches and return ``None``.
    """
    return False


def _dsu_datastore_search_sql(sql: str) -> dict:  # noqa: ARG001
    """No datastore to query. Return the same dict shape CKAN's real
    ``datastore_search_sql`` returns (``{"records": [...]}``), so callers
    that do ``records.get("records", [])`` get an empty list and short-
    circuit cleanly instead of raising ``AttributeError`` on a bare list.
    """
    return {"records": []}


# ---------------------------------------------------------------------------
# ckan.plugins.toolkit  (a tiny subset of CKAN's helper namespace)
# ---------------------------------------------------------------------------


class _TkConfig:
    """Mimics ``ckan.plugins.toolkit.config`` with dict-like .get()."""

    def __init__(self) -> None:
        self._data: dict[str, Any] = {
            # DP+ checks this when deciding whether to expose SQL search.
            # Must match exactly what jinja2_helpers.py reads
            # (see the decorator-time check around line 297-303 of the
            # vendored helpers). A non-matching key would silently fall
            # back to the default and break any future gating logic.
            "ckan.datastore.sqlsearch.enabled": False,
        }

    def get(self, key: str, default: Any = None) -> Any:
        return self._data.get(key, default)


def install() -> None:
    """Install all three stub modules into ``sys.modules`` so the vendored
    ``jinja2_helpers`` can ``import`` them.

    Idempotent -- safe to call repeatedly (e.g. from multiple test runs).
    """
    # Parent CKAN namespaces have to exist before child modules can be
    # resolved as ``ckan.plugins.toolkit`` / ``ckanext.datapusher_plus.config``.
    sys.modules.setdefault("ckan", types.ModuleType("ckan"))
    sys.modules.setdefault("ckan.plugins", types.ModuleType("ckan.plugins"))
    sys.modules.setdefault("ckanext", types.ModuleType("ckanext"))
    sys.modules.setdefault(
        "ckanext.datapusher_plus",
        types.ModuleType("ckanext.datapusher_plus"),
    )

    _make_module(
        "ckanext.datapusher_plus.config",
        {
            "LATITUDE_FIELDS": LATITUDE_FIELDS,
            "LONGITUDE_FIELDS": LONGITUDE_FIELDS,
            "JINJA2_BYTECODE_CACHE_DIR": JINJA2_BYTECODE_CACHE_DIR,
            "DATE_FIELDS": DATE_FIELDS,
            "DATETIME_FIELDS": DATETIME_FIELDS,
        },
    )

    _make_module(
        "ckanext.datapusher_plus.datastore_utils",
        {
            "index_exists": _dsu_index_exists,
            "datastore_search_sql": _dsu_datastore_search_sql,
        },
    )

    _make_module(
        "ckan.plugins.toolkit",
        {
            "config": _TkConfig(),
        },
    )
