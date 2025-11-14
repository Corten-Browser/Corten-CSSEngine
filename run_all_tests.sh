#!/bin/bash

# Run tests for all components and collect results

echo "======================================================================="
echo "CSS ENGINE - INTEGRATION TEST RUN"
echo "======================================================================="
echo ""

TOTAL_TESTS=0
TOTAL_PASSED=0
TOTAL_FAILED=0
COMPONENTS_TESTED=0

# Core components to test (in dependency order)
COMPONENTS=(
    "css_types"
    "css_parser_core"
    "css_matcher_core"
    "css_cascade"
    "css_stylist_core"
    "css_engine"
)

for component in "${COMPONENTS[@]}"; do
    echo "-----------------------------------------------------------------------"
    echo "Testing: $component"
    echo "-----------------------------------------------------------------------"

    if [ -d "components/$component" ]; then
        cd "components/$component"

        # Run tests and capture output
        TEST_OUTPUT=$(cargo test --quiet 2>&1)
        TEST_EXIT_CODE=$?

        # Extract test results
        if [ $TEST_EXIT_CODE -eq 0 ]; then
            # Parse successful test output
            PASSED=$(echo "$TEST_OUTPUT" | grep "test result:" | sed -n 's/.*\([0-9]\+\) passed.*/\1/p')
            FAILED=$(echo "$TEST_OUTPUT" | grep "test result:" | sed -n 's/.*\([0-9]\+\) failed.*/\1/p')

            if [ -z "$PASSED" ]; then
                PASSED=0
            fi
            if [ -z "$FAILED" ]; then
                FAILED=0
            fi

            TESTS=$((PASSED + FAILED))

            TOTAL_TESTS=$((TOTAL_TESTS + TESTS))
            TOTAL_PASSED=$((TOTAL_PASSED + PASSED))
            TOTAL_FAILED=$((TOTAL_FAILED + FAILED))
            COMPONENTS_TESTED=$((COMPONENTS_TESTED + 1))

            echo "✓ $component: $PASSED passed, $FAILED failed"
        else
            echo "✗ $component: Test run failed"
            echo "$TEST_OUTPUT" | tail -10
            COMPONENTS_TESTED=$((COMPONENTS_TESTED + 1))
        fi

        cd /home/user/Corten-CSSEngine
    else
        echo "⚠ $component: Component directory not found"
    fi

    echo ""
done

echo "======================================================================="
echo "TEST SUMMARY"
echo "======================================================================="
echo "Components tested: $COMPONENTS_TESTED/${#COMPONENTS[@]}"
echo "Total tests run: $TOTAL_TESTS"
echo "Passed: $TOTAL_PASSED"
echo "Failed: $TOTAL_FAILED"
echo ""

if [ $TOTAL_FAILED -eq 0 ]; then
    echo "✓✓✓ ALL TESTS PASSED ✓✓✓"
    exit 0
else
    echo "✗✗✗ SOME TESTS FAILED ✗✗✗"
    exit 1
fi
