use youth_sdk::prelude::*;
use youth_sdk::{StateReader, StateWriter};

use crate::model::{Command, Decimal, Entry, Mode, Model, Operator};

pub(crate) struct Calculator;

impl Application for Calculator {
    fn view(context: &ViewContext) -> Result<Tree> {
        let model = load(context.state())?;
        Ok(Tree::root(BoxNode::column([
            Text::new(node!("display"), model.display()),
            Button::new(node!("clear"), "C"),
            Button::new(node!("backspace"), "Backspace"),
            Button::new(node!("toggle-sign"), "+/-"),
            Button::new(node!("divide"), "/"),
            Button::new(node!("digit-7"), "7"),
            Button::new(node!("digit-8"), "8"),
            Button::new(node!("digit-9"), "9"),
            Button::new(node!("multiply"), "*"),
            Button::new(node!("digit-4"), "4"),
            Button::new(node!("digit-5"), "5"),
            Button::new(node!("digit-6"), "6"),
            Button::new(node!("subtract"), "-"),
            Button::new(node!("digit-1"), "1"),
            Button::new(node!("digit-2"), "2"),
            Button::new(node!("digit-3"), "3"),
            Button::new(node!("add"), "+"),
            Button::new(node!("digit-0"), "0"),
            Button::new(node!("decimal"), "."),
            Button::new(node!("equals"), "="),
        ])))
    }

    fn handle(context: &mut EventContext, events: &Events) -> Result<Update> {
        let Some(command) = command(events) else {
            return Ok(Update::unchanged());
        };
        let mut model = load(context.state())?;
        let before = model.clone();
        model.apply(command);
        if model == before {
            return Ok(Update::unchanged());
        }
        save(context.state(), &model)?;
        Ok(Update::new().set_text(node!("display"), model.display()))
    }
}

fn command(events: &Events) -> Option<Command> {
    let commands = [
        (node!("clear"), Command::Clear),
        (node!("backspace"), Command::Backspace),
        (node!("toggle-sign"), Command::ToggleSign),
        (node!("divide"), Command::Operator(Operator::Divide)),
        (node!("digit-7"), Command::Digit(7)),
        (node!("digit-8"), Command::Digit(8)),
        (node!("digit-9"), Command::Digit(9)),
        (node!("multiply"), Command::Operator(Operator::Multiply)),
        (node!("digit-4"), Command::Digit(4)),
        (node!("digit-5"), Command::Digit(5)),
        (node!("digit-6"), Command::Digit(6)),
        (node!("subtract"), Command::Operator(Operator::Subtract)),
        (node!("digit-1"), Command::Digit(1)),
        (node!("digit-2"), Command::Digit(2)),
        (node!("digit-3"), Command::Digit(3)),
        (node!("add"), Command::Operator(Operator::Add)),
        (node!("digit-0"), Command::Digit(0)),
        (node!("decimal"), Command::DecimalPoint),
        (node!("equals"), Command::Equals),
    ];
    commands
        .into_iter()
        .find_map(|(node, command)| events.activated(node).then_some(command))
}

trait ReadState: Copy {
    fn read_text(self, key: &str) -> Result<Option<String>>;
    fn read_integer(self, key: &str) -> Result<Option<i64>>;
    fn read_boolean(self, key: &str) -> Result<Option<bool>>;
}

impl ReadState for StateReader {
    fn read_text(self, key: &str) -> Result<Option<String>> {
        self.text(key)
    }

    fn read_integer(self, key: &str) -> Result<Option<i64>> {
        self.integer(key)
    }

    fn read_boolean(self, key: &str) -> Result<Option<bool>> {
        self.boolean(key)
    }
}

impl ReadState for StateWriter {
    fn read_text(self, key: &str) -> Result<Option<String>> {
        self.text(key)
    }

    fn read_integer(self, key: &str) -> Result<Option<i64>> {
        self.integer(key)
    }

    fn read_boolean(self, key: &str) -> Result<Option<bool>> {
        self.boolean(key)
    }
}

fn load(state: impl ReadState) -> Result<Model> {
    let Some(mode) = state.read_text("mode")? else {
        return Ok(Model::default());
    };
    let model = Model {
        mode: parse_mode(&mode)?,
        entry: Entry {
            coefficient: to_u64(required_integer(state, "entry-coefficient")?)?,
            scale: to_u8(required_integer(state, "entry-scale")?)?,
            negative: required_boolean(state, "entry-negative")?,
            decimal_entered: required_boolean(state, "entry-decimal")?,
        },
        accumulator: load_decimal(state, "accumulator")?,
        pending_operator: load_operator(state, "pending-operator")?,
        last_operator: load_operator(state, "last-operator")?,
        last_operand: load_decimal(state, "last-operand")?,
    };
    if !model.is_valid() {
        return Err(Error::invalid_state().with_message("calculator state is invalid"));
    }
    Ok(model)
}

fn save(state: StateWriter, model: &Model) -> Result<()> {
    state.set_text("mode", mode_name(model.mode))?;
    state.set_integer(
        "entry-coefficient",
        i64::try_from(model.entry.coefficient).map_err(|_| Error::invalid_state())?,
    )?;
    state.set_integer("entry-scale", i64::from(model.entry.scale))?;
    state.set_boolean("entry-negative", model.entry.negative)?;
    state.set_boolean("entry-decimal", model.entry.decimal_entered)?;
    save_decimal(state, "accumulator", model.accumulator)?;
    save_operator(state, "pending-operator", model.pending_operator)?;
    save_operator(state, "last-operator", model.last_operator)?;
    save_decimal(state, "last-operand", model.last_operand)?;
    Ok(())
}

fn required_integer(state: impl ReadState, key: &str) -> Result<i64> {
    state
        .read_integer(key)?
        .ok_or_else(|| Error::invalid_state().with_message(format!("missing state key {key}")))
}

fn required_boolean(state: impl ReadState, key: &str) -> Result<bool> {
    state
        .read_boolean(key)?
        .ok_or_else(|| Error::invalid_state().with_message(format!("missing state key {key}")))
}

fn load_decimal(state: impl ReadState, prefix: &str) -> Result<Option<Decimal>> {
    let coefficient = state.read_integer(&format!("{prefix}-coefficient"))?;
    let scale = state.read_integer(&format!("{prefix}-scale"))?;
    match (coefficient, scale) {
        (None, None) => Ok(None),
        (Some(coefficient), Some(scale)) => Ok(Some(Decimal {
            coefficient,
            scale: to_u8(scale)?,
        })),
        _ => Err(Error::invalid_state().with_message(format!("partial decimal state {prefix}"))),
    }
}

fn save_decimal(state: StateWriter, prefix: &str, value: Option<Decimal>) -> Result<()> {
    let coefficient = format!("{prefix}-coefficient");
    let scale = format!("{prefix}-scale");
    if let Some(value) = value {
        state.set_integer(&coefficient, value.coefficient)?;
        state.set_integer(&scale, i64::from(value.scale))?;
    } else {
        state.delete(&coefficient)?;
        state.delete(&scale)?;
    }
    Ok(())
}

fn load_operator(state: impl ReadState, key: &str) -> Result<Option<Operator>> {
    state
        .read_text(key)?
        .map(|value| parse_operator(&value))
        .transpose()
}

fn save_operator(state: StateWriter, key: &str, value: Option<Operator>) -> Result<()> {
    if let Some(value) = value {
        state.set_text(key, operator_name(value))?;
    } else {
        state.delete(key)?;
    }
    Ok(())
}

fn parse_mode(value: &str) -> Result<Mode> {
    match value {
        "entering" => Ok(Mode::Entering),
        "awaiting-operand" => Ok(Mode::AwaitingOperand),
        "result" => Ok(Mode::Result),
        "error" => Ok(Mode::Error),
        _ => Err(Error::invalid_state().with_message("unknown calculator mode")),
    }
}

fn mode_name(value: Mode) -> &'static str {
    match value {
        Mode::Entering => "entering",
        Mode::AwaitingOperand => "awaiting-operand",
        Mode::Result => "result",
        Mode::Error => "error",
    }
}

fn parse_operator(value: &str) -> Result<Operator> {
    match value {
        "add" => Ok(Operator::Add),
        "subtract" => Ok(Operator::Subtract),
        "multiply" => Ok(Operator::Multiply),
        "divide" => Ok(Operator::Divide),
        _ => Err(Error::invalid_state().with_message("unknown calculator operator")),
    }
}

fn operator_name(value: Operator) -> &'static str {
    match value {
        Operator::Add => "add",
        Operator::Subtract => "subtract",
        Operator::Multiply => "multiply",
        Operator::Divide => "divide",
    }
}

fn to_u8(value: i64) -> Result<u8> {
    u8::try_from(value).map_err(|_| Error::invalid_state())
}

fn to_u64(value: i64) -> Result<u64> {
    u64::try_from(value).map_err(|_| Error::invalid_state())
}
