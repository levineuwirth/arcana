//! Kami of Fire's Roar — `{3}{R}` 2/3 red Spirit. "Whenever you cast a
//! Spirit or Arcane spell, target creature can't block this turn."
//! Filtered cast trigger (Spirit OR Arcane subtype) + a targeted
//! "can't block this turn" effect.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kami of Fire's Roar");
    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![spirit, arcane]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: forbid_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn forbid_block(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ForbidBlocking {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}
