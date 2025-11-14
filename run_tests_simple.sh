#!/bin/bash

echo "======================================================================="
echo "CSS ENGINE - INTEGRATION TEST RUN"
echo "======================================================================="
echo ""

cd /home/user/Corten-CSSEngine

# Test css_types
echo "=== Testing css_types ==="
cd components/css_types && cargo test 2>&1 | grep "test result:"
cd /home/user/Corten-CSSEngine
echo ""

# Test css_parser_core
echo "=== Testing css_parser_core ==="
cd components/css_parser_core && cargo test 2>&1 | grep "test result:"
cd /home/user/Corten-CSSEngine
echo ""

# Test css_matcher_core
echo "=== Testing css_matcher_core ==="
cd components/css_matcher_core && cargo test 2>&1 | grep "test result:"
cd /home/user/Corten-CSSEngine
echo ""

# Test css_cascade
echo "=== Testing css_cascade ==="
cd components/css_cascade && cargo test 2>&1 | grep "test result:"
cd /home/user/Corten-CSSEngine
echo ""

# Test css_stylist_core
echo "=== Testing css_stylist_core ==="
cd components/css_stylist_core && cargo test 2>&1 | grep "test result:"
cd /home/user/Corten-CSSEngine
echo ""

# Test css_engine
echo "=== Testing css_engine ==="
cd components/css_engine && cargo test 2>&1 | grep "test result:"
cd /home/user/Corten-CSSEngine
echo ""

echo "======================================================================="
echo "ALL COMPONENT TESTS COMPLETE"
echo "======================================================================="
