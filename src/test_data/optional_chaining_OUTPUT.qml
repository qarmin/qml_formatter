import QtQuick 2.12

Column {
    required property var prop

    Text {
        text: "test"
    }

    function func(a, b) {
        return a ? prop?.foo : b
    }

    Row {
        Column {
            spacing: 4
            width: 10
            Repeater {
                model: [0, 2]

                Rect {
                    required property int index
                    property string foo: prop!.foo
                }
            }
        }
        Column {
            spacing: 4
            width: prop!.foo?.var
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
