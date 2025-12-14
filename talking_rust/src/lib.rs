/*
Server runs.
Clients connect.
Clients send message to server.
Server boadcasts to all.
Like a big group chat.
*/

pub struct ServerMessage {
    from: usize,
    content: String
}

impl ServerMessage {
    pub fn new(from: usize, content: String) -> ServerMessage {
        ServerMessage {from: from, content: content}
    }

    pub fn get_from(&self) -> &usize {
        &self.from
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}
