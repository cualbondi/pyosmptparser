#!/usr/bin/env python3

import pyosmptparser

import pytest


def test_it_works():
    p = pyosmptparser.Parser.new_ptv2('test.pbf')
    pts = p.get_public_transports(150)

    pt1 = [p for p in pts if p.id == 85965][0]
    assert pt1.tags['name'] == 'Trolebus Quitumbe => La Y'
    assert pt1.info['version'] == '231'
    assert pt1.info['timestamp'] == '1577992722'
    assert len(pt1.geometry) == 0
    assert pt1.status.code == 501

    pt2 = [p for p in pts if p.id == 2030162][0]
    assert pt2.tags['name'] == 'B6 Mapasingue Oeste Ida'
    assert pt2.info['version'] == '13'
    assert pt2.info['timestamp'] == '1555013271'
    assert len(pt2.geometry) == 1
    assert pt2.status.code == 0

    pts = p.get_public_transports(1500)
    pt1 = [p for p in pts if p.id == 85965][0]
    assert len(pt1.geometry) > 0
