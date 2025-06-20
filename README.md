# Yaazhi Framework

![Yaazhi Logo](/641554a6-52fa-47fd-b96f-a0da2b88ef64-removebg-preview%20(2).png)

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

# Discord https://discord.gg/4KUqWsrvVu

Built with ❤️ by a small team with big ideas.
