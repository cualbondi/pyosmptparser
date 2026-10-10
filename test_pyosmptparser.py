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
