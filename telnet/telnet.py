#!/usr/bin/env python3
import pexpect
import os, sys, time

ip = "127.0.0.1"
port = "10000"
username = "nikitapekin@gmail.com"
password = "12345"

os.remove('../maildir/.lock')

child = pexpect.spawn('telnet '+ ip + ' ' + port)

child.expect('.\n')
child.logfile = sys.stdout.buffer
time.sleep(1)
child.sendline('1 login ' + username + ' ' + password)
child.expect('1 OK logged in successfully as nikitapekin@gmail.com')
child.sendline('2 select INBOX')
child.expect('successful')
child.sendline('3 fetch 1:2 (FLAGS BODY[HEADER.FIELDS (DATE FROM)])')
child.expect('unimplemented')