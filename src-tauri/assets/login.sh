#!/bin/bash
expect -c '
  set timeout 30
  spawn {ssh}
  expect_before ".*yes/no.*" { send "yes\r" }
  expect \
  " password:?" {
      send "{pwd}\r"
      exp_continue
  } "*\[#\\\$]" {
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