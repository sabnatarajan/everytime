# Every Time

`everytime` is a command line program to work with dates and times

```
$ everytime now                 # prints current local time
2021-04-06 23:20:45.281102 -05:00

$ everytime now -e              # prints current Epoch time
1617769538

$ everytime now UTC             # prints current UTC time
2021-04-07 04:21:08.630195 UTC

$ everytime now Europe/London   # prints current time at a timezone
2021-04-07 04:21:08.630195 UTC

$ everytime parse 1617767254    # parses an epoch timestamp
2021-04-06T22:47:34-05:00

```

# To-Do
- [ ] Display the time across common timezones
- [ ] Handle micro/nano-seconds while parsing