#!/usr/bin/env bash

# standard command
command='echo "there are ${free_cores} available cores"'

# sleep between the tests
sleeptime=5

# how many cores do we have in total
cores=$((2 * $(grep ^cpu\\scores /proc/cpuinfo | uniq |  awk '{print $4}')))

#how many cores do we keep free for normal work?
security_cores=0

# threshold value, from we decide a core is fully used or not, in procent.
THRESHOLD=90.0

# creates loads array
loads_tresh=()

#Sadly this is needed to be universally usefull
export LANG="en_US"
#LC_NUMERIC="en_US"

# HELPER STRING
STRING_HELP="$0: [-esh]
  -e  --security_cores  How many (virtual) cores should the program not touch. Standard value ${security_cores}
  -s  --sleeptime       Each 'sleeptime' seconds the programm wakes up and reruns the command. Standard value: ${sleeptime}
  -c  --command         command to execute
  -h  --help            this help
"

help() {
  echo "${STRING_HELP}"
  exit
}

check_security_cores () {
  # checks weather `security_cores` is correctly set.
  if [ ${security_cores} -ge ${cores} ]
  then
    echo "More cores in reserve than available. Doesnt make sense."
    exit
  fi

  if [ ${security_cores} -lt 0 ]
  then
    echo "negative security_cores?"
    ecit
  fi
}

get_cores_load () {
	# get the average load of all available cores (vm_usage)
	loads=($(mpstat -P ALL 1 1 | awk '/Average:/ && $2 ~ /[0-9]/ {print $3}'))
  #echo "load" ${loads[@]}

	# fill this array with a all loads of cores with load > THRESHHOLD
	for i in "${loads[@]}"
	do
    #replace all , with .
	  load=${i//,/.}
    if (( $(echo "${load} > $THRESHOLD" | bc -l))) ;then
			loads_tresh+=(${load})
		fi
	done
}


POSITIONAL=()
while [[ $# -gt 0 ]]
do
key="$1"

case $key in
    -e|--security_cores)
    security_cores="$2"
    shift # past argument
    shift # past value
    ;;
    -s|--sleeptime)
    sleeptime="$2"
    shift # past argument
    shift # past value
    ;;
    -c|--command )
    command="$2"
    shift # past argument
    shift # past value
    ;;
    -h|--help)
    help
    shift # past argument
    ;;
#    --default)
#    DEFAULT=YES
#    shift # past argument
#    ;;
    *)    # unknown option
    POSITIONAL+=("$1") # save it in an array for later
    shift # past argument
    ;;
esac
done
set -- "${POSITIONAL[@]}" # restore positional parameters

# checks weather the input make sense
check_security_cores

if [[ -n "${COMMAND}" ]]; then
  command="${COMMAND}"
fi

while [ 1 ]
do
	get_cores_load
	free_cores=$(echo "$cores - ${#loads_tresh[@]} - $security_cores" | bc -l)
	# echo "number of free cores "${free_cores}

	eval ${command}

	# clear the array
	loads_tresh=()
	sleep $sleeptime
done

