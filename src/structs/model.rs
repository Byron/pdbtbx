#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;

#[derive(Debug)]
/// A Model containing multiple Chains
pub struct Model {
    /// The serial number of this Model
    serial_number: usize,
    /// The Chains making up this model
    chains: Vec<Chain>,
    /// The Chains with Hetero Atoms making up this model
    hetero_chains: Vec<Chain>,
}

impl Model {
    /// Create a new Model
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number
    pub fn new(serial_number: usize) -> Model {
        Model {
            serial_number,
            chains: Vec::new(),
            hetero_chains: Vec::new(),
        }
    }

    /// The serial number of this Model
    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    /// Set the serial number of this Model
    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    /// Get the amount of Chains making up this Model.
    /// This disregards all Hetero Chains.
    pub fn chain_count(&self) -> usize {
        self.chains.len()
    }

    /// Get the amount of Residues making up this Model.
    /// This disregards all Hetero Residues.
    pub fn residue_count(&self) -> usize {
        self.chains()
            .fold(0, |sum, chain| chain.residue_count() + sum)
    }

    /// Get the amount of Atoms making up this Model.
    /// This disregards all Hetero Atoms.
    pub fn atom_count(&self) -> usize {
        self.chains().fold(0, |sum, chain| chain.atom_count() + sum)
    }

    /// Get the amount of Chains making up this Model.
    /// This includes all Hetero Chains.
    pub fn total_chain_count(&self) -> usize {
        self.chains.len() + self.hetero_chains.len()
    }

    /// Get the amount of Residues making up this Model.
    /// This includes all Hetero Residues.
    pub fn total_residue_count(&self) -> usize {
        self.all_chains()
            .fold(0, |sum, chain| chain.residue_count() + sum)
    }

    /// Get the amount of Atoms making up this Model.
    /// This includes all Hetero Atoms.
    pub fn total_atom_count(&self) -> usize {
        self.all_chains()
            .fold(0, |sum, chain| chain.atom_count() + sum)
    }

    /// Get a specific Chain from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.all_chains().nth(index)
    }

    /// Get a specific Chain as a mutable reference from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.all_chains_mut().nth(index)
    }

    /// Get a specific Residue from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.all_residues().nth(index)
    }

    /// Get a specific Residue as a mutable reference from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.all_residues_mut().nth(index)
    }

    /// Get a specific Atom from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.all_atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.all_atoms_mut().nth(index)
    }

    /// Get the list of Chains making up this Model.
    /// This disregards all Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter()
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This disregards all Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut()
    }

    /// Get the list of Residues making up this Model.
    /// This disregards all Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains.iter().flat_map(|a| a.residues())
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This disregards all Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains.iter_mut().flat_map(|a| a.residues_mut())
    }

    /// Get the list of Atoms making up this Model.
    /// This disregards all Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This disregards all Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains.iter_mut().flat_map(|a| a.atoms_mut())
    }

    /// Get the list of Chains making up this Model.
    /// This disregards all Normal Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.hetero_chains.iter()
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This disregards all Normal Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.hetero_chains.iter_mut()
    }

    /// Get the list of Residues making up this Model.
    /// This disregards all Normal Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.hetero_chains.iter().flat_map(|a| a.residues())
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This disregards all Normal Residues
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.hetero_chains
            .iter_mut()
            .map(|a| a.residues_mut())
            .flatten()
    }

    /// Get the list of Atoms making up this Model.
    /// This disregards all Normal Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.hetero_chains.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This disregards all Normal Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.hetero_chains
            .iter_mut()
            .map(|a| a.atoms_mut())
            .flatten()
    }

    /// Get the list of Chains making up this Model.
    /// This includes all Normal and Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter().chain(self.hetero_chains.iter())
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This includes all Normal and Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut().chain(self.hetero_chains.iter_mut())
    }

    /// Get the list of Residues making up this Model.
    /// This includes all Normal and Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains
            .iter()
            .map(|a| a.residues())
            .flatten()
            .chain(self.hetero_chains.iter().flat_map(|a| a.residues()))
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This includes all Normal and Hetero Residues
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains
            .iter_mut()
            .flat_map(|a| a.residues_mut())
            .chain(self.hetero_chains.iter_mut().flat_map(|a| a.residues_mut()))
    }

    /// Get the list of Atoms making up this Model.
    /// This includes all Normal and Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains
            .iter()
            .flat_map(|a| a.atoms())
            .chain(self.hetero_chains.iter().flat_map(|a| a.atoms()))
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This includes all Normal and Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains
            .iter_mut()
            .flat_map(|a| a.atoms_mut())
            .chain(self.hetero_chains.iter_mut().flat_map(|a| a.atoms_mut()))
    }

    /// Add a new Atom to this Model. It finds if there already is a Chain with the given `chain_id` if there is it will add this atom to that Chain, otherwise it will create a new Chain and add that to the list of Chains making up this Model. It does the same for the Residue, so it will create a new one if there does not yet exist a Residue with the given serial number.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `chain_id` - the id of the Chain to add the Atom to
    /// * `residue_serial_number` - the serial number of the Residue to add the Atom to
    /// * `residue_name` - the name of the Residue to add the Atom to, only used to create a new Residue if needed
    ///
    /// ## Panics
    /// It panics if the Chain id or Residue name contains any invalid characters.
    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        chain_id: char,
        residue_serial_number: usize,
        residue_name: [char; 3],
    ) {
        let mut found = false;
        let mut new_chain = Chain::new(chain_id).expect("Invalid characters in chain creation");
        let mut current_chain = &mut new_chain;
        for chain in &mut self.chains {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }
        if !found {
            // As this moves the chain the atom should be added later to keep the reference intact
            self.chains.push(new_chain);
            current_chain = (&mut self.chains).last_mut().unwrap();
        }

        current_chain.add_atom(new_atom, residue_serial_number, residue_name);
    }

    /// Add a new Atom to the hetero Atoms of this Model. It finds if there already is a Chain with the given `chain_id` if there is it will add this atom to that Chain, otherwise it will create a new Chain and add that to the list of Chains making up this Model. It does the same for the Residue, so it will create a new one if there does not yet exist a Residue with the given serial number.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `chain_id` - the id of the Chain to add the Atom to
    /// * `residue_serial_number` - the serial number of the Residue to add the Atom to
    /// * `residue_name` - the name of the Residue to add the Atom to, only used to create a new Residue if needed
    ///
    /// ## Panics
    /// It panics if the Chain id or Residue name contains any invalid characters.
    pub fn add_hetero_atom(
        &mut self,
        new_atom: Atom,
        chain_id: char,
        residue_serial_number: usize,
        residue_name: [char; 3],
    ) {
        let mut found = false;
        let mut new_chain = Chain::new(chain_id)
            .unwrap_or_else(|| panic!("Invalid characters in chain creation ({})", chain_id));
        let mut current_chain = &mut new_chain;
        for chain in &mut self.hetero_chains {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }
        if !found {
            self.hetero_chains.push(new_chain);
            current_chain = self.hetero_chains.last_mut().unwrap();
        }

        current_chain.add_atom(new_atom, residue_serial_number, residue_name);
    }

    /// Add a Chain to the list of Chains making up this Model. This does not detect any duplicates of names or serial numbers in the list of Chains.
    fn add_chain(&mut self, chain: Chain) {
        self.chains.push(chain);
    }

    /// Add a Chain to the list of Hetero Chains making up this Model. This does not detect any duplicates of names or serial numbers in the list of Chains.
    fn add_hetero_chain(&mut self, chain: Chain) {
        self.hetero_chains.push(chain);
    }

    /// Remove all Atoms matching the given predicate. The predicate will be run on all Atoms (Normal and Hetero).
    /// As this is done in place this is the fastest way to remove Atoms from this Model.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        for residue in self.all_residues_mut() {
            residue.remove_atoms_by(&predicate);
        }
    }

    /// Remove all Residues matching the given predicate. The predicate will be run on all Residues (Normal and Hetero).
    /// As this is done in place this is the fastest way to remove Residues from this Model.
    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        for chain in self.all_chains_mut() {
            chain.remove_residues_by(&predicate);
        }
    }

    /// Remove all Chains matching the given predicate. The predicate will be run on all Chains (Normal and Hetero).
    /// As this is done in place this is the fastest way to remove Chains from this Model.
    pub fn remove_chains_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Chain) -> bool,
    {
        let chains = std::mem::take(&mut self.chains);
        self.chains
            .extend(chains.into_iter().filter(|chain| !predicate(chain)));
        let hetero_chains = std::mem::take(&mut self.hetero_chains);
        self.hetero_chains
            .extend(hetero_chains.into_iter().filter(|chain| !predicate(chain)));
    }

    /// Remove the Chain specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_chain(&mut self, index: usize) {
        self.chains.remove(index);
    }

    /// Remove the Chain specified. It returns `true` if it found a matching Chain and removed it.
    /// It removes the first matching Chain from the list.
    ///
    /// ## Arguments
    /// * `id` - the id of the Chain to remove
    pub fn remove_chain_id(&mut self, id: char) -> bool {
        let index = self.chains.iter().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_chain(i);
            true
        } else {
            false
        }
    }

    /// Apply a transformation to the position of all atoms (Normal and Hetero) making up this Model, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.all_atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Model with another Model, this moves all atoms from the other Model
    /// to this Model. All other (meta) data of this Model will stay the same. It will add
    /// new Chains and residues as defined in the other model.
    pub fn join(&mut self, other: Model) {
        self.chains.extend(other.chains);
        self.hetero_chains.extend(other.hetero_chains);
    }
}

use std::fmt;
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MODEL SerialNumber:{}, Chains: {}",
            self.serial_number,
            self.chains.len() + self.hetero_chains.len()
        )
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        let mut model = Model::new(self.serial_number);
        model.chains = self.chains.clone();
        model.hetero_chains = self.hetero_chains.clone();
        model
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.serial_number == other.serial_number
            && self.chains == other.chains
            && self.hetero_chains == other.hetero_chains
    }
}
