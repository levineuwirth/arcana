//! Bone-Cairn Butcher — `{1}{R}{W}{B}` 4/4 Demon.
//!
//! * Mobilize 2 (Whenever this creature attacks, create two tapped and
//!   attacking 1/1 red Warrior creature tokens. Sacrifice them at the
//!   beginning of the next end step.) → Mobilize is not in the usable keyword
//!   surface, but its triggered ability is wired: an attack trigger that
//!   mints two tapped-and-attacking 1/1 red Warrior tokens.
//!   // GAP: "Sacrifice them at the beginning of the next end step" — the
//!   token ids are minted engine-side and unknown to the resolver, so no
//!   `DelayedAction` can reference them; the end-step sacrifice is unmodeled.
//! * Attacking tokens you control have deathtouch.
//!   // GAP: a static keyword-granting anthem ("attacking tokens you control
//!   have deathtouch") has no expressible Effect form; unmodeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bone-Cairn Butcher");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    // Token subtype interned ahead of resolution.
    let _warrior = reg.interner_mut().intern("Warrior");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: mobilize_two_warriors,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn mobilize_two_warriors(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: warrior,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateTokenTappedAttacking {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateTokenTappedAttacking {
            controller: trig.controller,
            token,
        },
    ]
}
