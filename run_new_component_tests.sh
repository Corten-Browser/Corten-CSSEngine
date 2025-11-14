#!/bin/bash
# Contract validation - run tests for new components

echo "======================================================================="
echo "CONTRACT VALIDATION: New Components (Phase 6)"
echo "======================================================================="
echo ""

total_tests=0
total_passed=0
components=("css_transforms" "css_layout_multicolumn" "css_transitions" "css_media_queries")

for component in "${components[@]}"; do
    echo "Testing $component..."
    cd "components/$component"
    
    # Run cargo test and capture output
    output=$(cargo test 2>&1)
    exit_code=$?
    
    # Extract test count
    if echo "$output" | grep -q "test result:"; then
        passed=$(echo "$output" | grep "test result:" | grep -oP '\d+(?= passed)')
        failed=$(echo "$output" | grep "test result:" | grep -oP '\d+(?= failed)')
        
        if [ -n "$passed" ]; then
            total_passed=$((total_passed + passed))
            total_tests=$((total_tests + passed))
        fi
        
        if [ -n "$failed" ] && [ "$failed" -gt 0 ]; then
            total_tests=$((total_tests + failed))
        fi
    fi
    
    if [ $exit_code -eq 0 ]; then
        echo "✅ $component: Tests passing"
    else
        echo "❌ $component: Tests FAILED"
        echo "$output"
    fi
    
    cd ../..
    echo ""
done

echo "======================================================================="
echo "CONTRACT VALIDATION SUMMARY"
echo "======================================================================="
echo "Total tests: $total_tests"
echo "Tests passed: $total_passed"

if [ $total_passed -eq $total_tests ] && [ $total_tests -gt 0 ]; then
    echo ""
    echo "✅ ALL NEW COMPONENTS PASS CONTRACT VALIDATION"
    echo "   - 100% pass rate achieved"
    echo "   - Ready for integration testing"
    exit 0
else
    echo ""
    echo "❌ CONTRACT VALIDATION FAILED"
    echo "   - Some tests failing or not running"
    echo "   - Must fix before proceeding"
    exit 1
fi
