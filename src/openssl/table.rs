pub enum TableRow {
    Header(String,String,String),
    Data(String,String,i64),
}

impl TableRow {
    pub fn new(host: String, organisation: String, remaining: i64) -> TableRow {
        //wrap in ""
        let organisation = format!("\"{}\"", organisation);
        TableRow::Data(host, organisation, remaining)
    }

    pub fn header() -> TableRow {
        TableRow::Header("HOST".to_string(), "ORGANISATION".to_string(), "DAYS".to_string())
    }

    pub fn print(&self, len_url: usize, len_org: usize) {
        match self {
            TableRow::Header(host, organisation, remaining) => {
                println!("{0:<ul$} {1:<ol$} {2:<5}", host, organisation, remaining, ul = len_url, ol = len_org);
            },
            TableRow::Data(host, organisation, remaining) => {
                println!("{0:<ul$} {1:<ol$} {2:<5}", host, organisation, remaining, ul = len_url, ol = len_org);
            }
        }
    }
}

pub struct Table {
    pub rows: Vec<TableRow>,
    pub len_url: usize,
    pub len_org: usize,
    pub len_remaining: usize,
}

impl Table {
    pub fn new() -> Table {
        Table { rows: Vec::new(), len_url: 0, len_org: 0 , len_remaining: 0}
    }

    pub fn add(&mut self, row: TableRow) -> &mut Table {
        match &row {
            TableRow::Header(host, organisation, remaining) => {
                self.len_url = host.len() + 1;
                self.len_org = organisation.len() + 1;
                self.len_remaining = remaining.len() + 1;

                self.rows.push(row);
            },
            TableRow::Data(host, organisation, remaining) => {
                if host.len() + 1 > self.len_url {
                    self.len_url = host.len() + 1;
                }
                if organisation.len() + 1 > self.len_org {
                    self.len_org = organisation.len() + 1;
                }
                if remaining.to_string().len() + 1 > self.len_remaining {
                    self.len_remaining = remaining.to_string().len() + 1;
                }

                self.rows.push(row);
            }
        }
        self
    }

    pub fn order_by_host(&mut self) -> &mut Table {
        self.rows.sort_by(|a, b| {
            match (a, b) {
                (TableRow::Header(_, _, _), TableRow::Header(_, _, _)) => std::cmp::Ordering::Equal,
                (TableRow::Header(_, _, _), TableRow::Data(_, _, _)) => std::cmp::Ordering::Less,
                (TableRow::Data(_, _, _), TableRow::Header(_, _, _)) => std::cmp::Ordering::Greater,
                (TableRow::Data(host_a, _, _), TableRow::Data(host_b, _, _)) => host_a.cmp(host_b),
            }
        });
        self
    }

    pub fn order_by_remaining_days(&mut self) -> &mut Table {
        self.rows.sort_by(|a, b| {
            match (a, b) {
                (TableRow::Header(_, _, _), TableRow::Header(_, _, _)) => std::cmp::Ordering::Equal,
                (TableRow::Header(_, _, _), TableRow::Data(_, _, _)) => std::cmp::Ordering::Less,
                (TableRow::Data(_, _, _), TableRow::Header(_, _, _)) => std::cmp::Ordering::Greater,
                (TableRow::Data(_, _, remaining_a), TableRow::Data(_, _, remaining_b)) => remaining_a.cmp(remaining_b),
            }
        });
        self
    }

    pub fn print(&self) {
        for row in &self.rows {
            row.print(self.len_url, self.len_org);
        }
    }
}