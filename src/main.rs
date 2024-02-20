slint::slint! {
    component MemoryTile inherits Rectangle {
    width: 64px;
    height: 64px;
    background: #3960D5;

    Image {
        source: @image-url("icons/cancel48.png");
        width: parent.width;
        height: parent.height;
        }
    }

    export component MainWindow inherits Window {
        MemoryTile {}
        // Text {
        //     text: "hello world";
        //     color: green;
        // }
    }
}

fn main() {
    println!("Hello, world!");
    MainWindow::new().unwrap().run().unwrap();
}
