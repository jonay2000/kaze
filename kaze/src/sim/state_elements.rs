use crate::graph;

use std::collections::HashMap;

pub(super) struct Register<'a> {
    pub data: &'a graph::RegisterData<'a>,
    pub value_name: String,
    pub next_name: String,
}

pub(super) struct Mem<'a> {
    pub mem: &'a graph::Mem<'a>,
    pub mem_name: String,
    pub read_signal_names: HashMap<(&'a graph::Signal<'a>, &'a graph::Signal<'a>), ReadSignalNames>,
    pub write_address_name: String,
    pub write_value_name: String,
    pub write_enable_name: String,
}

pub struct ReadSignalNames {
    pub address_name: String,
    pub enable_name: String,
    pub value_name: String,
}

pub(super) struct StateElements<'a> {
    pub mems: HashMap<&'a graph::Mem<'a>, Mem<'a>>,
    pub regs: HashMap<&'a graph::Signal<'a>, Register<'a>>,
}

impl<'a> StateElements<'a> {
    pub fn new() -> StateElements<'a> {
        StateElements {
            mems: HashMap::new(),
            regs: HashMap::new(),
        }
    }

    // TODO: Move this to ctor and iterate over input module outputs there?
    pub fn gather(
        &mut self,
        signal: &'a graph::Signal<'a>,
        signal_reference_counts: &mut HashMap<&'a graph::Signal<'a>, u32>,
    ) {
        // TODO: Do we even need this with just the one member?
        struct Frame<'a> {
            signal: &'a graph::Signal<'a>,
        }

        let mut frames = Vec::new();
        frames.push(Frame { signal });

        while let Some(frame) = frames.pop() {
            let signal = frame.signal;

            let reference_count = signal_reference_counts
                .entry(signal)
                .or_insert(0);
            *reference_count += 1;

            if *reference_count > 1 {
                continue;
            }

            match signal.data {
                graph::SignalData::Lit { .. } => (),

                graph::SignalData::Input { data } => {
                    if let Some(driven_value) = data.driven_value.borrow().clone() {
                        frames.push(Frame {
                            signal: driven_value,
                        });
                    }
                }
                graph::SignalData::Output { data } => {
                    frames.push(Frame {
                        signal: data.source,
                    });
                }

                graph::SignalData::Reg { data } => {
                    let key = signal;
                    let value_name = format!("__reg_{}_{}", data.name, self.regs.len());
                    let next_name = format!("{}_next", value_name);
                    self.regs.insert(
                        key,
                        Register {
                            data,
                            value_name,
                            next_name,
                        },
                    );
                    frames.push(Frame {
                        signal: data.next.borrow().unwrap(),
                    });
                }

                graph::SignalData::UnOp { source, .. } => {
                    frames.push(Frame {
                        signal: source,
                    });
                }
                graph::SignalData::SimpleBinOp { lhs, rhs, .. } => {
                    frames.push(Frame {
                        signal: lhs,
                    });
                    frames.push(Frame {
                        signal: rhs,
                    });
                }
                graph::SignalData::AdditiveBinOp { lhs, rhs, .. } => {
                    frames.push(Frame {
                        signal: lhs,
                    });
                    frames.push(Frame {
                        signal: rhs,
                    });
                }
                graph::SignalData::ComparisonBinOp { lhs, rhs, .. } => {
                    frames.push(Frame {
                        signal: lhs,
                    });
                    frames.push(Frame {
                        signal: rhs,
                    });
                }
                graph::SignalData::ShiftBinOp { lhs, rhs, .. } => {
                    frames.push(Frame {
                        signal: lhs,
                    });
                    frames.push(Frame {
                        signal: rhs,
                    });
                }

                graph::SignalData::Mul { lhs, rhs, .. } => {
                    frames.push(Frame {
                        signal: lhs,
                    });
                    frames.push(Frame {
                        signal: rhs,
                    });
                }
                graph::SignalData::MulSigned { lhs, rhs, .. } => {
                    frames.push(Frame {
                        signal: lhs,
                    });
                    frames.push(Frame {
                        signal: rhs,
                    });
                }

                graph::SignalData::Bits { source, .. } => {
                    frames.push(Frame {
                        signal: source,
                    });
                }

                graph::SignalData::Repeat { source, .. } => {
                    frames.push(Frame {
                        signal: source,
                    });
                }
                graph::SignalData::Concat { lhs, rhs, .. } => {
                    frames.push(Frame {
                        signal: lhs,
                    });
                    frames.push(Frame {
                        signal: rhs,
                    });
                }

                graph::SignalData::Mux {
                    cond,
                    when_true,
                    when_false,
                    ..
                } => {
                    frames.push(Frame {
                        signal: cond,
                    });
                    frames.push(Frame {
                        signal: when_true,
                    });
                    frames.push(Frame {
                        signal: when_false,
                    });
                }

                graph::SignalData::MemReadPortOutput { mem, .. } => {
                    let key = mem;
                    let mem_name = format!("{}_{}", mem.name, self.mems.len());
                    // TODO: It might actually be too conservative to trace all read ports,
                    //  as we only know that the write port and _this_ read port are reachable
                    //  at this point, but we have to keep some extra state to know whether or
                    //  not we've hit each read port otherwise.
                    let mut read_signal_names = HashMap::new();
                    for (index, (address, enable)) in mem.read_ports.borrow().iter().enumerate() {
                        let name_prefix = format!("{}_read_port_{}_", mem_name, index);
                        read_signal_names.insert(
                            (*address, *enable),
                            ReadSignalNames {
                                address_name: format!("{}address", name_prefix),
                                enable_name: format!("{}enable", name_prefix),
                                value_name: format!("{}value", name_prefix),
                            },
                        );
                    }
                    let name_prefix = format!("{}_write_port_", mem_name);
                    let write_address_name = format!("{}address", name_prefix);
                    let write_value_name = format!("{}value", name_prefix);
                    let write_enable_name = format!("{}enable", name_prefix);
                    self.mems.insert(
                        key,
                        Mem {
                            mem,
                            mem_name,
                            write_address_name,
                            write_value_name,
                            write_enable_name,
                            read_signal_names,
                        },
                    );
                    for (address, enable) in mem.read_ports.borrow().iter() {
                        frames.push(Frame {
                            signal: address,
                        });
                        frames.push(Frame {
                            signal: enable,
                        });
                    }
                    if let Some((address, value, enable)) = *mem.write_port.borrow() {
                        frames.push(Frame {
                            signal: address,
                        });
                        frames.push(Frame {
                            signal: value,
                        });
                        frames.push(Frame {
                            signal: enable,
                        });
                    }
                }
            }
        }
    }
}
