#!/bin/sh
wasm-pack build --target web && \
    rollup ./main.js --format iife --file ./pkg/bundle.js && \
    cp index.html ./pkg