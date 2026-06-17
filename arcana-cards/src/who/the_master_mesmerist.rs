//! The Master, Mesmerist — `{2}{U}{B}` 3/3 Legendary Creature — Time Lord Rogue.
//! Keywords: Goad.
//! {T}: Target creature an opponent controls with power <= The Master's power
//!   gains skulk until end of turn. Goad it.
//! Whenever a creature with skulk deals combat damage to one of your opponents,
//!   put a +1/+1 counter on The Master and draw a card.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Master, Mesmerist");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = arcana_core::types::SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // "Goad" (Scryfall keyword) is not a standalone KeywordAbility variant;
        // it appears only via the activated ability's Effect::Goad below.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature an opponent controls with power less than or equal to The Master's power gains skulk until end of turn. Goad it.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: filter cannot restrict to "power <= The Master's power" (dynamic power compare not expressible on TargetRequirement filters)
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: skulk_and_goad,
            })
            // GAP: trigger filter "a creature with skulk" cannot be expressed (no keyword predicate on DamageDealt source_filter shown); using broad creature source as closest, draw+counter on this card
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: counter_and_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn skulk_and_goad(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Skulk,
            duration: Duration::EndOfTurn,
        },
        Effect::Goad {
            target: *id,
            goader: ctx.controller,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn counter_and_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
