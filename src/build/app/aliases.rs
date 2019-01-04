struct Alias<'help> {
    id: u64,
    name: &'help str,
    visible: bool
}

impl<'help> Alias {
    fn new<S: AsRef<&'help str>>(name: S) -> Self {
        let name = name.as_ref();
        Alias {
            id: hash(s),
            name,
            visible: false
        }
    }

    fn visible(mut self) -> Self {
        self.visible = true;
        self
    }

    fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }

    fn set_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl<'help> Default for Alias<'help> {
    fn default() -> Self {
        Alias {
            id: 0,
            name: "",
            visible: false
        }
    }
}

#[derive(Default)]
struct Aliases<'help>(Vec<Alias<'help>>);

impl<'help> Aliases<'help> {
    fn add_visible<S: AsRef<&'help str>>(&mut self, name: S) {
        self.0.push(Alias::new(name));
    }
}