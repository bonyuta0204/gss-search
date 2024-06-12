use std::{fmt::Display, sync::Arc};

use inquire::Select;
use skim::{
    prelude::{unbounded, Key, SkimOptionsBuilder},
    Skim, SkimItemReceiver, SkimItemSender,
};

pub fn interactive_select<S: Display>(data: Vec<S>) -> Result<S, inquire::InquireError> {
    Select::new("Select Row", data).prompt()
}

pub fn skim_select<C: skim::SkimItem + Clone + 'static>(data: Vec<C>) -> Option<C> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .bind(vec!["Enter:accept"])
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in data {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| match out.final_key {
            Key::Enter => out.selected_items,
            _ => Vec::new(),
        })
        .unwrap_or_else(Vec::new);

    let selection = selected_items.into_iter().next()?;

    let selection = (*selection).as_any().downcast_ref::<C>()?;

    let selection = selection.clone();

    Some(selection)
}
