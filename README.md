# RustyTok 🦀

A privacy-friendly TikTok frontend written in Rust. All requests are proxied server-side, so TikTok never sees your IP or tracks you.

[![Open in CodeSandbox](https://codesandbox.io/static/img/play-codesandbox.svg)](https://codesandbox.io/p/github/arthiccc/rustytok)

## Features

- 🕵️ **Private**: All requests proxied through the server
- 🔒 **Secure**: Strong Content Security Policy blocks TikTok in browser
- ⚡ **Fast**: Rust + Axum = minimal overhead
- 📦 **Lightweight**: No JavaScript required, minimal CSS
- 🔗 **LibRedirect Ready**: Works as a drop-in TikTok replacement

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) 1.70+

### Run Locally

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/rustytok.git
cd rustytok

# Copy environment file
cp .env.example .env

# Run the server
cargo run

# Open http://localhost:3000
```

### Docker

```bash
docker build -t rustytok .
docker run -p 3000:3000 rustytok
```

## Usage

| URL Pattern | Description |
|-------------|-------------|
| `/` | Home page with search |
| `/@username` | View user profile |
| `/video/VIDEO_ID` | View single video |
| `/tag/hashtag` | View hashtag feed |

### LibRedirect Setup

Add your RustyTok instance to [LibRedirect](https://github.com/libredirect/libredirect) to automatically redirect TikTok links:

1. Install LibRedirect browser extension
2. Go to Settings → TikTok
3. Add your instance URL (e.g., `https://your-instance.com`)

### Redirector Setup

If using [Redirector](https://github.com/einaregilsson/Redirector):

```
Description: TikTok to RustyTok
Include pattern: (.*//.*)(tiktok.com)(.*)
Redirect to: https://your-instance.com$3
Pattern type: Regular Expression
```

## Development

```bash
# Run with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run

# Run tests
cargo test

# Build release
cargo build --release
```

## CodeSandbox

[![Open in CodeSandbox](https://codesandbox.io/static/img/play-codesandbox.svg)](https://codesandbox.io/p/github/arthiccc/rustytok)

This project is configured to run on CodeSandbox. Just click the button above!

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the AGPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [ProxiTok](https://github.com/pablouser1/ProxiTok)
- Part of the [LibRedirect](https://github.com/libredirect/libredirect) ecosystem

