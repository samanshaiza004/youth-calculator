const PRECISION: u32 = 12;
const MAX_COEFFICIENT: i64 = 999_999_999_999;
const DIVISION_SCALE: u8 = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Entering,
    AwaitingOperand,
    Result,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    Digit(u8),
    DecimalPoint,
    ToggleSign,
    Backspace,
    Clear,
    Operator(Operator),
    Equals,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Decimal {
    pub coefficient: i64,
    pub scale: u8,
}

impl Decimal {
    fn from_parts(coefficient: i128, scale: u8) -> Option<Self> {
        let mut coefficient = coefficient;
        let mut scale = scale;
        if coefficient == 0 {
            return Some(Self {
                coefficient: 0,
                scale: 0,
            });
        }

        while decimal_digits(coefficient) > PRECISION {
            if scale == 0 {
                return None;
            }
            coefficient = round_divide_half_even(coefficient, 10)?;
            scale -= 1;
        }
        while scale > 0 && coefficient % 10 == 0 {
            coefficient /= 10;
            scale -= 1;
        }
        let coefficient = i64::try_from(coefficient).ok()?;
        if coefficient.unsigned_abs() > MAX_COEFFICIENT as u64 {
            return None;
        }
        Some(Self { coefficient, scale })
    }

    fn calculate(self, operator: Operator, right: Self) -> Option<Self> {
        match operator {
            Operator::Add | Operator::Subtract => {
                let scale = self.scale.max(right.scale);
                let left =
                    i128::from(self.coefficient).checked_mul(power_of_ten(scale - self.scale)?)?;
                let right = i128::from(right.coefficient)
                    .checked_mul(power_of_ten(scale - right.scale)?)?;
                let coefficient = match operator {
                    Operator::Add => left.checked_add(right)?,
                    Operator::Subtract => left.checked_sub(right)?,
                    Operator::Multiply | Operator::Divide => unreachable!(),
                };
                Self::from_parts(coefficient, scale)
            }
            Operator::Multiply => Self::from_parts(
                i128::from(self.coefficient).checked_mul(i128::from(right.coefficient))?,
                self.scale.checked_add(right.scale)?,
            ),
            Operator::Divide => {
                if right.coefficient == 0 {
                    return None;
                }
                let numerator = i128::from(self.coefficient)
                    .checked_mul(power_of_ten(right.scale.checked_add(DIVISION_SCALE)?)?)?;
                let denominator =
                    i128::from(right.coefficient).checked_mul(power_of_ten(self.scale)?)?;
                let coefficient = round_divide_half_even(numerator, denominator)?;
                Self::from_parts(coefficient, DIVISION_SCALE)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Entry {
    pub coefficient: u64,
    pub scale: u8,
    pub negative: bool,
    pub decimal_entered: bool,
}

impl Entry {
    fn from_decimal(value: Decimal) -> Self {
        Self {
            coefficient: value.coefficient.unsigned_abs(),
            scale: value.scale,
            negative: value.coefficient < 0,
            decimal_entered: value.scale > 0,
        }
    }

    fn decimal(self) -> Option<Decimal> {
        let coefficient = i64::try_from(self.coefficient).ok()?;
        let coefficient = if self.negative {
            coefficient.checked_neg()?
        } else {
            coefficient
        };
        Decimal::from_parts(i128::from(coefficient), self.scale)
    }

    fn push_digit(&mut self, digit: u8) {
        if digit > 9 {
            return;
        }
        if self.decimal_entered && self.scale >= DIVISION_SCALE {
            return;
        }
        let next = match self
            .coefficient
            .checked_mul(10)
            .and_then(|value| value.checked_add(u64::from(digit)))
        {
            Some(value) => value,
            None => return,
        };
        if significant_digits(next) > PRECISION {
            return;
        }
        self.coefficient = next;
        if self.decimal_entered {
            self.scale += 1;
        }
    }

    fn backspace(&mut self) {
        if self.decimal_entered && self.scale == 0 {
            self.decimal_entered = false;
            return;
        }
        if self.coefficient == 0 {
            self.negative = false;
            return;
        }
        self.coefficient /= 10;
        if self.scale > 0 {
            self.scale -= 1;
        }
        if self.coefficient == 0 {
            self.negative = false;
        }
    }

    fn formatted(self) -> String {
        let mut digits = self.coefficient.to_string();
        if self.scale > 0 {
            let scale = usize::from(self.scale);
            if digits.len() <= scale {
                digits.insert_str(0, &"0".repeat(scale + 1 - digits.len()));
            }
            digits.insert(digits.len() - scale, '.');
        } else if self.decimal_entered {
            digits.push('.');
        }
        if self.negative && self.coefficient != 0 {
            digits.insert(0, '-');
        }
        digits
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    pub mode: Mode,
    pub entry: Entry,
    pub accumulator: Option<Decimal>,
    pub pending_operator: Option<Operator>,
    pub last_operator: Option<Operator>,
    pub last_operand: Option<Decimal>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            mode: Mode::Entering,
            entry: Entry::default(),
            accumulator: None,
            pending_operator: None,
            last_operator: None,
            last_operand: None,
        }
    }
}

impl Model {
    pub fn display(&self) -> String {
        if self.mode == Mode::Error {
            "Error".to_owned()
        } else {
            self.entry.formatted()
        }
    }

    pub fn apply(&mut self, command: Command) {
        match command {
            Command::Clear => *self = Self::default(),
            Command::Digit(digit) => self.digit(digit),
            Command::DecimalPoint => self.decimal_point(),
            Command::ToggleSign => self.toggle_sign(),
            Command::Backspace => self.backspace(),
            Command::Operator(operator) => self.operator(operator),
            Command::Equals => self.equals(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.entry.scale <= DIVISION_SCALE
            && significant_digits(self.entry.coefficient) <= PRECISION
            && self.accumulator.is_none_or(valid_decimal)
            && self.last_operand.is_none_or(valid_decimal)
            && !(self.mode == Mode::Error
                && (self.pending_operator.is_some() || self.accumulator.is_some()))
    }

    fn prepare_entry(&mut self) {
        if matches!(
            self.mode,
            Mode::AwaitingOperand | Mode::Result | Mode::Error
        ) {
            if matches!(self.mode, Mode::Result | Mode::Error) {
                self.accumulator = None;
                self.pending_operator = None;
                self.last_operator = None;
                self.last_operand = None;
            }
            self.entry = Entry::default();
            self.mode = Mode::Entering;
        }
    }

    fn digit(&mut self, digit: u8) {
        self.prepare_entry();
        self.entry.push_digit(digit);
    }

    fn decimal_point(&mut self) {
        self.prepare_entry();
        self.entry.decimal_entered = true;
    }

    fn toggle_sign(&mut self) {
        if self.mode == Mode::Error {
            return;
        }
        if self.mode == Mode::AwaitingOperand {
            self.entry = Entry::default();
            self.mode = Mode::Entering;
        }
        if self.entry.coefficient != 0 {
            self.entry.negative = !self.entry.negative;
        }
    }

    fn backspace(&mut self) {
        if self.mode == Mode::Entering {
            self.entry.backspace();
        }
    }

    fn operator(&mut self, operator: Operator) {
        if self.mode == Mode::Error {
            return;
        }
        if self.mode == Mode::AwaitingOperand {
            self.pending_operator = Some(operator);
            return;
        }
        let Some(entry) = self.entry.decimal() else {
            self.fail();
            return;
        };
        let next = match (self.accumulator, self.pending_operator) {
            (Some(left), Some(pending)) => left.calculate(pending, entry),
            _ => Some(entry),
        };
        let Some(next) = next else {
            self.fail();
            return;
        };
        self.entry = Entry::from_decimal(next);
        self.accumulator = Some(next);
        self.pending_operator = Some(operator);
        self.mode = Mode::AwaitingOperand;
    }

    fn equals(&mut self) {
        if self.mode == Mode::Error {
            return;
        }

        let (left, operator, right) =
            if let (Some(left), Some(operator)) = (self.accumulator, self.pending_operator) {
                let right = if self.mode == Mode::AwaitingOperand {
                    self.last_operand.unwrap_or(left)
                } else {
                    match self.entry.decimal() {
                        Some(value) => value,
                        None => {
                            self.fail();
                            return;
                        }
                    }
                };
                (left, operator, right)
            } else if self.mode == Mode::Result {
                let (Some(operator), Some(right), Some(left)) =
                    (self.last_operator, self.last_operand, self.entry.decimal())
                else {
                    return;
                };
                (left, operator, right)
            } else {
                self.mode = Mode::Result;
                return;
            };

        let Some(result) = left.calculate(operator, right) else {
            self.fail();
            return;
        };
        self.entry = Entry::from_decimal(result);
        self.accumulator = None;
        self.pending_operator = None;
        self.last_operator = Some(operator);
        self.last_operand = Some(right);
        self.mode = Mode::Result;
    }

    fn fail(&mut self) {
        self.mode = Mode::Error;
        self.entry = Entry::default();
        self.accumulator = None;
        self.pending_operator = None;
        self.last_operator = None;
        self.last_operand = None;
    }
}

fn valid_decimal(value: Decimal) -> bool {
    value.scale <= DIVISION_SCALE
        && value.coefficient.unsigned_abs() <= MAX_COEFFICIENT as u64
        && (value.coefficient == 0 || value.coefficient % 10 != 0 || value.scale == 0)
}

fn significant_digits(value: u64) -> u32 {
    if value == 0 { 1 } else { value.ilog10() + 1 }
}

fn decimal_digits(value: i128) -> u32 {
    let magnitude = value.unsigned_abs();
    if magnitude == 0 {
        1
    } else {
        magnitude.ilog10() + 1
    }
}

fn power_of_ten(exponent: u8) -> Option<i128> {
    10_i128.checked_pow(u32::from(exponent))
}

fn round_divide_half_even(numerator: i128, denominator: i128) -> Option<i128> {
    if denominator == 0 {
        return None;
    }
    let quotient = numerator.checked_div(denominator)?;
    let remainder = numerator.checked_rem(denominator)?;
    let doubled = remainder.unsigned_abs().checked_mul(2)?;
    let divisor = denominator.unsigned_abs();
    let round = doubled > divisor || (doubled == divisor && quotient % 2 != 0);
    if !round {
        return Some(quotient);
    }
    let direction = if numerator.signum() == denominator.signum() {
        1
    } else {
        -1
    };
    quotient.checked_add(direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(commands: impl IntoIterator<Item = Command>) -> Model {
        let mut model = Model::default();
        for command in commands {
            model.apply(command);
            assert!(model.is_valid());
        }
        model
    }

    #[test]
    fn entry_formatting_preserves_decimal_intent_and_normalizes_negative_zero() {
        let model = run([
            Command::DecimalPoint,
            Command::Digit(0),
            Command::Digit(5),
            Command::ToggleSign,
        ]);
        assert_eq!(model.display(), "-0.05");

        let zero = run([Command::ToggleSign, Command::DecimalPoint]);
        assert_eq!(zero.display(), "0.");
    }

    #[test]
    fn backspace_edits_only_an_active_entry() {
        let model = run([
            Command::Digit(1),
            Command::Digit(2),
            Command::DecimalPoint,
            Command::Digit(3),
            Command::Backspace,
            Command::Backspace,
        ]);
        assert_eq!(model.display(), "12");

        let result = run([Command::Digit(9), Command::Equals, Command::Backspace]);
        assert_eq!(result.display(), "9");
    }

    #[test]
    fn operators_chain_left_to_right_and_can_be_replaced() {
        let model = run([
            Command::Digit(2),
            Command::Operator(Operator::Add),
            Command::Operator(Operator::Multiply),
            Command::Digit(3),
            Command::Operator(Operator::Add),
            Command::Digit(4),
            Command::Equals,
        ]);
        assert_eq!(model.display(), "10");
    }

    #[test]
    fn subtraction_and_clear_are_explicit_commands() {
        let model = run([
            Command::Digit(9),
            Command::Operator(Operator::Subtract),
            Command::Digit(4),
            Command::Equals,
            Command::Clear,
        ]);
        assert_eq!(model, Model::default());
    }

    #[test]
    fn equals_repeats_the_last_operation() {
        let model = run([
            Command::Digit(5),
            Command::Operator(Operator::Add),
            Command::Digit(2),
            Command::Equals,
            Command::Equals,
            Command::Equals,
        ]);
        assert_eq!(model.display(), "11");
    }

    #[test]
    fn equals_without_a_right_operand_reuses_the_left_operand() {
        let model = run([
            Command::Digit(5),
            Command::Operator(Operator::Add),
            Command::Equals,
        ]);
        assert_eq!(model.display(), "10");
    }

    #[test]
    fn digit_and_decimal_after_equals_begin_a_new_calculation() {
        let digit = run([
            Command::Digit(2),
            Command::Operator(Operator::Add),
            Command::Digit(3),
            Command::Equals,
            Command::Digit(7),
        ]);
        assert_eq!(digit.display(), "7");

        let decimal = run([
            Command::Digit(2),
            Command::Equals,
            Command::DecimalPoint,
            Command::Digit(5),
        ]);
        assert_eq!(decimal.display(), "0.5");
    }

    #[test]
    fn division_rounds_half_even_to_twelve_significant_digits() {
        let third = run([
            Command::Digit(1),
            Command::Operator(Operator::Divide),
            Command::Digit(3),
            Command::Equals,
        ]);
        assert_eq!(third.display(), "0.333333333333");

        assert_eq!(round_divide_half_even(25, 10), Some(2));
        assert_eq!(round_divide_half_even(35, 10), Some(4));
        assert_eq!(round_divide_half_even(-25, 10), Some(-2));
    }

    #[test]
    fn division_by_zero_and_overflow_enter_error_until_clear_or_digit() {
        let mut model = run([
            Command::Digit(1),
            Command::Operator(Operator::Divide),
            Command::Digit(0),
            Command::Equals,
        ]);
        assert_eq!(model.display(), "Error");
        model.apply(Command::ToggleSign);
        assert_eq!(model.display(), "Error");
        model.apply(Command::Digit(7));
        assert_eq!(model.display(), "7");

        let overflow = run(std::iter::repeat_n(Command::Digit(9), 12).chain([
            Command::Operator(Operator::Add),
            Command::Digit(1),
            Command::Equals,
        ]));
        assert_eq!(overflow.display(), "Error");
    }

    #[test]
    fn entry_is_bounded_to_twelve_significant_digits_and_fractional_places() {
        let model = run(std::iter::repeat_n(Command::Digit(9), 20));
        assert_eq!(model.display(), "999999999999");

        let fraction = run(std::iter::once(Command::DecimalPoint)
            .chain(std::iter::repeat_n(Command::Digit(0), 12).chain([Command::Digit(1)])));
        assert_eq!(fraction.display(), "0.000000000000");
    }
}
