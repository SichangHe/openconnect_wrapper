# openconnect wrapper

## Objective

Every time I connect to Duke VPN using openconnect,
I need to type in the `<GROUP>`,
`<USERNAME>`,
and `<PASSWORD>`
manually,
which is annoying.
openconnect wrapper does those for me.

## Overview

openconnect wrapper runs

```shell
openconnect <URL_TO_CONNECT_TO>
```

and fill in

```text
<GROUP>
<USERNAME>
<PASSWORD>

```

into the process's stdin
so I don't have to type all the boilerplate every time.

## Usage

1. Change the `<URL_TO_CONNECT_TO>`,
    `<GROUP>`,
    `<USERNAME>`,
    and `<PASSWORD>`
    to your use case in `src/main.rs`.
1. Run the binary with `sudo` so openconnect can work:

   ```shell
   sudo cargo r --release
   ```

1. Type in the Response to the MFA option and press <kbd>enter</kbd>.
1. If you want to drop the connection and reconnect,
    press <kbd>r</kbd> and then <kbd>enter</kbd> so it reconnects.
1. To exit, press <kbd>ctrl</kbd> <kbd>c</kbd>.

## Compatibility

I use macOS and this is where openconnect wrapper is tested.
I'm confident that it would just work on Linux as well.
There is no way that this would work on Windows without modifications.
