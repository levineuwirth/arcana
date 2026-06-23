//! Myr Galvanizer — `{3}` 2/2 Artifact Creature — Myr.
//!
//! Oracle:
//! * "Other Myr creatures you control get +1/+1." — GAP (static anthem
//!   continuous effect; no trigger word, no activation cost).
//! * "{1}, {T}: Untap each other Myr you control." — an activated ability with
//!   a mana + tap cost that untaps every OTHER Myr you control (this card
//!   excluded), wired via `Effect::ForEach` over the matching ids.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myr Galvanizer");
    let myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "Other Myr creatures you control get +1/+1." — a continuous
    // anthem with no trigger/cost; not expressible in this card class.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}: Untap each other Myr you control.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: untap_other_myr,
        }),
    )
}

fn untap_other_myr(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Myr").controlled_by(ControllerConstraint::You);
    let targets: Vec<_> = script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .filter(|id| *id != ctx.source)
        .collect();
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::Untap {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
