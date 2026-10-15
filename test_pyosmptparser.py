#!/usr/bin/env python3

import pyosmptparser

import pytest


def test_version():
    assert pyosmptparser.__version__.count('.') == 2


def test_it_works():
    p = pyosmptparser.Parser.new_ptv2('test.pbf')
    pts = p.get_public_transports(150)

    pt1 = [p for p in pts if p.id == 85965][0]
    assert pt1.tags['name'] == 'Trolebus Quitumbe => La Y'
    assert pt1.info['version'] == '231'
    assert pt1.info['timestamp'] == '1577992722'
    assert len(pt1.geometry) == 0
    assert pt1.status.code == 501
    assert pt1.status.detail == 'Broken'

    pt2 = [p for p in pts if p.id == 2030162][0]
    assert pt2.tags['name'] == 'B6 Mapasingue Oeste Ida'
    assert pt2.info['version'] == '13'
    assert pt2.info['timestamp'] == '1555013271'
    assert len(pt2.geometry) == 1
    assert pt2.status.code == 0
    assert pt2.status.detail == ''
    lon, lat = pt2.geometry[0][0]
    assert -80.5 < lon < -79.5 and -2.5 < lat < -1.5

    pts = p.get_public_transports(1500)
    pt1 = [p for p in pts if p.id == 85965][0]
    assert len(pt1.geometry) > 0


def test_stops_keep_member_order_and_tags():
    p = pyosmptparser.Parser.new_ptv2('test.pbf')
    pt = [p for p in p.get_public_transports(150) if p.id == 85965][0]
    assert len(pt.stops) > 10
    first = pt.stops[0]
    assert first.id == 890878123
    assert isinstance(first.tags, dict)
    assert -79 < first.lon < -78 and -1 < first.lat < 0
    assert 'Node(id=890878123' in repr(first)


def test_num_threads_and_generic_filter():
    pts = pyosmptparser.Parser.new_ptv2('test.pbf', 1).get_public_transports(150)
    assert {pt.id for pt in pts} == {85965, 2030162}

    pts = pyosmptparser.Parser('test.pbf', 'route=bus', num_threads=2).get_public_transports(150)
    assert [pt.id for pt in pts] == [2030162]


def test_missing_file_raises():
    with pytest.raises(FileNotFoundError):
        pyosmptparser.Parser.new_ptv2('does-not-exist.pbf')


def _area(areas, id_type, id):
    return [a for a in areas if a.id_type == id_type and a.id == id][0]


def _has_point(ring, lon, lat):
    return any(abs(x - lon) < 1e-6 and abs(y - lat) < 1e-6 for x, y in ring)


def test_get_areas_builds_multipolygons():
    # areas.pbf: hand made admin areas, see tests/areas.osm in osmptparser
    p = pyosmptparser.Parser.new_aa('areas.pbf')
    areas = p.get_areas(150)
    # 3 boundary relations + 1 standalone closed way; the bus route is filtered out
    assert sorted((a.id_type, a.id) for a in areas) == [('r', 100), ('r', 200), ('r', 300), ('w', 6)]

    # outer ring split in two ways given out of member order + one inner ring (hole)
    province = _area(areas, 'r', 100)
    assert province.tags['name'] == 'Province'
    assert province.tags['admin_level'] == '4'
    assert province.info['version'] == '1'
    assert province.status.code == 0
    assert len(province.geometry) == 1            # 1 polygon
    outer, hole = province.geometry[0]            # outer ring + 1 hole
    assert len(outer) == 5 and outer[0] == outer[-1]
    assert _has_point(outer, 0.0, 0.0) and _has_point(outer, 1.0, 1.0)
    assert len(hole) == 5 and hole[0] == hole[-1]
    assert _has_point(hole, 0.4, 0.4) and _has_point(hole, 0.6, 0.6)

    # two closed outer ways -> 2 polygons without holes
    town = _area(areas, 'r', 200)
    assert town.status.code == 0
    assert [len(polygon) for polygon in town.geometry] == [1, 1]

    # outer way that cannot be closed -> broken, no geometry
    broken = _area(areas, 'r', 300)
    assert broken.status.code == 501
    assert broken.status.detail == 'Broken'
    assert broken.geometry == []

    # standalone closed way tagged boundary=administrative
    village = _area(areas, 'w', 6)
    assert village.tags['name'] == 'Village'
    assert village.status.code == 0
    assert len(village.geometry) == 1 and len(village.geometry[0]) == 1
    assert len(village.geometry[0][0]) == 5

    assert repr(province) == 'Area(id=r100, name="Province", status=0)'


def test_get_areas_with_generic_filter_and_num_threads():
    p = pyosmptparser.Parser('areas.pbf', 'admin_level=8', num_threads=2)
    assert sorted(a.id for a in p.get_areas(150)) == [200, 300]


def test_node_role():
    p = pyosmptparser.Parser.new_ptv2('areas.pbf')
    bus = p.get_public_transports(150)[0]
    assert bus.id == 400
    assert [(s.id, s.role) for s in bus.stops] == [(61, 'stop')]
    assert repr(bus.stops[0]) == 'Node(id=61, lon=0.5, lat=0.5, role="stop")'

    p = pyosmptparser.Parser.new_ptv2('test.pbf')
    pt = [p for p in p.get_public_transports(150) if p.id == 2030162][0]
    assert {s.role for s in pt.stops} <= {'stop', 'platform', 'stop_exit_only', 'platform_exit_only',
                                          'stop_entry_only', 'platform_entry_only'}


def test_missing_file_raises_for_areas():
    with pytest.raises(FileNotFoundError):
        pyosmptparser.Parser.new_aa('does-not-exist.pbf')
