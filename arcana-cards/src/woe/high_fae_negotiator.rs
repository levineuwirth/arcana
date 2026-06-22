//! High Fae Negotiator — `{3}{B}{B}` 3/5 black Faerie Warlock.
//!
//! * Bargain (you may sacrifice an artifact, enchantment, or token as you
//!   cast this spell) — not an expressible keyword; recorded as a GAP.
//! * Flying.
//! * When this creature enters, if it was bargained, each opponent loses 3
//!   life and you gain 3 life.
//!
//! The "if it was bargained" intervening-if has no expressible condition
//! (the engine exposes no "was-bargained" predicate), so the gate is GAP'd
//! and the life-swing effect is wired to fire on ETB.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("High Fae Negotiator");
    let faerie = reg.interner_mut().intern("Faerie");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Bargain (sacrifice an artifact, enchantment, or token as you
        // cast this spell) is not an expressible keyword.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if it was bargained" — no engine predicate
            // for the bargained flag; the life-swing effect fires on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Each opponent loses 3 life and you gain 3 life.
fn etb_drain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 3 })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: 3 });
    effects
}
