# GCM-bot: A chart info bot for GekiChuMai!

This bot provides info about SEGA's three arcade rhythm games: maimai, CHUNITHM, and Ongeki. Find out levels and chart constants of the songs of your choice! Supports English nicknames and abbreviations for song titles (ex. "135 seconds" for "封焔の135秒")

Supports English and Korean song nicknames.

[Invite link](https://discord.com/api/oauth2/authorize?client_id=986651489529397279&permissions=2147502080&scope=applications.commands%20bot)

[Update notes](https://twitter.com/GCM_bot)

## Usage

Method 1. Slash commands (recommended usage)

Method 2. @GCM-bot `command-name` `command-arguments`

**Nicknames for songs are supported - try stuff out!**

**Example usage:**

/mai-info bbb

@GCM-bot mai-info 3 seconds until dawn

In addition to using these commands on servers, you can also DM the bot the same commands to get the same responses.

## Supported commands

- /\[mai|chuni|ongeki\]-info `song name`: Shows the maimai/chunithm/ongeki level and chart constants of the song, as well as other info.
- /\[mai|chuni|ongeki\]-jacket `song name`: Shows the maimai/chunithm/ongeki jacket of the chart.
- /help: Prints help info, and a link to this page.

There may be more hidden undocumented commands!

## Help / How to Contribute

If there are requested features or nicknames to songs that you want to add, you can choose one of the below methods:

1. Send the question to @Lomo#2363 on Discord or the [Discord support channel](https://discord.gg/8tVDqfZzAN).
2. Add an issue on this repository.
3. Contribute the requested change as a pull request.

Nicknames for songs are contained in the `data/aliases/{locale}/{game}.tsv` folder. Inside the tsv file, there are tab-separated lines, one line for each song.

The first item in the line is always the song title, and the following items are nicknames for that song. If you wish to add nicknames, just add them one at a time separated by tabs.

Example line of `data/aliases/en/maimai.tsv`:
```
封焔の135秒	135	135 seconds of flame	135 seconds
```
Here there are three nicknames each separated by a tab.

## Thanks

Thanks to the following people who are maintaining these awesome repositories:

- [zetaraku](https://github.com/zetaraku) of [arcade-songs](https://github.com/zetaraku/arcade-songs) (https://arcade-songs.zetaraku.dev/)
- [project-primera](https://github.com/project-primera) of [ongeki-score](https://github.com/project-primera/ongeki-score) (https://ongeki-score.net/)
  - Contributors to ongeki-score: [@Rinsaku471](https://twitter.com/Rinsaku471), [@RKS49019722](https://twitter.com/RKS49019722), [@masa_9713](https://twitter.com/masa_9713)
