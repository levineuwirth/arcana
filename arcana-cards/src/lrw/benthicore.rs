//! Benthicore — `{6}{U}` 5/5 blue Elemental.
//!
//! * "When this creature enters, create two 1/1 blue Merfolk Wizard
//!   creature tokens." An ETB trigger minting two tokens.
//! * "Tap two untapped Merfolk you control: Untap this creature. It
//!   gains shroud until end of turn." An activated ability whose cost is
//!   tapping two untapped Merfolk (`tap_other` + count 2); the effect
//!   untaps this creature and grants it shroud.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Benthicore");
    let elemental = reg.interner_mut().intern("Elemental");
    let _merfolk = reg.interner_mut().intern("Merfolk");
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let merfolk_filter = script::subtype_filter(reg, "Merfolk");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_merfolk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped Merfolk you control: Untap this creature. \
                       It gains shroud until end of turn."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(merfolk_filter),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_and_shroud,
            }),
    )
}

fn make_merfolk(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let merfolk = reg.interner().lookup("Merfolk").unwrap_or_default();
    let wizard = reg.interner().lookup("Wizard").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let token = TokenDefinition {
        name: merfolk,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn untap_and_shroud(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Untap { target: ctx.source },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Shroud,
            duration: Duration::EndOfTurn,
        },
    ]
}
