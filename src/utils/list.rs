use ratatui::{
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

pub fn create_list(
    title: String,
    items: Vec<String>,
    color: Color,
    bg_color: Color,
    borderflags: Borders,
) -> List<'static> {
    let mut list_items = Vec::<ListItem>::new();

    for item in items.iter() {
        list_items.push(create_list_item(item.to_string(), color));
    }

    let list_block = Block::default()
        .borders(borderflags)
        .border_type(BorderType::Rounded)
        .border_style(bg_color)
        .style(Style::default())
        .title(Line::from(title).centered());

    let list = List::new(list_items).block(list_block);

    list
}

fn create_list_item(text: String, color: Color) -> ListItem<'static> {
    let current_screen = ListItem::new(Text::styled(text, Style::default().fg(color)));
    current_screen
}
