impl super::Daemon {
    pub fn exit(&mut self) {
        println!("[main] shutting down");

        self.settings.save();
    }
}
