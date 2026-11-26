# Aplicación básica de contador

Este tutorial explica cómo crear una aplicación básica de contador con Ratatui.

## Crear un nuevo proyecto

Crea un nuevo proyecto de Rust y ábrelo en tu editor:

```shell
cargo new ratatui-counter-app
cd ratatui-counter-app
$EDITOR .
```

Añade las dependencias de las bibliotecas Ratatui y Crossterm (consulta backends para obtener más información sobre por qué utilizamos Crossterm).

```shell
cargo add ratatui crossterm
```

El archivo `Cargo.toml` tendrá ahora el siguiente contenido en la sección de dependencias:

```toml
[dependencies]
ratatui = "0.30.2"
crossterm = "0.29.0"
```

## Configuración de la aplicación

### Importaciones principales

En `main.rs`, añade las importaciones necesarias de Ratatui y Crossterm. Las utilizaremos más adelante en este tutorial. En los tutoriales, generalmente utilizamos importaciones globales (`wildcard imports`) para simplificar el código, pero puedes utilizar importaciones explícitas si ese es tu estilo preferido.

```rust
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
```

  > **Caution:**
  > Algunos editores eliminan automáticamente las importaciones que no se utilizan. Por tanto, si encuentras errores relacionados con tipos inexistentes, etc., asegúrate de que estas importaciones estén presentes. Comprueba también que las importaciones compartidas, como `Stylize`, procedan de Ratatui y no de Crossterm, para evitar conflictos.

### Función principal

Un patrón habitual en la mayoría de las aplicaciones desarrolladas con Ratatui consiste en:

1. Inicializar el terminal.
2. Ejecutar la aplicación en un bucle hasta que el usuario decida salir.
3. Restaurar el terminal a su estado original.

La función `main` llama a `ratatui::run()`. Esta función configura el terminal mediante las llamadas a los métodos `ratatui::init` y `ratatui::restore`, y después crea y ejecuta la aplicación (`App`), que definiremos más adelante.

La función `ratatui::run()` pospone la propagación del resultado devuelto por `App::run()` hasta que el terminal se haya restaurado. De esta manera, cualquier error (`Error`) podrá mostrarse al usuario después de que la aplicación haya finalizado.

Completa la función principal:

rust

```rust
fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
```

### Estado de la aplicación

La aplicación de contador necesita almacenar una pequeña cantidad de información: un contador y una variable que indique si la aplicación debe finalizar.

El contador será un entero sin signo de 8 bits (`u8`), mientras que la variable de salida puede ser un simple booleano (`bool`). Las aplicaciones que tengan más de un estado o modo principal podrían utilizar, en su lugar, una enumeración (`enum`) para representar esta variable.

Crea una estructura `App` que represente el estado de tu aplicación:

rust

```rust
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}
```

La llamada a `App::default()` creará una instancia de `App` inicializada con `counter` establecido en `0` y `exit` establecido en `false`.

### Bucle principal de la aplicación

La mayoría de las aplicaciones tienen un bucle principal que se ejecuta hasta que el usuario decide salir. En cada iteración del bucle se dibuja un único fotograma mediante una llamada a `Terminal::draw()` y, a continuación, se actualiza el estado de la aplicación.

Crea un bloque `impl` para `App` con un nuevo método `run`, que actuará como bucle principal de la aplicación:

rust

```rust
impl App {

    /// ejecuta el bucle principal de la aplicación hasta que el usuario sale
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}
```

## Mostrar la aplicación

### Renderizar un fotograma

Para renderizar la interfaz de usuario, una aplicación llama a `Terminal::draw()` con una clausura que recibe un `Frame`.

El método más importante de `Frame` es `render_widget()`, que permite renderizar cualquier tipo que implemente el trait `Widget`, como `Paragraph`, `List`, etc.

Implementaremos el trait `Widget` para la estructura `App`, de modo que todo el código relacionado con el renderizado esté organizado en un único lugar. Esto nos permitirá llamar a `Frame::render_widget()` pasando la aplicación como argumento dentro de la clausura proporcionada a `Terminal::draw()`.

En primer lugar, añade un nuevo bloque `impl Widget for &App`. Implementamos este trait sobre una referencia al tipo `App`, ya que la función de renderizado no modificará ningún estado y queremos poder seguir utilizando la aplicación después de la llamada a `draw`.

La función de renderizado creará un bloque con un título, un texto de instrucciones en la parte inferior y algunos bordes. Dentro del bloque se renderizará un widget `Paragraph` con el estado de la aplicación (el valor del campo `counter` de `App`).

Tanto el bloque como el párrafo ocuparán todo el espacio disponible del widget:

```rust
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
```

A continuación, renderiza la aplicación como un widget:

```rust
impl App {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}
```

  > **NOTE:**
  > La clausura (método anónimo) que se pasa al método `Terminal::draw()` debe renderizar toda la interfaz de usuario.
Solo debes llamar al método `draw` una vez por cada iteración del bucle principal de tu aplicación.
Consulta las preguntas frecuentes para obtener más información.


### Probar el resultado de la interfaz de usuario

Para comprobar cómo mostrará Ratatui el widget cuando se llame a `render`, puedes renderizar la aplicación en un búfer durante una prueba.

Añade el siguiente módulo `tests` a `main.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}
```

Para ejecutar esta prueba, introduce el siguiente comando en tu terminal:

```shell
cargo test
```

Deberías obtener el siguiente resultado:

```text
running 1 test
test tests::render ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Interactividad

La aplicación debe aceptar los eventos que recibe del usuario a través de la entrada estándar.

El único tipo de evento que necesita gestionar esta aplicación son los eventos de teclado. Para obtener información sobre otros eventos disponibles, consulta la documentación del [módulo de eventos de Crossterm](<https://docs.rs/crossterm/latest/crossterm/event/index.html?utm_source=chatgpt.com>). Estos incluyen eventos de redimensionamiento de ventana, cambio de foco, pegado de texto y eventos del ratón.

En aplicaciones más avanzadas, los eventos también podrían proceder del sistema, de la red o de otras partes de la propia aplicación.

### Gestionar eventos

El método `handle_events` que definiste anteriormente es el lugar donde la aplicación esperará y gestionará los eventos que reciba de Crossterm.

Actualiza el método `handle_events` que definiste anteriormente:

```rust
impl App {

    // -- código omitido --

    /// actualiza el estado de la aplicación en función de la entrada del usuario
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // es importante comprobar que el evento sea una pulsación de tecla,
            // ya que Crossterm también emite eventos de liberación y repetición
            // de teclas en Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}
```

  >**NOTE:**
  > La función [`event::read`](<https://docs.rs/crossterm/latest/crossterm/event/fn.read.html?utm_source=chatgpt.com>) bloquea la ejecución hasta que se produce un evento. 
  > Si tu aplicación necesita realizar otras tareas además de gestionar la interfaz de usuario, debería comprobar si existe algún evento pendiente mediante una llamada a [`event::poll`](<https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html?utm_source=chatgpt.com>), utilizando un tiempo de espera adecuado para tu aplicación.
  > Este tema se tratará con más detalle en un capítulo posterior.

### Gestionar eventos del teclado

Tu aplicación de contador actualizará los campos de la estructura `App` en función de la tecla que se haya pulsado.

El evento de teclado tiene dos campos de especial interés para esta aplicación:

- `kind`: Es importante comprobar que su valor sea `KeyEventKind::Press`. De lo contrario, la aplicación podría recibir eventos duplicados (al pulsar una tecla, al repetirla y al soltarla).
- `code`: Representa el código `KeyCode` correspondiente a la tecla específica que se ha pulsado.

Añade un método `handle_key_event` a `App` para gestionar los eventos del teclado.

```rust
impl App {

    // -- código omitido --

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }
}
```

A continuación, añade algunos métodos para gestionar las actualizaciones del estado de la aplicación.

Normalmente es una buena práctica definir estos métodos dentro de la propia aplicación, en lugar de incluir toda la lógica directamente en la sentencia `match`. Esto facilita la creación de pruebas unitarias para comprobar el comportamiento de la aplicación independientemente de los eventos.

```rust
impl App {

    // -- código omitido --

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}
```

  > **Caution**
  > Normalmente, tu aplicación debería evitar los errores de tipo panic. Sin embargo, vamos a dejar aquí un error de desbordamiento para mostrar cómo gestionar los errores en la siguiente sección.
  > Una aplicación real podría utilizar `saturating_sub` y `saturating_add` para evitar este tipo de errores.

### Probar los eventos del teclado

Separar la gestión de los eventos del teclado en una función independiente, como hemos hecho aquí, facilita las pruebas de la aplicación sin necesidad de simular el terminal.

Puedes escribir pruebas que envíen eventos de teclado y comprueben cómo afectan al estado de la aplicación.

Añade las siguientes pruebas para `handle_key_event` dentro del módulo `tests`:

```rust
#[cfg(test)]
mod tests {

    // -- código omitido --

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}
```

Ejecuta las pruebas:

```shell
cargo test
```

Deberías obtener el siguiente resultado:

```text
running 2 tests
test tests::handle_key_event ... ok
test tests::render ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## La aplicación terminada

Una vez integrado todo lo anterior, deberías tener los siguientes archivos:

```rust
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// ejecuta el bucle principal de la aplicación hasta que el usuario sale
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// actualiza el estado de la aplicación en función de la entrada del usuario
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // es importante comprobar que el evento sea una pulsación de tecla,
            // ya que Crossterm también emite eventos de liberación y repetición
            // de teclas en Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}
```

### Ejecutar la aplicación

Asegúrate de guardar todos los archivos y de que las importaciones indicadas anteriormente sigan estando al principio del archivo (algunos editores eliminan automáticamente las importaciones que no se utilizan).

Ahora ejecuta la aplicación:

```shell
cargo run
```

Verás la siguiente interfaz de usuario:

Pulsa las teclas de flecha Izquierda y Derecha para interactuar con el contador. Pulsa Q para salir.

Observa qué ocurre cuando pulsas Izquierda mientras el contador está en 0.

En una consola de Mac o Linux puedes ejecutar el comando `reset` para restaurar la consola.

En una consola de Windows, es posible que tengas que reiniciarla para solucionar el problema.

En la siguiente sección de este tutorial, dedicada a la gestión de errores, aprenderemos a gestionar correctamente esta situación.

## Conclusión

Al comprender la estructura y los componentes utilizados en esta sencilla aplicación de contador, ya tienes una base para explorar cómo crear interfaces de terminal más complejas utilizando `ratatui`.
