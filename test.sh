#!/usr/bin/env sh
set -e

run_zig_unit_test() {
	echo "run unit test: ${1}"
	zig test "${1}" 
}

run_zig_int_test() {
	day="${1}"
	part="${2}"
	input="${3}"
	expected="${4}"

	echo "run integration test day:${day} part:${part}"
	result=$(zig build run -- "${day}" "${part}" "${input}")
	if [[ "${result}" == "${expected}" ]]; then
		echo "  OK";
	else
		echo "  FAIL ('${result}' != '${expected}')";
	fi
}

run_rst_unit_test() {
	cargo -C "$(pwd)/rust/" -Z unstable-options test
}

run_rst_int_test() {
	day="${1}"
	part="${2}"
	input="${3}"
	expected="${4}"

	echo "run integration test day:${day} part:${part}"
	result=$(cargo -C "$(pwd)/rust" -Z unstable-options run --release -- "${day}" "${part}" "${input}")
	if [[ "${result}" == "${expected}" ]]; then
		echo "  OK";
	else
		echo "  FAIL ('${result}' != '${expected}')";
	fi
}

#pushd zig
#run_zig_unit_test "src/day1.zig"
#run_zig_unit_test "src/day3.zig"
#run_zig_unit_test "src/day4.zig"
#run_zig_unit_test "src/day8.zig"
#run_zig_unit_test "src/day9.zig"
#run_zig_unit_test "src/day11.zig"
#run_zig_unit_test "src/day12.zig"
#run_zig_unit_test "src/day15.zig"
#run_zig_int_test 1 1 "../inputs/day1_part1.txt" 56108
#run_zig_int_test 1 2 "../inputs/day1_part1.txt" 55652
#run_zig_int_test 3 1 "../inputs/day3_part1.txt" 517021
#run_zig_int_test 4 1 "../inputs/day4_part1.txt" 26218
#run_zig_int_test 8 1 "../inputs/day8_part1.txt" 21883
#run_zig_int_test 8 2 "../inputs/day8_part1.txt" 12833235391111
#run_zig_int_test 9 1 "../inputs/day9_part1.txt" 1789635132
#run_zig_int_test 9 2 "../inputs/day9_part1.txt" 913
#run_zig_int_test 11 1 "../inputs/day11_part1.txt" 9563821
#run_zig_int_test 11 2 "../inputs/day11_part1.txt" 827009909817
#run_zig_int_test 12 1 "../inputs/day12_part1.txt" 7173
#run_zig_int_test 15 1 "../inputs/day15_part1.txt" 510801
#popd
run_rst_unit_test
run_rst_int_test 2 1 "../inputs/day2_part1.txt" 2600
run_rst_int_test 2 2 "../inputs/day2_part1.txt" 86036
run_rst_int_test 3 1 "../inputs/day3_part1.txt" 517021
run_rst_int_test 3 2 "../inputs/day3_part1.txt" 81296995
run_rst_int_test 16 1 "../inputs/day16_part1.txt" 7517
run_rst_int_test 16 2 "../inputs/day16_part1.txt" 7741
run_rst_int_test 19 1 "../inputs/day19_part1.txt" 353046
run_rst_int_test 19 2 "../inputs/day19_part1.txt" 125355665599537
run_rst_int_test 20 1 "../inputs/day20_part1.txt" 807069600
run_rst_int_test 22 1 "../inputs/day22_part1.txt" 454
run_rst_int_test 22 2 "../inputs/day22_part1.txt" 74287
