<h1 align="center">
  <img src="data/icons/app.drey.PaperPlane.svg" alt="Paper Plane" width="192" height="192"/>
  <br>
  Paper Plane
</h1>

<p align="center"><strong>Chat over Telegram on a modern and elegant client</strong></p>

<p align="center">
  <a href="https://hosted.weblate.org/engage/paper-plane/">
    <img src="https://hosted.weblate.org/widgets/paper-plane/-/main/svg-badge.svg" alt="Translation status" />
  </a>
  <a href="https://github.com/paper-plane-developers/paper-plane/actions/workflows/ci.yml">
    <img src="https://github.com/paper-plane-developers/paper-plane/actions/workflows/ci.yml/badge.svg" alt="CI status"/>
  </a>
  <a href="https://t.me/paperplanechat">
    <img src="https://img.shields.io/static/v1?label=Chat&message=@paperplanechat&color=blue&logo=telegram" alt="Telegram group">
  </a>
</p>

<p align="center">
  <img width=600 src="data/resources/screenshots/screenshot1.png" alt="Screenshot"/>
</p>

## Overview

Paper Plane is an alternative Telegram client that utilizes libadwaita for its user interface, aiming to adhere to the design principles of the GNOME desktop.

## Features

While Paper Plane is still under development and not yet feature complete, the following functionalities are already implemented:

- Multi-account support.
- Viewing of text messages, images, stickers, and files.
- Sending of text messages and images.
- Replying to messages.
- Chats and users search.

## Installation Instructions

**Note:** Paper Plane is currently under development and may not be considered stable software.

All installation methods, except Flathub, default to using test API credentials. The use of these credentials may result in your account being **banned**, especially if your account was recently created or it's somehow considered suspicious by Telegram.

You can recover a banned account by sending an email to `recover@telegram.org`, but if you prefer to be on the safe side, we recommend trying the Flathub builds, or building the app yourself with more secure credentials.

### Flathub Beta

If you'd like to try Paper Plane, you can install the latest beta release from the Flathub Beta repository using the following command:

```shell
$ flatpak remote-add --if-not-exists flathub-beta https://flathub.org/beta-repo/flathub-beta.flatpakrepo
```

Then you can install the application by issuing:

```shell
$ flatpak install flathub-beta app.drey.PaperPlane
```

To keep Paper Plane updated, use the flatpak update command:

```shell
$ flatpak update
```

### CI Build (Not Recommended)

If you want to try the latest development version with test API credentials, you can download the latest CI build [here](https://nightly.link/paper-plane-developers/paper-plane/workflows/ci/main). After downloading, unzip the archive's content and install the application by using the command:

```shell
$ flatpak install paper-plane.flatpak
```

Please note that these builds have a higher risk of problems due to the more bleeding edge code and the use of testing credentials, as mentioned in [Installation Instructions](#installation-instructions).
In addition, you'll need to keep the application updated manually.

## Build Instructions

### GNOME Builder

The simplest way to build Paper Plane is by using GNOME Builder. Clone the repository and click the "Run" button to automatically build the app along with its dependencies, without even requiring you to open your terminal.

### Meson

#### Prerequisites

Ensure you have the following packages installed to build Paper Plane:

- meson
- cargo
- GTK >= 4.10 (with the patch included in the build-aux directory)
- libadwaita >= 1.4
- [rlottie](https://github.com/paper-plane-developers/rlottie)
- [TDLib 1.8.14](https://github.com/tdlib/td/commit/8517026415e75a8eec567774072cbbbbb52376c1)
- [Telegram API Credentials](https://my.telegram.org/auth?to=apps) (optional, but recommended)

Also, install the following GStreamer plugins to handle media files correctly:

- gstreamer-libav
- gstreamer-plugins-good

#### Building

```shell
meson . _build -Dtg_api_id=ID -Dtg_api_hash=HASH
ninja -C _build
sudo ninja -C _build install
```

## Contributing

We welcome all forms of contribution, including translation, design, art, and code. To translate, refer to [our weblate project](https://hosted.weblate.org/engage/paper-plane). Design contributions can be made to [our design repository](https://github.com/paper-plane-developers/paper-plane-designs).

For code contributions, please format your commit messages according to [conventional commits](https://www.conventionalcommits.org/en/v1.0.0), but capitalizing the description after the colon (`:`).

## Acknowledgements

Paper Plane's code architecture is inspired by [Fractal](https://gitlab.gnome.org/GNOME/fractal), and its logic is influenced by [Telegram X](https://github.com/TGX-Android/Telegram-X). The latter makes it easier to rightly use some of TDLib's features and take full advantage of its potential.
