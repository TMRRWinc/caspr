
![Logo](https://caspr-distribution.s3.ca-central-1.amazonaws.com/casprheader.png)



Caspr is a Desktop GPT Wrapper built on Tauri and Sveltekit. With Caspr, ChatGPT is just a keystroke away. This started as a closed source project, but the amount of time we are able to spend on it has shrunk and we thought it would just be best to let the community contribute. We are excited to see what we will build together!



## Installation

Install Caspr via our official Github releases or clone the Repo and run

```bash
  npm install
```
then add a file named .env to the project root with the following content
```bash
# Public
PUBLIC_API_URL=<a URL that will respond to ping>
```
the run
```bash
  npm run tauri build
```

If downloading via the released executable, your system may yell at you like so:

### MacOS    
 
 
 ![Unknown Dev](https://caspr-distribution.s3.ca-central-1.amazonaws.com/unknowndev.png)
 
 To circumvent this warning: 

- In the Finder  on your Mac, locate the app you want to open.

- Don’t use Launchpad to do this. Launchpad doesn’t allow you to access the shortcut menu.

- Control-click the app icon, then choose Open from the shortcut menu.

- Click Open.

The app is saved as an exception to your security settings, and you can open it in the future by double-clicking it just as you can any registered app.

### Windows
![Unverified Publicsher](https://caspr-distribution.s3.ca-central-1.amazonaws.com/unknown-publisher-warning-message.png)

To circumvent this warning:
- Click "Run anyway"

### Entering Api Key
After Installation, open settings either via menu tray or in the app:
  - Click the Caspr Cog in the Bottom Right
  - Click Open Settings -> API
  - Paste API Key and Click "Set API Key"
  - Caspr Will Reboot

You're all set!

## Default Hotkeys

To open the app:
```CMD/Ctrl + Shift + C```

To toggle conversation history:
```CMD/Ctrl + Y```

To copy response:
 ```CMD/Ctrl + C```

To regenerate the prompt:
```CMD/Ctrl + R```

To start a new conversation:
```CMD/Ctrl + X```



## Features

- Local-only conversation history
- Light/dark mode toggle
- Auto-start toggle
- Usage tracking
- Always on top toggle


## Contributing

Contributions are always welcome!

Here's some features we would like to see implemented:

- Customizable Hotkeys
- Always on top hotkey
- Selectable GPT engine
- Testing

Join our [Discord](https://discord.gg/AbKCgz7XCD) to stay connected


## Run Development Version

Clone the project

```bash
  git clone https://github.com/TMRRWinc/caspr
```

Go to the project directory

```bash
  cd caspr
```

Install dependencies

```bash
  npm install
```

Start the tauri dev environment

```bash
  npm run tauri dev
```


## Original Authors

- [@shawnvogt](https://github.com/shawnvogt)
- [@jackadair](https://github.com/jackadair)
- [@elebumm](https://github.com/elebumm)

[TMRRW Tech](https://www.tmrrwtech.com/)
