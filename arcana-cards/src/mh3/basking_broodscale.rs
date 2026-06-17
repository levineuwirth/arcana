//! Basking Broodscale — `{1}{G}` 2/2 Eldrazi Lizard. Devoid (colorless).
//! "{1}{G}: Adapt 1." (put a +1/+1 counter; the "only if it has no
//!  +1/+1 counters" Adapt rider is GAP'd.)
//! "Whenever one or more +1/+1 counters are put on this creature, you
//!  may create a 0/1 colorless Eldrazi Spawn creature token with
//!  'Sacrifice this token: Add {C}.'" (token created; its sac-for-mana
//!  ability is GAP'd.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basking Broodscale");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let lizard = reg.interner_mut().intern("Lizard");
    let _spawn = reg.interner_mut().intern("Eldrazi Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        // Devoid — this card has no color.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Devoid / Adapt are not in the usable keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}: Adapt 1.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_one,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: arcana_core::triggers::TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: make_spawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn adapt_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Adapt's "only if this creature has no +1/+1 counters" rider.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn make_spawn(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Eldrazi Spawn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spawn);
    // GAP: token's printed "Sacrifice this token: Add {C}." ability.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spawn,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
