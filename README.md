# QML Formatter
This repository is simple project which implements basic qml formatter, which is used internally inside my projects.

It aims to format QML files in unified way.

I don't expect to add any options to configure it in runtime.

For now formatter can:
- remove empty lines from start and end of file
- connect multiple empty lines into one
- add at the end empty line
- add spaces before and after ? :
- format if/else/for oneliners
- handle strings inside ', " and `
- format classes inside other classes

Example  
Before:
```qml


import QtQuick
import "../../commons/elements"
import "../preparationScreen" as ExamCommons


Text {
    id     : root
    signal pressed()
    image       :       "qrc://image.svg"
    layer.effect: ElevationEffect 
    
    {
        elevation: elevation
    }
    
    property var able: is_able? no_able: very_able
}



```
After
```qml
import QtQuick
import "../../commons/elements"
import "../preparationScreen" as ExamCommons

Text {
    id: root
    signal pressed()
    image: "qrc://image.svg"
    layer.effect: ElevationEffect {
        elevation: elevation
    }

    property var able: is_able ? no_able : very_able
}

```

