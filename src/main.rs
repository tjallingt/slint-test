slint::slint! {
    export component HelloWorld {
        Text {
            text: "Hello World!";
            horizontal-alignment: center;
        }
    }
}


fn main() {
    HelloWorld::new().unwrap().run().unwrap();
}
