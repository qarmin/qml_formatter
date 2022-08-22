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