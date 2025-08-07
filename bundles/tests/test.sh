#!/usr/bin/env bash

this_dir="$(pwd)"

test_dir="${this_dir}"

bundle_path="$(realpath "${1}")"
output_dir="${test_dir}/build"

rm -drf "${output_dir}"
mkdir -p "${output_dir}"


function relative() {
	echo "./$(realpath --relative-to="${this_dir}" "${1}")"
}


RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'


function test_files() {

	rm -drf "${output_dir}/files"
	mkdir -p "${output_dir}/files"
	mkdir -p "${output_dir}/files/logs"



	for f in "${test_dir}/files"/*; do
		echo -n "Testing file $(relative "${f}")..."

		tectonic \
			--chatter minimal \
			--outdir "${output_dir}/files" \
			--bundle "${bundle_path}" \
			"${f}" \
		&> "${output_dir}/files/logs/$(basename "${f}").log"
		
		if [[ $? == 0 ]]; then
			echo -en "\r${GREEN}PASS${NC}"
		else
			echo -en "\r${RED}FAIL${NC}"
		fi
		echo " Tested file $(relative "${f}")"
	done


	for f in "${test_dir}/formats"/*; do
		echo -n "Testing format $(relative "${f}")..."

		tectonic \
			--chatter minimal \
			--outdir "${output_dir}/files" \
			-p --outfmt "fmt" \
			--bundle "${bundle_path}" \
			"${f}" \
		&> "${output_dir}/files/logs/$(basename "${f}").log"
		
		if [[ $? == 0 ]]; then
			echo -en "\r${GREEN}PASS${NC}"
		else
			echo -en "\r${RED}FAIL${NC}"
		fi
		echo " Tested format $(relative "${f}")"
	done
}



function test_class_single() {
	local class="${1}"
	local flags="${2}"

	mkdir -p "${output_dir}/classes/logs/failed"
	mkdir -p "${output_dir}/classes/logs/passed"
	local target="$(mktemp --tmpdir="${output_dir}/classes" "tmp.XXXX")"

	(
		echo "\documentclass{${class}}"
		echo ""

		if [[ $flags =~ "titleauth" ]]; then
			echo "title{Test Title}"
			echo "\author{An Author}"
			echo ""
		fi

		echo "\begin{document}"
		echo "Hello, world"
		echo "\end{document}"
	) > "${target}"


	tectonic \
		--chatter minimal \
		--outdir "${output_dir}/classes" \
		--bundle "${bundle_path}" \
		"${target}" \
	&> "${output_dir}/classes/logs/${class}.log"

	if [[ $? == 0 ]]; then
		echo "$class" >> "${output_dir}/classes/passed"
		mv "${output_dir}/classes/logs/${class}.log" "${output_dir}/classes/logs/passed"
		echo 0
	else
		echo "$class" >> "${output_dir}/classes/failed"
		mv "${output_dir}/classes/logs/${class}.log" "${output_dir}/classes/logs/failed"
		echo 1
	fi

	rm "${target}"
}


function test_classes() {
	rm -drf "${output_dir}/classes"
	mkdir -p "${output_dir}/classes"

	local fails=0
	local passes=0
	local skipped=0
	local total=$(wc -l < "${test_dir}/classes.list")

	cat "${test_dir}/classes.list" | while read class flags; do

		if [[ $flags =~ "xfail" ]]; then
			skipped=$(($skipped+1))
			continue
		fi

		r=$(test_class_single "${class}" "${flags}")

		if [[ $r == 0 ]]; then
			passes=$(($passes+1))
		else
			fails=$(($fails+1))
		fi

		echo -en "\r"
		echo -en "$(($passes + $fails + $skipped))/${total} "
		echo -en "${GREEN}P:${passes}${NC} "
		echo -en "${RED}F:${fails}${NC} "
		echo -en "S:${skipped}${NC} "
		echo -en "  Tested class ${class}"

		# Delete remnant of previous class name
		# and move cursor back.
		echo -en "                      "
		echo -en "\033[22D"
	done

	echo ""
}


function test_class() {
	class="$1"
	flags="$2"
	
	exists=false;
	exists=$(
		cat "${test_dir}/classes.list" | while read tclass flags; do
			if [[ "${class}" == "${tclass}" ]]; then
				echo "${class}"
				break
			fi
		done
	)

	if [[ -z $exists ]]; then
		echo "No such class "${class}""
		exit 1
	fi

	echo -n "Testing class "${class}"..."
	r=$(test_class_single "${class}" "${flags}")

	if [[ $r == 0 ]]; then
		echo -e " ${GREEN}Pass${NC}"
	else
		echo -e " ${RED}Fail${NC}"
	fi
}


case "${2}" in

	"all")
		test_files
		test_classes
	;;

	"files")
		test_files
	;;

	"classes")
		test_classes
	;;

	"class")
		test_class "${3}" "${4}"
	;;

	*)
		echo "Unknown test suite `${1}`"
		echo "See README.md"
	;;
esac