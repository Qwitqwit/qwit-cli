use super::type_::{Tipe, Type};

const SCHEMA_SEPERATOR: &char = &';';

#[derive(Debug)]
pub struct Column {
    pub header: String,
    pub val_required: bool,
    pub col_required: bool,
    pub type_: Type,
}

impl Column {
    pub fn from_row(row: &str, sep: &str) -> Result<Self, String> {
        let values: Vec<String> = row
            .split(*SCHEMA_SEPERATOR)
            .map(std::string::ToString::to_string)
            .collect();

        let header = values
            .first()
            .ok_or("did not find anything at pos 0, header should be here".to_owned())?;
        let tipe: &String = values
            .get(1)
            .ok_or("did not find anything at pos 1, type should be here".to_owned())?;
        let col_required = values
            .get(2)
            .ok_or("did not find anything at pos 2, col_required should be here".to_owned())?;
        let val_required = values
            .get(3)
            .ok_or("did not find anything at pos 3, val_required should be here".to_owned())?;
        let misc = values
            .get(4)
            .ok_or("did not find anything at pos 4, misc should be here".to_owned())?;

        let tipe: Tipe = tipe.as_str().into();

        let header: String = if header.contains(sep) {
            return Err(format!(
                "header contains the seperator ->{sep}<-, this is not allowed ->{header}<-"
            )
            .to_string());
        } else {
            header.to_string()
        };

        let type_ = Type::from_tipe(tipe, misc)?;
        let col_required: bool = matches!(col_required.as_str(), "true");
        let val_required: bool = matches!(val_required.as_str(), "true");

        Ok(Self {
            header,
            val_required,
            col_required,
            type_,
        })
    }
}
