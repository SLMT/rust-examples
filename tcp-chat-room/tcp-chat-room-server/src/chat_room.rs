
// TODO: Remove this attribute after finishing this project
#[derive(Debug)]
pub struct ChatRoom {
    name_list : Vec<String>
}

impl ChatRoom {
    pub fn new() -> ChatRoom {
        ChatRoom {
            name_list: Vec::new()
        }
    }

    pub fn join(&mut self, name: String) {
        self.name_list.push(name);
    }

    pub fn get_name_list(&self) -> Vec<String> {
        self.name_list.clone()
    }
}
