//! Vinereap Mentor — `{B}{G}` 3/2 Squirrel Druid. "When this
//! creature enters or dies, create a Food token." Two triggers
//! ride a single oracle clause; the engine has no
//! "enters-or-dies" composite, so we register two separate
//! `TriggeredAbilityDef`s (SelfEntersBattlefield + SelfDies) that
//! share one resolver. Food is emitted as a clean artifact token
//! with the `Food` subtype; the {2}{T}+sac activated ability is
//! deferred engine work (see token-recipes in the generator
//! prompt) and is intentionally NOT authored here.
//!
//! GAP: Scryfall lists "Food" as a keyword on this card. Food is
//! not a creature keyword in the engine's `KeywordAbility` set
//! (it's a token type recognised by subtype), so `keywords` is
//! left empty.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vinereap Mentor");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let druid = reg.interner_mut().intern("Druid");
    // Pre-intern the Food subtype so the resolver can look it up
    // through the non-mut interner at trigger-resolution time.
    let _food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_food_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: create_food_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Shared resolver for both the ETB and the dies trigger: the
/// ability's controller creates one colorless Food artifact token.
fn create_food_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let food = reg
        .interner()
        .lookup("Food")
        .expect("Food interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
