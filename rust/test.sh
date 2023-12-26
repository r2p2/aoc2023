#!/usr/bin/env bash
set -e

run_rst_unit_test() {
	cargo test
}

build_rst() {
	cargo build --release
}
run_rst_int_test() {
	day="${1}"
	part="${2}"
	input="${3}"
	expected="${4}"

	echo "run integration test day:${day} part:${part}"
	result=$(./target/release/aoc2023 "${day}" "${part}" "${input}")
	if [[ "${result}" == "${expected}" ]]; then
		echo "  OK";
	else
		echo "  FAIL ('${result}' != '${expected}')";
	fi
}

run_rst_unit_test
build_rst
run_rst_int_test 2 1 "../inputs/day2_part1.txt" 2600
run_rst_int_test 2 2 "../inputs/day2_part1.txt" 86036
run_rst_int_test 3 1 "../inputs/day3_part1.txt" 517021
run_rst_int_test 3 2 "../inputs/day3_part1.txt" 81296995
run_rst_int_test 5 1 "../inputs/day5_part1.txt" 218513636
run_rst_int_test 5 2 "../inputs/day5_part1.txt" 81956384
run_rst_int_test 16 1 "../inputs/day16_part1.txt" 7517
run_rst_int_test 16 2 "../inputs/day16_part1.txt" 7741
run_rst_int_test 19 1 "../inputs/day19_part1.txt" 353046
run_rst_int_test 19 2 "../inputs/day19_part1.txt" 125355665599537
run_rst_int_test 20 1 "../inputs/day20_part1.txt" 807069600
run_rst_int_test 22 1 "../inputs/day22_part1.txt" 454
run_rst_int_test 22 2 "../inputs/day22_part1.txt" 74287
run_rst_int_test 25 1 "../inputs/day25_part1.txt" 282626
