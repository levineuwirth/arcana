//! Shaun & Rebecca, Agents — `{1}{G}{W}{U}` Legendary 4/4 Human Assassin
//! Scientist with Vigilance.
//!
//! Oracle:
//! * Vigilance (keyword line).
//! * When Shaun & Rebecca enters, search your graveyard, hand, and library
//!   for a card named The Animus and put it onto the battlefield, then
//!   shuffle. (PARTIAL: only the library search is expressible via
//!   `TutorToBattlefield` by name; the graveyard/hand search is a GAP.)
//! * {T}: Add {C}. When you do, mill two cards. (Modeled as a non-mana
//!   activated ability that adds {C} and then mills two — the reflexive
//!   "when you do" rider is folded into the same resolution.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shaun & Rebecca, Agents");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let scientist = reg.interner_mut().intern("Scientist");
    // Pre-intern the tutored card name so the resolver lookup succeeds.
    let _ = reg.interner_mut().intern("The Animus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_the_animus,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}. When you do, mill two cards.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_add_colorless_then_mill,
            }),
    )
}

/// ETB: search for a card named The Animus and put it onto the battlefield.
/// Only the library search is expressible by name.
fn etb_tutor_the_animus(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("The Animus");
    // GAP: graveyard + hand search not expressible; only the library is
    // searched via TutorToBattlefield.
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        tapped: false,
    }]
}

/// {T}: Add {C}. When you do, mill two cards.
fn tap_add_colorless_then_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
        },
        Effect::Mill { player: ctx.controller, count: 2 },
    ]
}
