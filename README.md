# JetStream Desktop
JetStream is an Android-PC integration system. This repository contains the desktop app for the system.

# Experimental Setup
- Both the mobile device and the workstation have to be connected to the same LAN, via wired or wireless means.
- Both devices can be connected to a wired or wireless access point, or one device can be connected to the wireless hotspot of the other device.
- Both devices should have port 8000 and 5353 allowed through their firewall and the LAN router should also allow traffic through these ports.
- Most regular networks will have these ports allowed since they are used for mDNS discovery and miscellaneous traffic.

# Testing
- The JetStream Android App is installed on an android device and the JetStream Desktop app is installed on either a Windows or Linux system.
- The desktop app is run and it is observed that the UI is visible and a system tray icon is created.
- The android app is opened and it is confirmed that the background service required to connect is running. This is done by making sure that there is a persistant notification showing that JetStream Service is active but not connected.
- In the android app UI The discover servers button is pressed and in the displayed list of servers, the desktop will be visible.
- The desktop is selected from the server list and it is connected to automatically.
- Now the notification displays that the service is connected to the desktop.
- When a notification is received on the android device it is synced to the desktop device and a desktop notification is genereated.
