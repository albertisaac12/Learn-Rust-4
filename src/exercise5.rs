/*
Let's model a real-time chat system where users can
share audio and video files.
 
Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation.
 
Define a ChatMessage struct with two fields: `content`
and `time`. The struct should define one generic type, T,
which will be the type of the `content` field.
The `time` field should always be a String.
Derive a Debug implementation.
 
Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile".
 
Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct.
 
In `main`, create a ChatMessage with `content` set to a
string slice.
 
Create another ChatMessage with `content` set to a String.
 
Create another ChatMessage with `content' set to a
DigitalContent variant.
 
Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum.
 
Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time : String
}

fn main() {
    let m1 = ChatMessage {
        content: "hahahah",
        time: "10 AM".to_string()
    };
    let m2 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: "10 AM".to_string()
    };
    let m3 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: "10 AM".to_string()
    };

    m2.consume_entertainment();
    m1.retrieve_time();
    
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("The content is : {:?}",self.content);
    }
}

impl<L> ChatMessage<L> {
    fn retrieve_time(&self)-> String {
        self.time.clone() // here if there was no clone it would mean that we were trying to move a value behind a shared reference
    }
}