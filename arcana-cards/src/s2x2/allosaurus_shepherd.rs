//! Allosaurus Shepherd — `{G}` 1/1 Elf Shaman.
//! "This spell can't be countered.
//!  Green spells you control can't be countered.
//!  {4}{G}{G}: Until end of turn, each Elf creature you control has base
//!  power and toughness 5/5 and becomes a Dinosaur in addition to its
//!  other creature types."
//!
//! Decomposed as: one activated ability setting each Elf you control to
//! base 5/5 until end of turn. The two "can't be countered" lines are
//! static replacement-like effects with no demonstrated primitive; the
//! "becomes a Dinosaur" subtype addition has no subtype-granting effect.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Allosaurus Shepherd");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "This spell can't be countered" — no can't-be-countered
    // primitive.
    // GAP: static "Green spells you control can't be countered."
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}{G}{G}: Until end of turn, each Elf creature you control has base \
                   power and toughness 5/5 and becomes a Dinosaur in addition to its other \
                   creature types."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{G}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_elves,
        }),
    )
}

fn pump_elves(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "becomes a Dinosaur in addition to its other creature types" —
    // no subtype-granting effect (AddType only adds card types).
    let filter = script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    ids.into_iter()
        .map(|id| Effect::SetBasePT {
            target: id,
            power: 5,
            toughness: 5,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
