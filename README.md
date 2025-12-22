# Digital Ocean MCP Server (Zed Extension)

Expose DigitalOcean's MCP server inside Zed's Agent panel.

This extension downloads `@digitalocean/mcp` and launches it via Node to provide MCP tools for all DigitalOcean services.

## Setup

1. Install the extension in Zed (or load it as a dev extension).
2. Add your API token to the extension settings.
3. Enable the server in your assistant profile.

Example settings:

```jsonc
{
  "context_server": {
    "mcp-server-digitalocean": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "digitalocean_api_token": "YOUR_DIGITALOCEAN_API_TOKEN",
        "services": "apps,databases,droplets",
        "digitalocean_api_endpoint": ""
      }
    }
  }
}
```

Leave `services` empty to enable all services.

## Settings

- `digitalocean_api_token` (required): Your DigitalOcean personal access token.
- `services` (optional): Comma-separated services to enable (leave empty for all).
- `digitalocean_api_endpoint` (optional): Override the DigitalOcean API base URL.

## Remote MCP Servers

DigitalOcean also provides hosted MCP endpoints. If you prefer those, you can add them directly to Zed's MCP settings with the server URL and `Authorization: Bearer YOUR_API_TOKEN` header. See DigitalOcean's MCP documentation for the list of endpoints: https://github.com/digitalocean-labs/mcp-digitalocean for instructions.

## Development

Build the extension:

```sh
cargo build
```

## License

MIT
