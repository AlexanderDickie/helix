use helix_core::{Transaction, Rope};

use crate::{
    compositor::{Callback, Component, Context, Event, EventResult},
    ctrl, key,
};

pub struct CopilotCompletionPicker{
    original: Rope,
    cur: usize,
    transactions: Vec<Transaction>,
    id: &'static str,
}

impl CopilotCompletionPicker {
    // need to return the state
    pub fn new(original: Rope, transactions: Vec<Transaction>) 
        -> Option<(Self, Transaction)> 
    {
        if transactions.is_empty() {
            return None;
        }

        let first = transactions[0].clone();
        Some((
            Self {
                original,
                cur: 0,
                transactions,
                id: "copilot-picker",
            },
            first,
        ))
    }
    // returns (prev_applied_transaction, next_transaction)
    pub fn next(&mut self) -> Option<(Transaction, Transaction)> {
        if self.cur == self.transactions.len() - 1 {
            return None;
        }
        self.cur += 1;
        Some((
            self.transactions[self.cur-1].clone(),
            self.transactions[self.cur].clone()
        ))
    }

    fn prev(&mut self) -> Option<(Transaction, Transaction)>{
        if self.cur == 0 {
            return None;
        }
        self.cur -= 1;
        Some((
            self.transactions[self.cur + 1].clone(),
            self.transactions[self.cur].clone()
        ))
    }
}

impl Component for CopilotCompletionPicker {
    fn render(&mut self, _: helix_view::graphics::Rect, _: &mut tui::buffer::Buffer, _: &mut Context) {
       () 
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.id)
    }

    fn handle_event(&mut self, event: &Event, _: &mut Context) -> EventResult {
        let key = match event {
            Event::Key(event) => *event,
            _ => return EventResult::Ignored(None),
        };

        fn update_picker(transactions: Option<(Transaction, Transaction)>, original: &Rope) 
        -> EventResult 
        {
            match transactions {
                None => EventResult::Consumed(None),
                Some((prev, next)) => {
                    let original = original.clone();

                    let undo_then_apply: Callback = Box::new(move |_, context| {
                        let (view, doc) = current!(context.editor);

                        let invert = prev.invert(&original);
                        doc.apply(&invert, view.id);

                        doc.apply(&next, view.id);
                    });

                    EventResult::Consumed(Some(undo_then_apply))
                }
            }
        }

        match key {
            ctrl!('n') => update_picker(self.next(), &self.original),
            ctrl!('m') => update_picker(self.prev(), &self.original),
            key!(Enter) => {
                let id = self.id.clone();
                let remove_picker: Callback = Box::new(move |compositor, _| {
                    compositor.remove(id);
                });

                EventResult::Consumed(Some(remove_picker))
            },
            key!(Esc) => {
                let cur = self.transactions[self.cur].clone();
                let id = self.id.clone();
                let original = self.original.clone();

                let undo_remove_picker: Callback = Box::new(move |compositor, context| {
                    // undo cur transaction
                    let (view, doc) = current!(context.editor);
                    let invert = cur.invert(&original);
                    doc.apply(&invert, view.id);

                    compositor.remove(id);
                });

                EventResult::Consumed(Some(undo_remove_picker))
            },

            _ => EventResult::Consumed(None),
        }
    }
}
