# Yaazhi Framework

Yaazhi is a modular web framework written in Rust, inspired by the Moqui Framework.  
It focuses on XML-driven configuration and a clean runtime architecture.

> ⚠️ This project is in its **early stages** and under active development.

---

## Project Structure

- `yaazhi-core`: Core abstractions and shared logic
- `yaazhi-runtime`: Dev/runtime zone that handles XML config, databases, etc.
- `yaazhi-web`: Actix-based web server that loads configuration and starts the application

---

## Roadmap

✅ XML-based config loading  
🔜 Integrate [`sqlx`](https://github.com/launchbadge/sqlx) for async DB interaction  
🔜 Design and implement an **Entity Engine** similar to Moqui’s data layer

---

## Looking for Contributors

If you're passionate about Rust, frameworks, or XML-based configuration systems — we’d love your help!  
Feel free to explore the codebase and open issues or pull requests.

---

Built with ❤️ by a small team with big ideas.
