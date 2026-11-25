# OrbitBot

## Discord Setup

This bot uses the `Message Content Intent` to read `!ping` messages. If you see a `Gateway(DisallowedGatewayIntents)` error, enable **Bot → Privileged Gateway Intents → Message Content Intent** for your application in the Discord Developer Portal, save the changes, and restart the bot. Without this intent, the bot cannot read regular message content, so the `!ping` command will not work.