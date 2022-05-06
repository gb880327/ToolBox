#!/bin/bash
expect -c '
  set timeout 30
  spawn {ssh}
  expect \
  "*yes/no*" {
      send "yes\r"
      exp_continue
  } " password:?" {
      send "{pwd}\r"
      exp_continue
  } "*\[#\\\$]" {
      {cmd}
      interact
  } "* to host" {
      send_user "Connect faild!"
      exit 2
  } timeout {
      send_user "Connect timeout!"
      exit 2
  } eof {
      send_user "Lost connect!"
      exit
  }
'