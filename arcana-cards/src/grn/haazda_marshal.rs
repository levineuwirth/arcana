//! Haazda Marshal — `{W}` 1/1 white Human Soldier creature.
//! "Whenever this creature and at least two other creatures attack, create a 1/1 white
//! Soldier creature token with lifelink."
//!
//! # Notes
//! GAP: "this creature and at least two other creatures attack" — no compound attack condition.
//! SelfAttacks fires whenever this creature attacks; the "at least two other" condition is
//! evaluated inline using script::count_matching on attacking creatures.
//! GAP: no script helper for "creatures currently attacking" — using battlefield creature count.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haazda Marshal");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_with_two_more_soldiers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_with_two_more_soldiers(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at least two other creatures attack" — no script helper for attacking count;
    // using total battlefield creature count >= 3 as approximation.
    let creature_count = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if creature_count < 3 {
        return Vec::new();
    }
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
