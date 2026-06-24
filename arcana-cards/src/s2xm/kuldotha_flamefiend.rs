//! Kuldotha Flamefiend — `{4}{R}{R}` 4/4 red Elemental. "When this creature
//! enters, you may sacrifice an artifact. If you do, this creature deals 4
//! damage divided as you choose among any number of targets."

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kuldotha Flamefiend");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: divided_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Any,
                    controller: None,
                }],
            }),
    )
}

fn divided_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may sacrifice an artifact. If you do, this creature deals 4 damage
    // divided as you choose among any number of targets." The optional
    // sacrifice gates the divided-damage rider (the `then`).
    let targets: Vec<DamageTarget> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
                Some(DamageTarget::Object(*id))
            }
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
                Some(DamageTarget::Player(*p))
            }
        })
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Artifact),
        then: Box::new(Effect::DealDamageDivided {
            source: trig.source,
            targets,
            total: 4,
        }),
        else_effect: None,
    }]
}
