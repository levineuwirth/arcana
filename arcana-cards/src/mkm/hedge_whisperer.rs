//! Hedge Whisperer — `{G}` 0/3 Elf Druid Detective.
//! You may choose not to untap this creature during your untap step (GAP —
//! optional-untap static not expressible).
//! {3}{G}, {T}, Collect evidence 4: Target land you control becomes a 5/5
//! green Plant Boar creature with haste for as long as this creature remains
//! tapped. It's still a land. (GAP — the "Collect evidence" cost is not an
//! ActivationCost field, and the "as long as tapped" land-animation isn't
//! expressible.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hedge Whisperer");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    subtypes.0.insert(detective);

    // GAP: keyword — "Collect evidence" is not in the usable keyword surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static — optional "choose not to untap" is not expressible.
    // GAP: activated — "{3}{G}, {T}, Collect evidence 4: land becomes a 5/5
    // for as long as tapped" needs a Collect-evidence cost field and a
    // tap-conditional land animation, neither available.
    reg.register(CardDefinition::new(name, chars))
}
