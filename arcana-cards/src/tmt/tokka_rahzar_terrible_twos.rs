//! Tokka & Rahzar, Terrible Twos — `{B/R}{B/R}` 3/2 Legendary Turtle Wolf Mutant.
//! This spell can't be countered. (GAP — static uncounterable not expressible)
//! Menace.
//! Whenever a player casts a spell, if the amount of mana spent to cast it was
//! less than its mana value, deal 3 damage to that player.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tokka & Rahzar, Terrible Twos");
    let turtle = reg.interner_mut().intern("Turtle");
    let wolf = reg.interner_mut().intern("Wolf");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    subtypes.0.insert(wolf);
    subtypes.0.insert(mutant);
    // GAP: "This spell can't be countered" — no static uncounterable primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: deal_3_if_underpaid,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn deal_3_if_underpaid(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: gate "if the amount of mana spent was less than its mana value" is not
    // expressible (no intervening-if / accessor for mana-spent vs mana-value).
    // Firing the 3 damage unconditionally would be materially wrong, so the whole
    // effect is omitted.
    Vec::new()
}
