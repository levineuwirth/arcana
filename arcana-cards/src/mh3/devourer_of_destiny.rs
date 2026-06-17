//! Devourer of Destiny — `{5}{C}{C}` 6/6 Eldrazi (colorless).
//!
//! * You may reveal this card from your opening hand. If you do, at the beginning
//!   of your first upkeep, look at the top four cards ... — GAP'd (no opening-hand
//!   reveal / first-upkeep machinery).
//! * When you cast this spell, exile target permanent that's one or more colors.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devourer of Destiny");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{C}{C}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // Restrict the cast trigger to this card by name so only casting THIS spell fires it.
    let name_filter = ObjectFilter { name: Some(name), ..ObjectFilter::default() };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: opening-hand reveal + first-upkeep dig is not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(name_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: exile_colored_permanent,
                trigger_zones: vec![Zone::Battlefield, Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                // GAP: target restriction "that's one or more colors" not expressible
                // (no colored/has-any-color predicate); allows any permanent target.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn exile_colored_permanent(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: *id }]
}
