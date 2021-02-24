#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use std::fmt;

#[derive(Debug)]
/// A Conformer of a Conformer containing multiple atoms, analogous to 'atom_group' in cctbx
pub struct Conformer {
    /// The name of this Conformer
    name: String,
    /// The alternative location of this Conformer, None is blank
    alternative_location: Option<String>,
    /// The list of atoms making up this Conformer
    atoms: Vec<Atom>,
    /// The modification, if present
    modification: Option<(String, String)>,
}

impl Conformer {
    /// Create a new Conformer
    ///
    /// ## Arguments
    /// * `name` - the name
    /// * `alt_loc` - the alternative location identifier, if not blank
    /// * `atom` - if available it can already add an atom
    ///
    /// ## Fails
    /// It fails if any of the characters making up the name are invalid.
    pub fn new(name: &str, alt_loc: Option<&str>, atom: Option<Atom>) -> Option<Conformer> {
        if let Some(n) = prepare_identifier(name) {
            let mut res = Conformer {
                name: n,
                alternative_location: None,
                atoms: Vec::new(),
                modification: None,
            };
            if let Some(al) = alt_loc {
                res.alternative_location = prepare_identifier(al);
            }
            if let Some(a) = atom {
                res.atoms.push(a);
            }
            Some(res)
        } else {
            None
        }
    }

    /// The name of the Conformer
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name of the Conformer
    ///
    /// ## Fails
    /// It fails if any of the characters of the new name are invalid.
    pub fn set_name(&mut self, new_name: &str) -> bool {
        if let Some(n) = prepare_identifier(new_name) {
            self.name = n;
            true
        } else {
            false
        }
    }

    /// The alternative location of the Conformer
    pub fn alternative_location(&self) -> Option<&str> {
        self.alternative_location.as_deref()
    }

    /// Set the alternative location of the Conformer
    ///
    /// ## Fails
    /// It fails if any of the characters of the new alternative location are invalid.
    pub fn set_alternative_location(&mut self, new_loc: &str) -> bool {
        if let Some(l) = prepare_identifier(new_loc) {
            self.alternative_location = Some(l);
            true
        } else {
            false
        }
    }

    /// Returns the uniquely identifying construct for this Conformer.
    /// It consists of the name and the alternative location.
    pub fn id(&self) -> (&str, Option<&str>) {
        (&self.name, self.alternative_location())
    }

    /// Get the modification of this Conformer e.g., chemical or post-translational. These will be saved in the MODRES records in the PDB file
    pub fn modification(&self) -> Option<&(String, String)> {
        self.modification.as_ref()
    }

    /// Set the modification of this Conformer e.g., chemical or post-translational. These will be saved in the MODRES records in the PDB file
    pub fn set_modification(&mut self, new_modification: (String, String)) -> Result<(), String> {
        if !valid_identifier(&new_modification.0) {
            Err(format!(
                "New modification has invalid characters for standard conformer name, conformer: {:?}, standard name \"{}\"",
                self.id(), new_modification.0
            ))
        } else if !valid_text(&new_modification.1) {
            Err(format!(
                "New modification has invalid characters the comment, conformer: {:?}, comment \"{}\"",
                self.id(), new_modification.1
            ))
        } else {
            self.modification = Some(new_modification);
            Ok(())
        }
    }

    /// The amount of atoms making up this Conformer
    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// Get a specific atom from list of atoms making up this Conformer.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms.get(index)
    }

    /// Get a specific atom as a mutable reference from list of atoms making up this Conformer.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms.get_mut(index)
    }

    /// Get the list of atoms making up this Conformer.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.atoms.iter()
    }

    /// Get the list of atoms as mutable references making up this Conformer.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.atoms.iter_mut()
    }

    /// Add a new atom to the list of atoms making up this Conformer.
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    pub fn add_atom(&mut self, new_atom: Atom) {
        self.atoms.push(new_atom);
    }

    /// Returns if this Conformer is an amino acid
    pub fn amino_acid(&self) -> bool {
        reference_tables::get_amino_acid_number(self.name()).is_some()
    }

    /// Remove all Atoms matching the given predicate. As this is done in place this is the fastest way to remove Atoms from this Conformer.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        self.atoms.retain(|atom| !predicate(atom));
    }

    /// Remove the Atom specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_atom(&mut self, index: usize) {
        self.atoms.remove(index);
    }

    /// Remove the Atom specified. It returns `true` if it found a matching Atom and removed it.
    /// It removes the first matching Atom from the list.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_atom_by_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .atoms
            .iter()
            .position(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }

    /// Remove the Atom specified. It returns `true` if it found a matching Atom and removed it.
    /// It removes the first matching Atom from the list.
    ///
    /// ## Arguments
    /// * `name` - the name of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_atom_by_name(&mut self, name: String) -> bool {
        let index = self.atoms.iter().position(|a| a.name() == name);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }

    /// Apply a transformation to the position of all atoms making up this Conformer, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Conformer with another Conformer, this moves all atoms from the other Conformer
    /// to this Conformer. All other (meta) data of this Conformer will stay the same.
    pub fn join(&mut self, other: Conformer) {
        self.atoms.extend(other.atoms);
    }

    /// Extend the Atoms on this Conformer by the given iterator.
    pub fn extend<T: IntoIterator<Item = Atom>>(&mut self, iter: T) {
        self.atoms.extend(iter);
    }
}

impl fmt::Display for Conformer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CONFORMER ID:{:?}, Atoms:{}",
            self.id(),
            self.atoms.len(),
        )
    }
}

impl Clone for Conformer {
    fn clone(&self) -> Self {
        let mut res = Conformer::new(&self.name, self.alternative_location(), None)
            .expect("Invalid properties while cloning a Conformer");
        res.atoms = self.atoms.clone();
        res
    }
}

impl PartialEq for Conformer {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.atoms == other.atoms
    }
}