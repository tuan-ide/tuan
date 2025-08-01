use std::cmp::min;

use log::{error, trace};
use tuan_rpc::{Line, Update};

#[derive(Debug, Clone, Default)]
pub struct LineCache {
    pub n_invalid_before: u64,
    pub lines: Vec<Option<Line>>,
    pub n_invalid_after: u64,
}

impl LineCache {
    pub fn new() -> Self {
        Self {
            n_invalid_before: 0,
            lines: Vec::new(),
            n_invalid_after: 0,
        }
    }

    pub fn get_total_line_count(&self) -> u64 {
        self.n_invalid_before + self.lines.len() as u64 + self.n_invalid_after
    }

    pub fn get_longest_line_length(&self) -> u64 {
        self.lines
            .iter()
            .map(|line| match line {
                None => 0,
                Some(line) => line.text.len() as u64,
            })
            .max()
            .unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        if self.get_total_line_count() == 1 {
            if let Some(line) = self.get_line(0) {
                if &line.text == "" {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_line(&self, line_number: u64) -> Option<&Line> {
        if line_number < self.n_invalid_before
            || line_number > self.n_invalid_before + self.lines.len() as u64
        {
            return None;
        }

        let index = (line_number - self.n_invalid_before) as usize;

        if let Some(line) = self.lines.get(index) {
            line.as_ref()
        } else {
            None
        }
    }

    pub fn handle_xi_update(&mut self, update: Update) {
        let mut new_invalid_before = 0;
        let mut new_lines: Vec<Option<Line>> = Vec::new();
        let mut new_invalid_after = 0;
        let mut old_index = 0 as u64;

        for operation in update.operations {
            let nb_affected_lines = operation.nb_affected_lines;

            match operation.operation_type {
                tuan_rpc::OperationType::Copy_ => {
                    trace!("copy n={}", nb_affected_lines);

                    for _ in 0..new_invalid_after {
                        new_lines.push(None)
                    }

                    new_invalid_after = 0;

                    let mut n_remaining = nb_affected_lines;
                    if old_index < self.n_invalid_before {
                        let invalid = min(nb_affected_lines, self.n_invalid_before - old_index);
                        if new_lines.is_empty() {
                            new_invalid_before += invalid;
                        } else {
                            new_invalid_after += invalid;
                        }
                        old_index += invalid;
                        n_remaining -= invalid;
                    }
                    if n_remaining > 0
                        && old_index < self.n_invalid_before + self.lines.len() as u64
                    {
                        let n_copy = min(
                            n_remaining,
                            self.n_invalid_before + self.lines.len() as u64 - old_index,
                        );
                        let start_ix = old_index - self.n_invalid_before;

                        for i in start_ix as usize..(start_ix + n_copy) as usize {
                            if self.lines[i].is_none() {
                                error!(
                                    "line {}+{}={}, a copy source is none",
                                    self.n_invalid_before,
                                    i,
                                    self.n_invalid_before + i as u64
                                );
                            }
                        }
                        new_lines.extend_from_slice(
                            &self.lines[start_ix as usize..(start_ix + n_copy) as usize],
                        );

                        old_index += n_copy;
                        n_remaining -= n_copy;
                    }
                    if new_lines.is_empty() {
                        new_invalid_before += n_remaining;
                    } else {
                        new_invalid_after += n_remaining;
                    }
                    old_index += n_remaining;
                }
                tuan_rpc::OperationType::Skip => {
                    trace!("skip nb_affected_lines={}", nb_affected_lines);
                    old_index += nb_affected_lines;
                }
                tuan_rpc::OperationType::Invalidate => {
                    trace!("invalidate nb_affected_lines={}", nb_affected_lines);
                    if new_lines.is_empty() {
                        new_invalid_before += nb_affected_lines;
                    } else {
                        new_invalid_after += nb_affected_lines;
                    }
                }
                tuan_rpc::OperationType::Update => todo!(),
                tuan_rpc::OperationType::Insert => {
                    for _ in 0..new_invalid_after {
                        new_lines.push(None)
                    }
                    trace!("ins nb_affected_lines={}", nb_affected_lines);
                    new_invalid_after = 0;
                    for line in operation.lines {
                        new_lines.push(Some(line.into()));
                    }
                }
            }

            self.n_invalid_before = new_invalid_before;
            self.lines = new_lines.clone();
            self.n_invalid_after = new_invalid_after;
        }
    }
}
