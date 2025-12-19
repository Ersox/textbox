# TextBox

An elegant utility for constructing templates that can have text or images slotted into an existing layout.

---

## Overview

`TextBox` lets you define a **template image** and place named components on it:

- **Text components** (`TextArea`) for rendering text
- **Image components** (`ImageArea`) for overlaying images

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

--

## Design

`TextBox` is dependent on the `image`, `imageproc` and `ab_glyph` crates. The goal is to make it easy to create reusable containers for slotting images and text into predictable slots.

Text fields allow for a subset of markdown styles if the provided font is a _variable font_, like Roboto Flex. Bold, italic, and colored styles are usable. Colored styles use Markdown link syntax, i.e. `[Colored Text](#FF0000)`, since static images could never be clickable links anyway. If the font doesn't support those features, all styles will be ignored.