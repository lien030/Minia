# Minia

![GitHub License](https://img.shields.io/github/license/lien030/Minia?style=flat&color=blue)

A tool for simply measuring device-to-device latency when streaming or casting a screen

一个用于测量串流或投屏时屏幕间延迟的工具

ミラーキャストやストリーム時、ディスプレイ間の遅延を測定するツール

![app](/app.png)

## Features

- 🚀 Used CNN-based QR code detector ([WeChatCV](https://github.com/WeChatCV)) 
- Test results automatically saved as csv file
- Automatically recognizes the QR code position on the screen

## Usage

You can download [Minia](https://github.com/lien030/Minia/releases/latest) from the release page.

1.  Open the [URL](https://lien030.github.io/qrcode-clock/) in your device's browser. 🔗https://lien030.github.io/qrcode-clock/
2. To start the Minia.exe, use the  <kbd>N</kbd> or  <kbd>P</kbd>  key to select the camera and the <kbd>C</kbd> key to continue to the next step.
3.  Just as in the above image, when the delay is detected press C to go to the next step, calculate & record the result.

※ Click <kbd>Ctrl</kbd> + <kbd>C</kbd> on the black terminal to abort the measurement at any time.

## Development

Used the [opencv-rust](https://crates.io/crates/opencv) bindings library and uses the wechat_qrcode module of opencv.

Before the build, please install the [opencv](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md) and set the environment variables correctly.

**※ NOTE: You need to install opencv with the wechat_qrcode module, I will upload the pre-built opencv on the release page.**



**💡Reference Links**

- https://github.com/twistedfall/opencv-rust#environment-variables
- https://medium.com/@rajeshpachaikani/wechat-qr-reader-in-rust-using-opencv-6078d429255f

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=lien030/Minia&type=Timeline)](https://star-history.com/#lien030/Minia&Timeline)

