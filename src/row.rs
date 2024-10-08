use std::{collections::HashSet, num::ParseFloatError};

use crate::cell::Cell;
use crate::traits;
use traits::XMLString;
use xmlwriter::*;

static ALP: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub struct Row {
    cells: Vec<Cell>,
    row_number: usize,
    column_number: usize,
    cell_reference_set: HashSet<String>,
}

fn split_cell_ref(cell_ref: &str) -> Result<(String, String), &'static str> {
    let mut col_ref = String::new();
    let mut row_ref = String::new();
    for c in cell_ref.chars() {
        if c.is_alphabetic() {
            col_ref.push(c);
        } else {
            row_ref.push(c);
        }
    }
    if col_ref.is_empty() || row_ref.is_empty() {
        return Err("Invalid cell reference");
    }
    Ok((col_ref, row_ref))
}

impl Row {
    pub fn new(row_number: usize) -> Row {
        Row {
            row_number,

            cells: Vec::new(),
            column_number: 1,
            cell_reference_set: HashSet::new(),
        }
    }

    pub fn add_string(&mut self, value: String) -> &mut Cell {
        let cell_reference = self.get_next_cell_ref();
        let cell = Cell::from_string(value, cell_reference, false);
        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn add_inline_string(&mut self, value: String) -> &mut Cell {
        let cell_reference = self.get_next_cell_ref();
        let cell = Cell::from_string(value, cell_reference, true);
        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn add_number(&mut self, value: String) -> Result<&mut Cell, ParseFloatError> {
        let cell_reference = self.get_next_cell_ref();
        let cell = Cell::from_number(value, cell_reference)?;
        self.cells.push(cell);
        Ok(self.cells.last_mut().unwrap())
    }

    /// add a cell to an existing row
    /// can fail if the given reference is already present or not valid.
    pub fn add_cell(&mut self, cell: Cell) -> Result<&mut Cell, &'static str> {
        let cell_reference = cell.get_attributes().reference.as_ref().unwrap();
        if self.cell_reference_set.contains(cell_reference) {
            return Err("Cell reference already exists");
        }
        let (column_ref, row_number) = split_cell_ref(cell_reference).unwrap();
        if row_number != self.row_number.to_string() {
            return Err("Invalid row reference");
        }

        // update the set
        self.cell_reference_set.insert(cell_reference.clone());
        self.cells.push(cell);
        // todo: update the coulumn_number;
        self.column_number = self.column_ref_to_number(column_ref.as_str());
        Ok(self.cells.last_mut().unwrap())
    }

    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    fn get_next_cell_ref(&mut self) -> String {
        println!("<{}>", self.column_number);

        if self.column_number == 0 {
            self.column_number += 1;
            return format!("A{}", self.row_number);
        } else {
            let mut result = Vec::with_capacity(3);
            let mut idx = self.column_number; // Make a mutable copy of the index
                                              // Process the index until it is fully converted
            while idx > 0 {
                let remainder = ((idx - 1) % 26) as usize; // Find the remainder when divided by 26
                let ch = ALP.get(remainder).unwrap();
                result.push(ch); // Append character to result
                idx = idx / 26; // Update index for next iteration
            }
            self.column_number += 1;
            return format!(
                "{}{}",
                result.into_iter().rev().collect::<String>(),
                self.row_number
            );
        }
    }

    fn column_ref_to_number(&self, s: &str) -> usize {
        let mut index = 0;

        for ch in s.chars() {
            // Convert character to its 1-based value
            let value = (ch as u8 - b'A' + 1) as usize;
            // Calculate the index by considering base-26 positional values
            index = index * 26 + value;
        }

        index
    }
}

impl XMLString for Row {
    fn to_xml(self, writer: &mut XmlWriter) {
        writer.start_element("row");
        writer.write_attribute("r", &self.row_number.to_string());
        for cell in self.cells {
            cell.to_xml(writer);
        }
        writer.end_element();
    }
}

pub struct ColMovement<'a> {
    row: &'a mut Row,
}

impl<'a> ColMovement<'a> {
    pub fn new(row: &'a mut Row) -> ColMovement<'a> {
        ColMovement { row }
    }

    /// move to next column
    pub fn next(&mut self) {
        self.row.get_next_cell_ref();
    }

    /// skip n columns
    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.row.get_next_cell_ref();
        }
        println!(" > {} ", self.row.column_number);
    }
}
