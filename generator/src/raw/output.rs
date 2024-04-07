use crate::generator::TypeDef;

#[derive(Debug)]
pub struct Output {
    pub def: TypeDef,
    pub module_defs: Vec<TypeDef>,
    pub global_defs: Vec<TypeDef>,
}

impl Output {
    pub fn new() -> Self {
        Self {
            def: TypeDef::Unit,
            module_defs: Vec::new(),
            global_defs: Vec::new(),
        }
    }

    pub fn bare_def(def: TypeDef) -> Self {
        Self {
            def,
            module_defs: Vec::new(),
            global_defs: Vec::new(),
        }
    }

    pub fn absorb(&mut self, inner: Output) {
        self.module_defs.push(inner.def);
        self.module_defs.extend(inner.module_defs);
        self.global_defs.extend(inner.global_defs);
    }
}
