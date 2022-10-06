use super::*;

pub struct Transition {
    src: Reference<StateId>,
    dst: Reference<StateId>,
}

impl Transition {
    pub fn new(src: Reference<StateId>, dst: Reference<StateId>) -> Self {
        Self { src, dst }
    }

    pub fn src(&self) -> &Reference<StateId> {
        &self.src
    }

    pub fn dst(&self) -> &Reference<StateId> {
        &self.dst
    }
}

impl ToLang for Transition {
    fn to_lang(&self, model: &Model) -> String {
        format!("{} -> {}", self.src.to_lang(model), self.dst.to_lang(model))
    }
}

pub enum Transitions {
    All,
    List(Vec<Transition>),
}

impl ToLang for Transitions {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Transitions::All => format!("\t\t\ttransition all\n"),
            Transitions::List(l) => {
                let mut s = String::from("\t\t\ttransition {\n");
                for x in l {
                    s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(model)));
                }
                s.push_str("\t\t\t}\n");
                s
            }
        }
    }
}
