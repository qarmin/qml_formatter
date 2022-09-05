# QML Formatter
This repository is simple project which implements basic qml formatter, which is used internally inside my projects.

It is alternative for default QML formatter which sometimes broke ours code, so we want to create own app to format files only with needed rules.

I don't expect to add any options to configure it in runtime.

For now formatter can:
- remove empty lines from start and end of file
- connect multiple empty lines into one
- add at the end empty line
- add spaces before and after ? :
- format if/else/for oneliners
- handle strings inside ', " and `
- format classes inside other classes

## Usage
Formatter checks for qml files recursively inside provided folders:
```commandline
qml_formatter folder_with_files_to_check folder_with_files_to_check2 -efolder_with_excluded_files -efolder_with_excluded_files2
```
e.g.
```commandline
qml_formatter /home/a /home/b -e/home/a/not_to_check -e/home/a/completelly
```
will list all files inside folder `/home/a` and `/home/b` that are not inside `/home/a/not_to_check` and `/home/a/completelly`.

By default app runs in interactive mode which require to confirm formatting, but there is additional argument `NO_QUESTION` which format files without questions e.g.
```commandline
qml_formatter /home/a NO_QUESTION
```

## Conversion example
Before:
```qml


import QtQuick
import "../../commons/elements"
import "../preparationScreen" as ExamCommons


Text {
    id     : root
    property var able: is_able? no_able: very_able
    signal pressed()
    image       :       "qrc://image.svg"
    layer.effect: ElevationEffect 
    
    {
        elevation: elevation
    }

}


```
After
```qml
import QtQuick
import "../../commons/elements"
import "../preparationScreen" as ExamCommons

Text {
    id: root
    property var able: is_able ? no_able : very_able
    signal pressed()
    image: "qrc://image.svg"
    layer.effect: ElevationEffect {
        elevation: elevation
    }
}

```

