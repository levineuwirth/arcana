//! Sawtooth Avenger — `{5}` 0/0 Artifact Creature — Golem Construct.
//!
//! Oracle:
//! * Megasunburst (This enters the battlefield with two +1/+1 counters on it
//!   for each color of mana spent to cast it.)
//! * Remove three +1/+1 counters from Sawtooth Avenger: Sawtooth Avenger gains
//!   your choice of deathtouch, lifelink, or menace until end of turn.
//!
//! Decomposition: one activated ability whose cost is "remove three +1/+1
//! counters".
//!
//! GAP: Megasunburst (enters with two +1/+1 counters per color of mana spent)
//!      has no ETB-counters-per-color-of-mana primitive (cf. Sunburst) —
//!      omitted.
//! GAP: the activated ability's effect "gains your choice of deathtouch,
//!      lifelink, or menace" is a modal choice; triggered/activated abilities
//!      have no modal mechanism (only spell abilities carry `modal`). The cost
//!      is faithful; the chosen-keyword grant is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sawtooth Avenger");
    let golem = reg.interner_mut().intern("Golem");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove three +1/+1 counters from Sawtooth Avenger: Sawtooth Avenger gains \
                   your choice of deathtouch, lifelink, or menace until end of turn."
                .into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::PlusOnePlusOne, 3)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: choose_keyword,
        }),
    )
}

fn choose_keyword(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gains your choice of deathtouch, lifelink, or menace" is a modal
    //      choice; activated abilities carry no modal mechanism. Omitted.
    Vec::new()
}
