import fs from 'node:fs';

const DISCORD_API_VERSION = '/v9';
const buildDiscordAPIUrl = (ver = '') => {
    return [
        'http://',
        'discord.com',
        '/api',
        `${ver}`,
        '/applications',
        '/detectable'
    ];
}
const buildDetectableAPIUrl = (ver = '') => {
    return `https://markterence.github.io/discord-quest-completer/detectable${ver}.json`;
}

const data = {
    updatedAt: new Date().valueOf(),
    DISCORD_DETECTABLE_API_URL_VERSIONED: buildDiscordAPIUrl(DISCORD_API_VERSION).join(''),
    DISCORD_DETECTABLE_DISCORD_API_URL: buildDiscordAPIUrl().join(''),
    DETECTABLE_API_URL_MIRROR_1: buildDetectableAPIUrl(),
    DETECTABLE_API_URL_MIRROR_1_VERSIONED: buildDetectableAPIUrl('_v9'),
}

fs.writeFileSync('./main.json', JSON.stringify(data, null, 2), { encoding: 'utf-8' });