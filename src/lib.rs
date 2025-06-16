#[allow(warnings)]
mod bindings;

use bindings::Guest;
use bindings::promptrs::core::types::Message;

struct Component;

impl Guest for Component {
	fn prune(messages: Vec<Message>, size: u64) -> Vec<Message> {
		let mut pruned = messages.iter().skip(1).rev().scan(0, |acc, msg| {
			if *acc > size as usize {
				return None;
			}
			*acc += match msg {
				Message::System(content) => *acc + content.len(),
				Message::User(content) => *acc + content.len(),
				Message::Assistant(content) => *acc + content.len(),
				Message::ToolCall((req, res)) => *acc + req.len() + res.len(),
				Message::Status((req, res)) => *acc + req.len() + res.len(),
			};
			Some(msg)
		});

		let mut messages = if let Some(pos) = pruned.position(is_status) {
			pruned
				.clone()
				.take(pos + 1)
				.chain(pruned.skip(pos + 1).filter(|msg| !is_status(msg)))
				.chain(messages.iter().take(1))
				.cloned()
				.collect::<Vec<_>>()
		} else {
			pruned.chain(messages.iter().take(1)).cloned().collect()
		};

		messages.reverse();
		messages
	}
}

fn is_status(msg: &Message) -> bool {
	if let Message::Status(_) = msg {
		true
	} else {
		false
	}
}

bindings::export!(Component with_types_in bindings);
