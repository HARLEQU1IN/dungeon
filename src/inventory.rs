use {crate::items::Item, indoc::formatdoc};

pub(crate) const INVENTORY_SIZE: usize = 9;

#[derive(Debug, Default)]
pub(crate) struct Inventory([Item; INVENTORY_SIZE]);

impl Inventory {
    pub(crate) fn has_item(&self, item: Item) -> bool { self.0.contains(&item) }

    pub(crate) fn add_item(&mut self, item: Item) -> bool {
        for cell in self.0.iter_mut() {
            if *cell == Item::None {
                *cell = item;
                return true;
            }
        }
        false
    }

    pub(crate) fn remove_item(&mut self, item: Item) -> bool {
        for cell in self.0.iter_mut() {
            if *cell == item {
                *cell = Item::None;
                return true;
            }
        }
        false
    }

    pub(crate) fn get_pattern(&self) -> String {
        formatdoc! { r#"
            ╔═══╗╔═══╗╔═══╗
            ║{0}║║{1}║║{2}║
            ╚═══╝╚═══╝╚═══╝
            ╔═══╗╔═══╗╔═══╗
            ║{3}║║{4}║║{5}║
            ╚═══╝╚═══╝╚═══╝
            ╔═══╗╔═══╗╔═══╗
            ║{6}║║{7}║║{8}║
            ╚═══╝╚═══╝╚═══╝"#,
            self.0[0], self.0[1], self.0[2],
            self.0[3], self.0[4], self.0[5],
            self.0[6], self.0[7], self.0[8],
        }
    }
}
