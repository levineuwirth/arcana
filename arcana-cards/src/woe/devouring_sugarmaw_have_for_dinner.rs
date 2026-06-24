//! Devouring Sugarmaw // Have for Dinner — `{2}{B}{B}` Horror creature 6/6
//! with Menace and Trample. At the beginning of your upkeep, you may
//! sacrifice an artifact, enchantment, or token. If you don't, tap this
//! creature.
//! Adventure face "Have for Dinner" (`{1}{W}` instant): Create a 1/1 white
//! Human creature token and a Food token.
//!
//! # GAP
//! The upkeep triggered ability "you may sacrifice an artifact, enchantment,
//! or token — if you don't, tap this creature" pays by sacrificing one of an
//! artifact/enchantment/token disjunction. SacrificeFilter has no variant for
//! that class (NonLand is strictly more permissive — it would also let you
//! sacrifice a creature/planeswalker, which the oracle forbids; and there is no
//! token-only class). No faithful single-mode mapping exists, so the optional
//! sacrifice stays GAP'd: the tap consequence only is emitted (the creature
//! always taps at upkeep).

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devouring Sugarmaw");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror_sub);
    // Pre-intern token subtypes so resolve-time lookup succeeds.
    let _human_intern = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Trample],
        ..Default::default()
    };

    // Adventure face "Have for Dinner"
    let adv_name = reg.interner_mut().intern("Have for Dinner");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid adv cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a 1/1 white Human creature token and a Food token.".into(),
        target_requirements: vec![],
        modal: None,
        effect: have_for_dinner_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn upkeep_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice an artifact, enchantment, or token — if you
    // don't, tap this creature" — SacrificeFilter has no artifact/enchantment/
    // token disjunction (NonLand is over-permissive, no token-only class), so no
    // faithful single-mode payment exists. Emitting only the tap consequence
    // (always taps).
    vec![Effect::Tap { target: trig.source }]
}

fn have_for_dinner_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").expect("interned at register");
    let mut human_subtypes = SubtypeSet::default();
    human_subtypes.0.insert(human);
    vec![
        Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: human,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: human_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateCommodityToken {
            controller: entry.controller,
            kind: CommodityToken::Food,
            count: 1,
        },
    ]
}
