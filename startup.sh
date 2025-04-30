#!/bin/bash

function log_message () {
    time_stamp=$(date '+%Y-%m-%d %H:%M:%S')
    msg="$time_stamp | $1"
    echo $msg
    echo $msg >> $2
}

if [ ! -d "./system_logs" ]; then
  echo "./system_logs does not exist, creating logging directory"
  mkdir "./system_logs"
  echo "Logging directory at ./system_logs created"
fi

date=$(date '+%Y-%m-%d')
log_path=$"./system_logs/$date-log.log"

if [ ! -f "./system_logs/$date-log.log" ]; then
    echo "./system_logs/$date-log.log does not exist"
    touch "./system_logs/$date-log.log"
    echo "./system_logs/$date-log.log created"
fi

log_message "Starting middle tier system" $log_path

log_message "Middle tier system started" $log_path

log_message "Starting middle tier system" $log_path

log_message "Middle tier system started" $log_path