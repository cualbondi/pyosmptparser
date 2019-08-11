#!/usr/bin/env python3

import pyosmptparser

import pytest


def test_it_works():
    p = pyosmptparser.Parser('test.pbf')
    pts = p.get_public_transports(150)

    pt1 = [p for p in pts if p.id == 85965][0]
    assert pt1.tags['name'] == 'Trolebus Quitumbe => La Y'
    assert len(pt1.geometry) == 0

    pt2 = [p for p in pts if p.id == 2030162][0]
    assert pt2.tags['name'] == 'B6 Mapasingue Oeste Ida'
    assert len(pt2.geometry) == 1

    pts = p.get_public_transports(1500)
    pt1 = [p for p in pts if p.id == 85965][0]
    assert len(pt1.geometry) > 0
