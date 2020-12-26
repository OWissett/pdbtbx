#![allow(dead_code)]
use crate::structs::*;

#[derive(Debug)]
pub struct PDB {
    pub remarks: Vec<(usize, String)>,
    pub scale: Option<Scale>,
    pub unit_cell: Option<UnitCell>,
    pub symmetry: Option<Symmetry>,
    models: Vec<Model>,
}

impl PDB {
    pub fn new() -> PDB {
        PDB {
            remarks: Vec::new(),
            scale: None,
            unit_cell: None,
            symmetry: None,
            models: Vec::new(),
        }
    }

    pub fn add_model(&mut self, new_model: Model) {
        self.models.push(new_model);
        self.models.last_mut().unwrap().fix_pointers_of_children();
    }

    pub fn models(&self) -> impl DoubleEndedIterator<Item = &Model> + '_ {
        self.models.iter()
    }

    pub fn models_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Model> + '_ {
        self.models.iter_mut()
    }

    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().map(|a| a.chains()).flatten()
    }

    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models.iter_mut().map(|a| a.chains_mut()).flatten()
    }

    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().map(|a| a.residues()).flatten()
    }

    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models.iter_mut().map(|a| a.residues_mut()).flatten()
    }

    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().map(|a| a.atoms()).flatten()
    }

    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models.iter_mut().map(|a| a.atoms_mut()).flatten()
    }

    pub fn hetero_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().map(|a| a.hetero_chains()).flatten()
    }

    pub fn hetero_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_chains_mut())
            .flatten()
    }

    pub fn hetero_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().map(|a| a.hetero_residues()).flatten()
    }

    pub fn hetero_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_residues_mut())
            .flatten()
    }

    pub fn hetero_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().map(|a| a.hetero_atoms()).flatten()
    }

    pub fn hetero_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_atoms_mut())
            .flatten()
    }

    pub fn all_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().map(|a| a.all_chains()).flatten()
    }

    pub fn all_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models.iter_mut().map(|a| a.all_chains_mut()).flatten()
    }

    pub fn all_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().map(|a| a.all_residues()).flatten()
    }

    pub fn all_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.all_residues_mut())
            .flatten()
    }

    pub fn all_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().map(|a| a.all_atoms()).flatten()
    }

    pub fn all_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models.iter_mut().map(|a| a.all_atoms_mut()).flatten()
    }

    pub fn scale(&mut self) -> &mut Scale {
        match &mut self.scale {
            Some(s) => s,
            None => panic!("Expected a Scale but it was not in place (it was None)."),
        }
    }

    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut PDB = self;
        for model in &mut self.models {
            model.set_pdb_pointer(reference);
            model.fix_pointers_of_children();
        }
    }

    pub fn remove_model(&mut self, index: usize) {
        self.models.remove(index);
    }

    pub fn remove_model_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .models
            .iter()
            .position(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_model(i);
            true
        } else {
            false
        }
    }
}

use std::fmt;
impl fmt::Display for PDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PDB Models: {}", self.models.len())
    }
}
