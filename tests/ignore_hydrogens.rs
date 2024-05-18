// Test reading PDB file and discarding all hydrogen atoms.

use pdbtbx::*;

#[test]
fn main() {
    // PDB parser
    assert_eq!(
        947,
        count_hydrogens("example-pdbs/rosetta_model.pdb", false)
    );
    assert_eq!(0, count_hydrogens("example-pdbs/rosetta_model.pdb", true));

    // mmCIF parser
    assert_eq!(
        947,
        count_hydrogens("example-pdbs/rosetta_model.cif", false)
    );
    assert_eq!(0, count_hydrogens("example-pdbs/rosetta_model.cif", true));
}

fn count_hydrogens(filename: &str, discard_hydrogens: bool) -> usize {
    let (structure, _errors) = pdbtbx::open_with_options(
        filename,
        ReadOptions::default()
            .set_level(StrictnessLevel::Loose)
            .set_discard_hydrogens(discard_hydrogens),
    )
    .unwrap();

    structure.atoms().fold(0, |acc, a| {
        acc + usize::from(a.element() == Some(&Element::H))
    })
}
