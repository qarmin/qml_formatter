# QML Formatter
This repository is simple project which implements basic qml formatter.

It aims to format QML files in unified way.

I don't expect to add any options to configure in runtime what which exactly rules will run

For now formatter can:
- remove empty lines from start and end of file
- connect multiple empty lines into one


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
}
```

