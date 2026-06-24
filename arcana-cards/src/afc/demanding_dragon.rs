//! Demanding Dragon — `{3}{R}{R}` 5/5 red Dragon with Flying.
//! "When this creature enters, it deals 5 damage to target opponent unless
//! that player sacrifices a creature of their choice."
//!
//! The targeted opponent may sacrifice a creature to avoid the 5 damage —
//! an OptionalPayment whose chooser is that player (pay = sacrifice a creature,
//! decline = take 5 damage).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Demanding Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_demand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn etb_demand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // "...unless that player sacrifices a creature of their choice." The
    // opponent chooses: pay (sacrifice a creature) avoids the damage; decline
    // takes 5.
    vec![Effect::OptionalPayment {
        chooser: *p,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Creature),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(*p),
            amount: 5,
        })),
    }]
}
