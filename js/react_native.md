# react native

## Create App

```js
npx react-native init my-app
cd my-app
npx react-native start
```

## Running app on Android devices

Enable Debugging over USB

```bash
lsusb
```

ID: 22b8:2e76  前4位数字: 22b8

```bash
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="22b8", MODE="0666", GROUP="plugdev"' | sudo tee /etc/udev/rules.d/51-android-usb.rules

adb devices

react-native run-android
```

## Connecting to the development server

### Using adb reverse, Android 5.0 (Lollipop)

```bash
adb devices
adb -s <device name> reverse tcp:8081 tcp:8081
```
