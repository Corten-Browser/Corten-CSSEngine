#!/bin/bash
# Run tests for all 19 components (15 original + 4 new) and collect results

COMPONENTS=(
  # Phase 1: Core CSS2.1 (6 components)
  "css_types"
  "css_parser_core"
  "css_cascade"
  "css_matcher_core"
  "css_stylist_core"
  "css_engine"
  # Phase 2: Advanced Selectors & Box Model (3 components)
  "css_matcher_pseudo"
  "css_parser_values"
  "css_layout_box_model"
  # Phase 3: Layout Systems (4 components - 2 original + 2 NEW)
  "css_layout_flexbox"
  "css_layout_grid"
  "css_transforms"
  "css_layout_multicolumn"
  # Phase 4: Animations & Variables (4 components - 2 original + 2 NEW)
  "css_animations"
  "css_custom_properties"
  "css_transitions"
  # Phase 5: Optimizations (2 components)
  "css_stylist_cache"
  "css_invalidation"
  # Core Feature (NEW)
  "css_media_queries"
)

TOTAL_PASS=0
TOTAL_FAIL=0
TOTAL_TESTS=0
FAILED_COMPONENTS=()

echo "======================================================================="
echo "INTEGRATION TEST: Testing All 19 Components"
echo "======================================================================="
echo ""

for component in "${COMPONENTS[@]}"; do
    echo "Testing $component..."

    # Navigate to component directory
    cd "components/$component" || {
        echo "  ❌ ERROR: Component directory not found"
        FAILED_COMPONENTS+=("$component")
        cd ../..
        continue
    }

    # Run tests and capture output
    output=$(cargo test 2>&1)
    exit_code=$?

    if [ $exit_code -eq 0 ]; then
        # Extract test counts from all test suites
        passed=$(echo "$output" | grep "test result: ok" | grep -oP '\d+(?= passed)' | awk '{s+=$1} END {print s}')
        failed=$(echo "$output" | grep "test result:" | grep -oP '\d+(?= failed)' | awk '{s+=$1} END {print s}')

        # Handle cases where grep returns empty
        passed=${passed:-0}
        failed=${failed:-0}

        tests=$((passed + failed))

        if [ "$failed" -eq 0 ]; then
            echo "  ✅ PASS: $tests tests"
            TOTAL_PASS=$((TOTAL_PASS + passed))
            TOTAL_TESTS=$((TOTAL_TESTS + tests))
        else
            echo "  ❌ FAIL: $passed passed, $failed failed"
            FAILED_COMPONENTS+=("$component")
            TOTAL_PASS=$((TOTAL_PASS + passed))
            TOTAL_FAIL=$((TOTAL_FAIL + failed))
            TOTAL_TESTS=$((TOTAL_TESTS + tests))
        fi
    else
        echo "  ❌ ERROR: Tests failed to run"
        FAILED_COMPONENTS+=("$component")
    fi

    echo ""
    cd ../..
done

echo "======================================================================="
echo "RESULTS:"
echo "  Total tests: $TOTAL_TESTS"
echo "  Total passing: $TOTAL_PASS"
echo "  Total failing: $TOTAL_FAIL"
echo "  Components failed: ${#FAILED_COMPONENTS[@]}"

if [ ${#FAILED_COMPONENTS[@]} -eq 0 ]; then
    echo "  Status: ✅ ALL COMPONENTS PASS (100%)"
    exit 0
else
    echo "  Status: ❌ FAILURES DETECTED"
    echo "  Failed components: ${FAILED_COMPONENTS[*]}"
    exit 1
fi
echo "======================================================================="
