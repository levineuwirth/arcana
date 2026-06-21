//! Rakdos, the Muscle — `{2}{B}{B}{R}` 6/5 legendary Demon Mercenary
//! with Flying and Trample.
//! - Whenever you sacrifice another creature, exile cards equal to its
//!   mana value from the top of target player's library; until your next
//!   end step you may play those cards (GAP'd — see below).
//! - Sacrifice another creature: Rakdos gains indestructible until end of
//!   turn. Tap it. Activate only once each turn.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos, the Muscle");
    let demon = reg.interner_mut().intern("Demon");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: sac_exile_for_mv,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another creature: Rakdos gains indestructible until end of turn. Tap it. Activate only once each turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter::creature()),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: indestructible_and_tap,
            }),
    )
}

fn sac_exile_for_mv(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile cards equal to its mana value from the top of target
    // player's library" — no accessor for the sacrificed creature's mana
    // value, and ImpulseExile exiles from the controller's own library
    // (not a target player's), nor supports the "mana of any type" rider.
    Vec::new()
}

fn indestructible_and_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::Tap { target: ctx.source },
    ]
}
