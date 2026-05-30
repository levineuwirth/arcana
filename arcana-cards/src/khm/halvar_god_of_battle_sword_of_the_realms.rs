//! Halvar, God of Battle // Sword of the Realms — `{2}{W}{W}` white MDFC.
//! Front: Legendary Creature — God 4/4, white.
//!   "Creatures you control that are enchanted or equipped have double strike."
//!   "At the beginning of each combat, you may attach target Aura or Equipment
//!    attached to a creature you control to target creature you control."
//! Back: Legendary Artifact — Equipment (cast for `{1}{W}`).
//!   "Equipped creature gets +2/+0 and has vigilance."
//!   "Whenever equipped creature dies, return it to its owner's hand."
//!   "Equip {1}{W}"
//! GAP: "creatures enchanted or equipped have double strike" — continuous effect
//!      conditioned on enchanted/equipped status not expressible in current engine.
//! GAP: "attach target Aura or Equipment to target creature" — attach-reroute at
//!      beginning of combat not expressible (no Aura/Equipment filter for targets,
//!      no reroute-attach effect).
//! GAP: "whenever equipped creature dies, return it to its owner's hand" —
//!      back-face-only triggered ability; triggers live on CardDefinition not on face.
//! Back-face pump (+2/+0, vigilance) installed as an ETB/attach continuous effect
//! via with_equip; the pump proper is a GAP (attached_pt only does P/T not keywords).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Halvar, God of Battle");
    let god = reg.interner_mut().intern("God");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "creatures enchanted or equipped have double strike" — continuous
        // effect conditioned on enchanted/equipped status not expressible.
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Sword of the Realms");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(equipment);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid back cost")),
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT),
            subtypes: back_subs,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // GAP: "Creatures you control that are enchanted or equipped have double strike"
            // — continuous effect conditioned on enchanted/equipped status not expressible.
            // GAP: "At the beginning of each combat, you may attach target Aura or Equipment
            // to target creature" — attach-reroute trigger not expressible.
            // Back face equip {1}{W}: the equip activated ability for the back face.
            // Note: activated abilities here are shared and apply when the object on the
            // battlefield has the back-face Equipment characteristics.
            .with_equip(ManaCost::parse("{1}{W}").expect("valid equip cost")),
        // GAP: back-face triggered "whenever equipped creature dies, return it to its
        // owner's hand" — back-face-only triggered ability not modeled (engine debt).
    )
}
