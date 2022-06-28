# GCM-bot: A chart info bot for GekiChuMai! (only maimai for now)

## Usage

Method 1. Slash commands (recommended usage)

Method 2. @GCM-bot `command-name` `command-arguments`

**Nicknames for songs are supported - try stuff out!**

**Example usage:**

/mai-info bbb

@GCM-bot mai-info 3 seconds until dawn

## Help / How to Contribute

If there are requested features or nicknames to songs that you want to add, you can choose one of the below methods:

1. Send the question to @Lomo#2363 on Discord.
2. Add an issue on this repository.
3. Contribute the requested change as a pull request.

Nicknames for songs are contained in the `data/aliases/{locale}/{game}.tsv` folder. Inside the tsv file, there are tab-separated lines, one line for each song.

The first item in the line is always the song title, and the following items are nicknames for that song. If you wish to add nicknames, just add them one at a time separated by tabs.

Example line of `data/aliases/en/maimai.tsv`:
```
封焔の135秒	135	135 seconds of flame	135 seconds
```
Here there are three nicknames each separated by a tab.

## TODO

- [ ] Add KR 🇰🇷
- [ ] Remove hard panics and unwraps
- [ ] Refactor
- [ ] Add Chuni
- [ ] Add Ongeki
