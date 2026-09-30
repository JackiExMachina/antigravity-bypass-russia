# AGENTS.md — Antigravity Bypass Linux & Cross-Platform Protocol

Этот файл обязателен для любого агента (TARS / Gemini, Codex, Claude), работающего с проектом `antigravity-bypass-russia`.

---

## 🎯 Миссия проекта и Фокус Linux
Оригинальный проект `antigravity-bypass-russia` был сфокусирован на Windows (NRPT) и macOS (`/etc/resolver/`).
Задача нашего форка — обеспечить **первоклассную, нативную поддержку Linux (Ubuntu, Debian, Fedora, Arch)**:
1. **Сетевой уровень:** Интеграция со `systemd-resolved` (`resolvectl`) вместо неработающего NRPT. Запрещено слепо биндить `127.0.0.53:53` (порт уже занят `systemd-resolved` на Ubuntu!).
2. **Системная служба:** Нативный `systemd` unit (`/etc/systemd/system/antigravity-dns.service` или user service) вместо macOS LaunchDaemon.
3. **Обнаружение путей:** Полная поддержка путей Antigravity 2.0 (`~/.local/share/antigravity-2.0`, `Antigravity-x64`, распакованные архивы, `app.asar`).
4. **Бинарный патчер:** Поддержка System V AMD64 ABI сигнатур для Linux ELF бинарников `language_server` / `language_server_linux_x64`.

---

## 🛡️ Архитектурные инварианты
1. **Безопасность и чистый откат:** Любая модификация файлов обязана создавать `.bak` копию. Команда `rollback` должна на 100% восстанавливать систему в девственное состояние.
2. **Никаких зависаний в headless/non-interactive режимах:** Проверять `isatty` перед вызовами `pause()` и `sudo`, чтобы скрипты и субагенты не блокировались.
3. **Конфиденциальность:** Никаких логов пользовательского кода, ключей или персональных данных.

---

## 🛠️ Стек и сборка
* Язык: Rust (2021 edition)
* Сборка: `cargo build --release`
* Проверка: `cargo check` / `cargo clippy`
