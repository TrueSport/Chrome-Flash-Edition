use syntect::highlighting::{Highlighter, HighlightState};
use syntect::parsing::{ParseState, ScopeStack, SyntaxDefinition};

#[derive(Clone, Debug, PartialEq)]
pub struct RenderState {
    pub highlight: HighlightState,
    pub parse: ParseState
}

impl RenderSta