#!/usr/bin/env bash

rm ../zoop_web/public/zoop_engine*
rm ../zoop_web/src/services/zoop_engine*
cp -rf pkg/zoop_engine* ../zoop_web/public/
cp -rf pkg/zoop_engine* ../zoop_web/src/services
