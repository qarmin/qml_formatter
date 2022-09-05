

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

