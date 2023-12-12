



const SCHEMA_SEPERATOR: &char = ';';


struct Schema{
    raw: String,
    seperator: String,
    columns: Vec<Column>
}

struct Column{
    header: String,
    val_required: bool,
    col_required: bool,
    tipe: Tipe,
    misc: String
}

impl Column {
    fn from_row(row: &str)-> Result<Self, String>{
        let values: Vec<&str> = row.split(*SCHEMA_SEPERATOR).collect();

        let Some(header) = values.get(0);
        let Some(tipe) = values.get(1);
        let Some(colRequired) = values.get(2);
        let Some(valRequired) = values.get(3);
        let Some(misc) = values.get(4);

        Ok(Self{
            header: "",
            val_required: todo!(),
            col_required: todo!(),
            tipe: todo!(),
            misc: todo!(),
        })





    }
}

enum Tipe {
    Integer(f64),
    Float(i64),
    String(String),
    Enum(Vec<String>)
}




impl Schema {
    fn from_file()-> Self{

    }
}