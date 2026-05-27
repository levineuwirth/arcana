//! A-Rockslide Sorcerer — `{2}{R}` 2/2 red Human Wizard Sorcerer.
//! "Whenever you cast an instant, sorcery, or Wizard spell, this
//! creature deals 1 damage to any target."
//! GAP: SpellCast filter cannot express "instant OR sorcery OR Wizard
//! creature subtype" — filter covers instant/sorcery only; Wizard creature
//! spells would not trigger.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
    ObjectOrPlayer,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Rockslide Sorcerer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(sorcerer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: covers instant/sorcery only; Wizard creature spells not covered
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_deal_1_any_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn cast_deal_1_any_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    match target {
        TargetChoice::Object(id) => vec![Effect::DealDamage {
            target: DamageTarget::Object(*id),
            amount: 1,
            source: trig.source,
        }],
        TargetChoice::Player(p) => vec![Effect::DealDamage {
            target: DamageTarget::Player(*p),
            amount: 1,
            source: trig.source,
        }],
        TargetChoice::ObjectOrPlayer(op) => match op {
            ObjectOrPlayer::Object(id) => vec![Effect::DealDamage {
                target: DamageTarget::Object(*id),
                amount: 1,
                source: trig.source,
            }],
            ObjectOrPlayer::Player(p) => vec![Effect::DealDamage {
                target: DamageTarget::Player(*p),
                amount: 1,
                source: trig.source,
            }],
        },
        _ => Vec::new(),
    }
}
