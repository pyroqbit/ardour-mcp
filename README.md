# Ardour MCP Server

**English** | [Русский](#Русский)

## English

### Overview

`ardour_mcp_server` is a server that implements the Model Context Protocol (MCP) to provide an interface for controlling Ardour (a digital audio workstation) via Open Sound Control (OSC) messages. It allows external applications or scripts to interact with Ardour, automating tasks, and extending its functionality.

This server listens for MCP requests, translates them into Ardour-specific OSC messages, and sends them to a running Ardour instance. It also includes a `stdio_client_tester` binary for testing the server's functionality.

### Features

*   Control Ardour transport (play, stop, locate, speed, etc.).
*   Manage tracks (mute, solo, record enable, gain, trim).
*   Execute general Ardour menu actions via `/ardour/access_action`.
*   Read Ardour state (e.g., playback state).
*   MCP-compliant interface for easy integration.

### Prerequisites

*   **Rust**: Ensure you have Rust installed. You can get it from [rust-lang.org](https://www.rust-lang.org/).
*   **Ardour**: A running instance of Ardour 6.x, 7.x, 8.x or 9.x.
*   **Ardour OSC Configuration**:
    *   In Ardour, go to `Edit > Preferences > Control Surfaces`.
    *   Enable "OSC".
    *   Ensure the "Listen Port (UDP)" is set to `3819` (or update the server configuration if you use a different port).
    *   The server sends messages to `127.0.0.1:3819` by default.

### Building the Server

1.  Navigate to the `MCP/ardour_mcp_server` directory:
    ```bash
    cd MCP/ardour_mcp_server
    ```
2.  Build the server and the tester:
    ```bash
    cargo build
    ```
    You can also build in release mode for better performance:
    ```bash
    cargo build --release
    ```
    The binaries will be located in `target/debug/` or `target/release/`.

### Running the Server

You can run the server directly using:

```bash
cargo run --bin ardour_mcp_server
```

Or, after building, run the executable:

```bash
./target/debug/ardour_mcp_server 
# or ./target/release/ardour_mcp_server
```

The server will start and await MCP connections (e.g., from the `stdio_client_tester`).

### Running the Client Tester

The `stdio_client_tester` communicates with the server over standard input/output. It's a good way to test if the server is correctly interacting with Ardour.

1.  Ensure Ardour is running and configured for OSC.
2.  Ensure the `ardour_mcp_server` is **not** running separately, as the tester will launch it as a child process.
3.  Run the tester:
    ```bash
    cargo run --bin stdio-client-tester
    ```
    Logs for the tester will be in `MCP/ardour_mcp_server/logs/stdio_client_tester.log`.
    Logs for the server (when run by the tester) will be in `MCP/ardour_mcp_server/logs/ardour_mcp_server.log`.

### Implemented OSC Commands

For a list of implemented OSC commands and their corresponding MCP tools, please see [doc/osc_ardour_list.md](doc/osc_ardour_list.md).

The file [doc/index.html](doc/index.html) contains a reference of Ardour's general OSC capabilities, extracted from the official Ardour manual. Note that not all commands listed in `index.html` are directly implemented as distinct tools in this server; many can be accessed using the generic `access_action` tool.

---

## Русский

### Обзор

`ardour_mcp_server` — это сервер, реализующий Model Context Protocol (MCP) для предоставления интерфейса управления цифровой звуковой рабочей станцией Ardour посредством сообщений Open Sound Control (OSC). Он позволяет внешним приложениям или скриптам взаимодействовать с Ardour, автоматизировать задачи и расширять его функциональность.

Сервер принимает MCP-запросы, переводит их в OSC-сообщения, специфичные для Ardour, и отправляет их запущенному экземпляру Ardour. Проект также включает исполняемый файл `stdio_client_tester` для тестирования функциональности сервера.

### Возможности

*   Управление транспортом Ardour (воспроизведение, остановка, перемотка, скорость и т.д.).
*   Управление дорожками (mute, solo, включение записи, усиление, подстройка уровня).
*   Выполнение общих действий из меню Ardour через `/ardour/access_action`.
*   Чтение состояния Ardour (например, состояние воспроизведения).
*   Интерфейс, совместимый с MCP, для легкой интеграции.

### Требования

*   **Rust**: Убедитесь, что у вас установлен Rust. Его можно скачать с [rust-lang.org](https://www.rust-lang.org/).
*   **Ardour**: Запущенный экземпляр Ardour 6.x, 7.x, 8.x или 9.x.
*   **Настройка OSC в Ardour**:
    *   В Ardour перейдите в `Edit > Preferences > Control Surfaces` (или `Правка > Параметры > Управляющие устройства`).
    *   Включите "OSC".
    *   Убедитесь, что "Listen Port (UDP)" (Порт приема UDP) установлен на `3819` (или обновите конфигурацию сервера, если используете другой порт).
    *   По умолчанию сервер отправляет сообщения на `127.0.0.1:3819`.

### Сборка сервера

1.  Перейдите в директорию `MCP/ardour_mcp_server`:
    ```bash
    cd MCP/ardour_mcp_server
    ```
2.  Соберите сервер и тестер:
    ```bash
    cargo build
    ```
    Вы также можете выполнить сборку в релизном режиме для лучшей производительности:
    ```bash
    cargo build --release
    ```
    Исполняемые файлы будут находиться в `target/debug/` или `target/release/`.

### Запуск сервера

Вы можете запустить сервер напрямую с помощью:

```bash
cargo run --bin ardour_mcp_server
```

Или, после сборки, запустите исполняемый файл:

```bash
./target/debug/ardour_mcp_server 
# или ./target/release/ardour_mcp_server
```

Сервер запустится и будет ожидать MCP-соединений (например, от `stdio_client_tester`).

### Запуск клиент-тестера

`stdio_client_tester` взаимодействует с сервером через стандартные потоки ввода/вывода. Это хороший способ проверить, корректно ли сервер взаимодействует с Ardour.

1.  Убедитесь, что Ardour запущен и настроен для OSC.
2.  Убедитесь, что `ardour_mcp_server` **не** запущен отдельно, так как тестер запустит его как дочерний процесс.
3.  Запустите тестер:
    ```bash
    cargo run --bin stdio-client-tester
    ```
    Логи тестера будут находиться в `MCP/ardour_mcp_server/logs/stdio_client_tester.log`.
    Логи сервера (при запуске через тестер) будут находиться в `MCP/ardour_mcp_server/logs/ardour_mcp_server.log`.

### Реализованные OSC-команды

Список реализованных OSC-команд и соответствующих им MCP-инструментов смотрите в файле [doc/osc_ardour_list.md](doc/osc_ardour_list.md).

Файл [doc/index.html](doc/index.html) содержит справочную информацию об общих возможностях OSC в Ardour, извлеченную из официального руководства Ardour. Обратите внимание, что не все команды, перечисленные в `index.html`, напрямую реализованы как отдельные инструменты в этом сервере; ко многим из них можно получить доступ с помощью универсального инструмента `access_action`. 