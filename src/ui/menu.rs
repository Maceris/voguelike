pub trait DrawMenu {
    fn draw_menu();
}

pub struct TabMenu {
    pub tabs: Vec<String>,
    pub selected_tab: u8, 
}

pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>, 
}

pub struct TableRow {
    pub values: Vec<String>,
}
