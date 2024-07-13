fn main() {
    MainWindow::new().unwrap().run().unwrap();
}


//UI section
slint::slint! {
    import { Button, TextEdit } from "std-widgets.slint";
export component MainWindow inherits Window {
        width: 500px;
        height: 500px;
        title: "ADAFAS TEXT EDITORR";
        Rectangle {
            width: 500px;
            height: 500px;
            background: rgb(36,34,35);
            
            TextInput {
                width:500px;
                height:300px;
                text: "type here";
                color: white;
                x: 10px;
                y: 50px;
            }
            
            Button {
                text: "Clear!";
                x: 10px;
                y: 10px;        
            }

            Rectangle {
                background: grey;
                height: 30px;
                width: 500px;

                y: 470px;
                Text {
                    text: "Words:";
                    x:10px;

                }

                Text {
                    text: "UTF-8:";
                    x:450px;
                    
                }
                
                
            }
        
        }
    }
}