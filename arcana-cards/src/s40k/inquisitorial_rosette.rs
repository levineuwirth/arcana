//! Inquisitorial Rosette — `{2}` artifact — Equipment (Warhammer 40,000).
//! "Inquisition Agents — Whenever equipped creature attacks, create a
//! 2/2 white Astartes Warrior creature token with vigilance that's
//! attacking. Then attacking creatures gain menace until end of turn.
//! Equip {3}". Only the Equip half is expressible: there is no
//! "whenever equipped creature attacks" trigger condition in the
//! catalog, and "token that's attacking" / "attacking creatures gain
//! menace" are not expressible either, so the rider is a GAP.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inquisitorial Rosette");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: "Inquisition Agents — Whenever equipped creature attacks, create a
    // 2/2 white Astartes Warrior creature token with vigilance that's attacking.
    // Then attacking creatures gain menace until end of turn." — no
    // equipped-creature-attacks trigger condition, no enters-attacking token,
    // and no board-wide attacking-creatures keyword grant in this API surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{3}").expect("valid cost")),
    )
}
