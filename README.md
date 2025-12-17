# TextBox

An elegant utility for constructing templates that can have text or images slotted into an existing layout for the Mapgen project.

---

## Overview

`TextBox` lets you define a **template image** and place named components on it:

* **Text components** (`TextArea`) for rendering text
* **Image components** (`ImageArea`) for overlaying images

Once the template is defined, you can fill in content at runtime with a `TextBoxRender`.

---

## Quickstart

```rs
let mut template: DynamicImage = /* ... */;
let text_box = TextBox::new(template)
    .image_component("flag", ImageArea::new((0, 0)))
    .text_component("name", TextArea::new(
        (0, 200),
        400,
        BLACK,
        FontArc::try_from_slice(include_bytes!("path/to/font.ttf"))?,
        48.0,
        Align::Center
    ));

let flag: DynamicImage = /* ... */;

let render = TextBoxRender::new()
    .image("flag", flag)
    .text("name", "Germany");
let img = text_box.render(render)?;
```