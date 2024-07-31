import QtQuick 2.12

Column {
    required property string prop

    Text {
        text: "test"
    }

    function func(a, b){
        return `${a}|${index + 1}]`
    }

    Row {
        Column {
            spacing: 4
            width: 10
            Repeater {
                model: [0, 2]

                Rect {
                    required property int index
                    property string foo: func("a", index)
                }
            }
        }
        Column {
            spacing: 4
            width: 10
            Repeater {
                model: [0, 2]

                Rect {
                    required property int index
                    property string foo:`a|${index + 1}]`
                }
            }
        }
    }
}