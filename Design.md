# ESP-32 EInk timetable software

## Dictionary

EPD - E-Paper device (EInk)  
ESP - System on Chip with CPU, RAM, Flash, Wifi  
Board - development board with ESP and ports for external devices (epd and
battery)  
"the device" - meaning whole system: EPD + Board + ESP + battery Server -
backend server that host timetable

## Current state

PoC that can:

1. connect to wifi via WPA2Personal with login/pass
2. make http request to arbitrary server
3. draw images/text on EPD
4. go to deep sleep

## Functional requrements

### FR1: show timetable of meeting room

We understand this as:

- wake up
- connect to wifi
- make GET request to our Server
- draw timetable for following hours
- go to deep sleep for 15 minutes

We need clarification on:

- how often the device should contact the server
- what is the format of the timetable? what data should be shown
- will be timetable template stable or it will change sometimes
- should there be any additional info (like link/qr to reservation system,
  current time, messages, etc.)

### FR2: show notifications

We understand this as the device will pull nofitications from the server and
show them

We need clarification on:

- template of a notification
- how notifications are meant to be shown along with the timetable

## Non Functional Requirements

We need more info on NFRs

### NFR1: device have to live 1 month without rechaging

## Other

- when is the deadline?
