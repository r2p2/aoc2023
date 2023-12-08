#!/usr/bin/env sh
set -e

run_unit_test() {
	echo "run unit test: ${1}"
	zig test "${1}" 
}

run_int_test() {
	day="${1}"
	part="${2}"
	input="${3}"
	expected="${4}"

	echo "run integration test day:${day} part:${part}"
	result=$(zig build run -- "${day}" "${part}" "${input}")
	status="FAIL"
	if [[ "${result}" == "${expected}" ]]; then
		echo "  OK";
	else
		echo "  FAIL ("${result}" != "${expected}")";
	fi


}

run_unit_test "src/day1.zig"
run_unit_test "src/day3.zig"
run_unit_test "src/day8.zig"
run_int_test 1 1 "inputs/day1_part1.txt" 56108
run_int_test 1 2 "inputs/day1_part1.txt" 55652
run_int_test 8 1 "inputs/day8_part1.txt" 21883
run_int_test 8 2 "inputs/day8_part1.txt" 12833235391111
