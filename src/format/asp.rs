use std::{collections::HashMap, fmt::Display};

/// Represents an Answer Set Programming (ASP) structure.
pub(crate) struct Asp {
    label: HashMap<String, i64>, // A map of labels to their corresponding values
    rules: Vec<AspRule>, // A vector of ASP rules
}

/// Represents an ASP rule, which can be a regular rule or a disjunction rule.
pub(crate) enum AspRule {
    Regular(RegularRule), // A regular ASP rule
    Disjunction(DisjunctRule), // A disjunction ASP rule
}

/// Represents a regular ASP rule with a head and positive/negative body literals.
pub(crate) struct RegularRule {
    head: i64, // The head literal of the rule
    pos_body: Vec<i64>, // The positive body literals of the rule
    neg_body: Vec<i64>, // The negative body literals of the rule
}

/// Represents a disjunction ASP rule with a head and positive/negative body literals.
pub(crate) struct DisjunctRule {
    head: Vec<i64>, // The head literals of the rule
    pos_body: Vec<i64>, // The positive body literals of the rule
    neg_body: Vec<i64>, // The negative body literals of the rule
}

impl Asp {
    /// Creates a new instance of Asp.
    pub(crate) fn new() -> Self {
        Asp {
            label: HashMap::new(), // Initializes an empty label map
            rules: Vec::new(), // Initializes an empty rules vector
        }
    }

    /// Adds a label with its corresponding value to the ASP structure.
    ///
    /// # Arguments
    ///
    /// * `val` - The value of the label
    /// * `name` - The name of the label
    pub(crate) fn add_label(&mut self, val: i64, name: String) {
        self.label.insert(name, val); // Inserts the label and its value into the label map
    }

    /// Adds a rule to the ASP structure.
    ///
    /// # Arguments
    ///
    /// * `head` - The head literals of the rule
    /// * `pos_body` - The positive body literals of the rule
    /// * `neg_body` - The negative body literals of the rule
    pub(crate) fn add_rule(
        &mut self,
        head: Vec<i64>,
        pos_body: Vec<i64>,
        neg_body: Vec<i64>,
    ) {
        let rule = match head.len() {
            0 => {
                AspRule::Regular(RegularRule {
                    head: 1,
                    pos_body: pos_body,
                    neg_body: neg_body,
                })
            }
            1 => {
                AspRule::Regular(RegularRule {
                    head: head[0],
                    pos_body: pos_body,
                    neg_body: neg_body,
                })
            }
            _ => {
                AspRule::Disjunction(DisjunctRule {
                    head: head,
                    pos_body: pos_body,
                    neg_body: neg_body,
                })
            }
        };
        self.rules.push(rule); // Adds the rule to the rules vector
    }
}

impl Display for Asp {
    /// Formats the ASP structure for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rules.iter().try_for_each(|r| writeln!(f, "{}", r))?; // Writes each rule to the formatter
        writeln!(f, "0")?; // Writes '0' to the formatter
        self.label.iter().try_for_each(|(k, v)| writeln!(f, "{} {}", v, k))?; // Writes each label and its value to the formatter
        write!(f, "0\nB+\n0\nB-\n1\n0\n1\n") // Writes additional formatting to the formatter
    }
}

impl Display for AspRule {
    /// Formats the ASP rule for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AspRule::Regular(r) => {
                write!(
                    f,
                    "1 {} {} {}",
                    r.head,
                    r.pos_body.len(),
                    r.neg_body.len()
                )?; // Writes the regular rule header to the formatter
                r.pos_body.iter().try_for_each(|v| write!(f, " {}", v))?; // Writes the positive body literals to the formatter
                r.neg_body.iter().try_for_each(|v| write!(f, " {}", v))?; // Writes the negative body literals to the formatter
            }
            AspRule::Disjunction(r) => {
                write!(f, "8 {}", r.head.len())?; // Writes the disjunction rule header to the formatter
                r.head.iter().try_for_each(|v| write!(f, " {}", v))?; // Writes the head literals to the formatter
                write!(f, " {}", r.pos_body.len())?; // Writes the number of positive body literals to the formatter
                write!(f, " {}", r.neg_body.len())?; // Writes the number of negative body literals to the formatter
                r.pos_body.iter().try_for_each(|v| write!(f, " {}", v))?; // Writes the positive body literals to the formatter
                r.neg_body.iter().try_for_each(|v| write!(f, " {}", v))?; // Writes the negative body literals to the formatter
            }
        }
        Ok(()) // Returns Ok to indicate successful formatting
    }
}