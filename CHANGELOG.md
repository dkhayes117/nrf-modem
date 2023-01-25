# Changelog

## 0.2.0 (19-01-23)

- UICC is now disabled when the LTE is disabled to save on power.
- The error enum is now non-exhaustive
- Added the ability to send SMS messages (https://github.com/diondokter/nrf-modem/pull/9)

## 0.1.1 (26-12-23)

- Fix: Made GnssUsecase fields public (https://github.com/diondokter/nrf-modem/pull/8)

## 0.1.0 (25-12-23)

Initial release with support for:
- AT commands
- AT notifications
- DNS
- TCP
- UDP
- DTLS