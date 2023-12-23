#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Pulse {
    Z,
    Low,
    High,
    Initial,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LogicType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Probe,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Logic {
    pub kind: LogicType,
    pub dest: Vec<usize>,
    input: Pulse,
    inputs: Vec<Pulse>,
    state: FlipFlopState,
}

impl Ord for Logic {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Logic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Logic {
    pub fn new(kind: LogicType, dest: Vec<usize>) -> Self {
        let input = if kind == LogicType::FlipFlop {
            Pulse::Z
        } else {
            Pulse::Initial
        };
        Logic {
            input,
            inputs: Vec::new(),
            kind,
            state: FlipFlopState::Off,
            dest: Vec::from_iter(dest),
        }
    }

    pub fn blank() -> Self {
        Logic {
            input: Pulse::Z,
            inputs: Vec::new(),
            kind: LogicType::Probe,
            state: FlipFlopState::Off,
            dest: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input: usize, size: usize) {
        if self.inputs.len() != size {
            self.inputs.resize(size, Pulse::Z);
        }
        self.inputs[input] = Pulse::Low;
    }

    pub fn adj_input(&mut self, input: &[bool]) {
        if !self.inputs.is_empty() {
            input
                .iter()
                .enumerate()
                .filter(|(_, remove)| **remove)
                .for_each(|(index, _)| self.inputs[index] = Pulse::Z);
        }
    }

    fn set_basic(&mut self, input: Pulse) {
        self.input = input;
    }

    fn set_conj(&mut self, input: Pulse, source: usize) {
        self.inputs[source] = input;
        self.input = Pulse::Z;
    }

    pub fn set(&mut self, input: Pulse, source: usize) {
        match self.kind {
            LogicType::FlipFlop | LogicType::Broadcaster | LogicType::Probe => {
                self.set_basic(input)
            }
            LogicType::Conjunction => self.set_conj(input, source),
        }
    }

    pub fn prep(&mut self) {
        if self.kind == LogicType::FlipFlop {
            self.prep_flops();
        }
        self.input = match self.kind {
            LogicType::FlipFlop | LogicType::Probe => Pulse::Z,
            LogicType::Conjunction | LogicType::Broadcaster => Pulse::Initial,
        }
    }

    fn prep_flops(&mut self) {
        if self.input == Pulse::Low {
            if self.state == FlipFlopState::Off {
                self.state = FlipFlopState::On;
            } else {
                self.state = FlipFlopState::Off;
            }
        }
    }

    fn get_output_flipflop(&mut self) -> Option<Pulse> {
        let output = match self.input {
            Pulse::Z | Pulse::High | Pulse::Initial => Pulse::Z,
            Pulse::Low => {
                if self.state == FlipFlopState::Off {
                    Pulse::High
                } else {
                    Pulse::Low
                }
            }
        };
        if output == Pulse::Z {
            return None;
        }
        Some(output)
    }

    fn get_output_conj(&mut self) -> Option<Pulse> {
        let output = self.input;
        if output == Pulse::Initial {
            return None;
        }
        if self
            .inputs
            .iter()
            .filter(|input| **input != Pulse::Z)
            .all(|pulse| *pulse == Pulse::High)
        {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn get_broadcast(&mut self) -> Option<Pulse> {
        let output = self.input;
        if output == Pulse::Initial {
            return None;
        }
        Some(output)
    }

    pub fn get(&mut self) -> Option<Pulse> {
        let result = match self.kind {
            LogicType::FlipFlop => self.get_output_flipflop(),
            LogicType::Conjunction => self.get_output_conj(),
            LogicType::Broadcaster => self.get_broadcast(),
            LogicType::Probe => None,
        };
        self.prep();
        result
    }
}
