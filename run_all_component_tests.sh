#!/bin/bash
# Run tests for all 15 components and collect results

COMPONENTS=(
  "css_types"
  "css_parser_core"
  "css_cascade"
  "css_matcher_core"
  "css_stylist_core"
  "css_engine"
  "css_matcher_pseudo"
  "css_parser_values"
  "css_layout_box_model"
  "css_layout_flexbox"
  "css_layout_grid"
  "css_animations"
  "css_custom_properties"
  "css_stylist_cache"
  "css_invalidation"
)

TOTAL_PASS=0
TOTAL_FAIL=0
TOTAL_TESTS=0

echo "======================================================================="
echo "CONTRACT VALIDATION: Testing All 15 Components"
echo "======================================================================="
echo ""

for comp in "${COMPONENTS[@]}"; do
  echo "Testing $comp..."
  cd "/home/user/Corten-CSSEngine/components/$comp"

  OUTPUT=$(cargo test --quiet 2>&1)
  EXIT_CODE=$?

  if [ $EXIT_CODE -eq 0 ]; then
    # Extract test count from output
    TEST_COUNT=$(echo "$OUTPUT" | grep -oP '\d+ passed' | grep -oP '\d+' | head -1)
    if [ -z "$TEST_COUNT" ]; then
      TEST_COUNT=0
    fi

    echo "  ✅ PASS: $TEST_COUNT tests"
    TOTAL_PASS=$((TOTAL_PASS + TEST_COUNT))
    TOTAL_TESTS=$((TOTAL_TESTS + TEST_COUNT))
  else
    echo "  ❌ FAIL: Tests failed"
    echo "$OUTPUT"
    TOTAL_FAIL=$((TOTAL_FAIL + 1))
  fi

  echo ""
done

echo "======================================================================="
echo "RESULTS:"
echo "  Total tests: $TOTAL_TESTS"
echo "  Total passing: $TOTAL_PASS"
echo "  Components failed: $TOTAL_FAIL"
if [ $TOTAL_FAIL -eq 0 ]; then
  echo "  Status: ✅ ALL COMPONENTS PASS (100%)"
  echo "======================================================================="
  exit 0
else
  echo "  Status: ❌ SOME COMPONENTS FAILED"
  echo "======================================================================="
  exit 1
fi
