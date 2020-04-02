# Create App

```js
npx react-native init my-app
cd my-app
npx react-native run-android
```

# Running app on Android devices

Enable Debugging over USB

```bash
lsusb
```

grab the first four digits from the device ID: 22b8:2e76. In this case, it's 22b8

```bash
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="22b8", MODE="0666", GROUP="plugdev"' | sudo tee /etc/udev/rules.d/51-android-usb.rules

adb devices

react-native run-android
```

## Connecting to the development server

### Using adb reverse (recommended), Android 5.0 (Lollipop)

```bash
adb devices
adb -s <device name> reverse tcp:8081 tcp:8081
```

enable Live reloading from the Developer menu. Your app will reload whenever your JavaScript code has changed.

### Connect via Wi-Fi

```bash
/sbin/ifconfig
```

Open the in-app Developer menu

* Go to **Dev Settings** → **Debug server host & port for device**.
* Type in your machine's IP address and the port of the local dev server (e.g. 10.0.1.1:8081). 
* Go back to the **Developer menu** and select **Reload JS**.

enable Live reloading from the Developer menu. Your app will reload whenever your JavaScript code has changed.
