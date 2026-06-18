//! Chiss-Goria, Forge Tyrant — `{6}{R}{R}{R}` 5/4 legendary Dragon with
//! Flying, Haste. (Affinity for artifacts is a cost-reduction static — GAP'd;
//! "Affinity" is not in the usable keyword surface.)
//! Attack: exile the top five cards of your library; you may cast an artifact
//! spell from among them this turn (modeled as impulse exile — the artifact
//! restriction and "has affinity" rider are partials).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chiss-Goria, Forge Tyrant");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "Affinity for artifacts" cost reduction is a static and "Affinity"
    // is not in the usable keyword surface.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_impulse,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_impulse(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Partial: "you may cast an ARTIFACT spell from among them" and "if you do
    // it has affinity for artifacts" — modeled as plain impulse exile of 5;
    // the artifact-only restriction and affinity rider are not expressible.
    vec![Effect::ImpulseExile { player: trig.controller, count: 5 }]
}
