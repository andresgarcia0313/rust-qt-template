import QtQuick
import QtQuick.Controls
import UI

Window {
    visible: true
    width: 250; height: 150
    title: "Performance Pro"
    color: "#f0f0f0"

    MainComponent { id: core }

    Column {
        anchors.centerIn: parent
        spacing: 10

        Text {
            text: core.text
            font { pixelSize: 20; bold: true }
            color: "#333"
        }

        Button {
            text: "Run"
            onClicked: core.click()
        }
    }
}
