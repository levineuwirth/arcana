//! Steady Tortoise // Harried Dash
//!
//! Creature face: {4}{G} Turtle 3/3, Ward {2}.
//! "Whenever you attack, Steady Tortoise perpetually gets +1/+1. This ability
//!  also triggers if Steady Tortoise is in exile." — 'perpetually gets +1/+1'
//!  is an Arena-specific Alchemy mechanic (permanent counter gain across game
//!  zones); no catalog Effect for this. GAP noted below.
//!
//! Adventure face: "Harried Dash" {R} Sorcery — Adventure.
//! "Create a 1/1 white Rabbit creature token. It gains haste until end of turn."
//!
//! GAPs:
//! - "Perpetually gets +1/+1" — Alchemy/Arena mechanic; no Effect::Perpetually
//!   variant. The attack-trigger condition is also not in the TriggerCondition
//!   catalog; entire trigger omitted.
//! - "It gains haste until end of turn" on the newly created token cannot be
//!   targeted at creation time; haste is encoded in the token's keywords as a
//!   best-effort approximation (slight fidelity gap: it becomes a permanent
//!   keyword rather than an until-end-of-turn grant).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Steady Tortoise");
    let turtle_sub = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{2}").expect("valid ward cost"),
        )],
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Harried Dash");
    // Pre-intern the Rabbit subtype string so resolve can look it up
    let _rabbit_sub = reg.interner_mut().intern("Rabbit");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a 1/1 white Rabbit creature token. It gains haste until end of turn."
            .into(),
        target_requirements: vec![],
        modal: None,
        effect: harried_dash_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    // GAP: "Whenever you attack, Steady Tortoise perpetually gets +1/+1.
    // This ability also triggers if Steady Tortoise is in exile." —
    // TriggerCondition::Attacks is not in the catalog; entire trigger omitted.

    reg.register(CardDefinition::new(name, chars).with_adventure(adventure))
}

fn harried_dash_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Pre-interned during registration; unwrap_or(0) is safe since we interned it.
    let rabbit_name = reg.interner().lookup("Rabbit").unwrap_or(0);
    let mut rabbit_subtypes = SubtypeSet::default();
    rabbit_subtypes.0.insert(rabbit_name);
    // GAP: "it gains haste until end of turn" targets the newly created token
    // which has no id at resolution time; haste encoded in token keywords as
    // best-effort approximation.
    let token = TokenDefinition {
        name: rabbit_name,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: rabbit_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }]
}
