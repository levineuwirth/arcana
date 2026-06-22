//! Gate Colossus — `{8}` 8/8 Artifact Creature — Construct.
//! Affinity for Gates.
//! This creature can't be blocked by creatures with power 2 or less.
//! Whenever a Gate you control enters, you may put this card from your
//! graveyard on top of your library.
//!
//! GAP (keyword): Affinity is not in the usable KeywordAbility surface (a
//! cost-reduction cast mechanic) — recorded but unwired.
//! GAP (static): "can't be blocked by creatures with power 2 or less" is a
//! filtered evasion static; Effect::CantBeBlocked is all-or-nothing and there
//! is no power-filtered block restriction, so this is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gate Colossus");
    let construct = reg.interner_mut().intern("Construct");
    let gate = reg.interner_mut().intern("Gate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    let gate_filter = ObjectFilter::new()
        .with_subtype_sym(gate)
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: gate_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: gate_enters_recur,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gate_enters_recur(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "you may put this card from your graveyard on top of your library."
    // ("may" is a resolution-time choice — minor fidelity gap.)
    vec![Effect::PutOnTopOfLibrary { target: trig.source }]
}
