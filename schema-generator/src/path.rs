#[derive(Debug, Clone, PartialEq)]
pub enum PathElement {
    Literal(String),
    Placeholder(String),
}

impl From<&str> for PathElement {
    fn from(value: &str) -> Self {
        if value.starts_with('{') && value.ends_with('}') {
            Self::Placeholder(value[1..value.len() - 1].to_string())
        } else {
            Self::Literal(value.to_string())
        }
    }
}

impl core::fmt::Display for PathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathElement::Literal(l) => write!(f, "{l}"),
            PathElement::Placeholder(p) => write!(f, "{{{p}}}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    elements: Vec<PathElement>,
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(err) = self.elements.iter().find_map(|v| write!(f, "/{v}").err()) {
            return Err(err);
        }

        Ok(())
    }
}

impl TryFrom<&str> for Path {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        if !value.starts_with('/') {
            return Err(());
        }

        let elements = value
            .split('/')
            .skip(1)
            .map(|v| PathElement::from(v))
            .collect();

        Ok(Self { elements })
    }
}

impl Path {
    pub fn matches<'a>(&self, other: impl IntoIterator<Item = &'a str>) -> bool {
        let mut elements = self.elements.iter();
        let mut other = other.into_iter();
        let mut zipped = (&mut elements).zip(&mut other);

        for (l, r) in &mut zipped {
            if let PathElement::Literal(literal) = l {
                if literal != r {
                    return false;
                }
            }
        }

        other.next().is_none() && elements.next().is_none()
    }
}
