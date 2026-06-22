//! Junji, the Midnight Sky — `{3}{B}{B}` 5/5 Legendary Dragon Spirit
//! with Flying and Menace.
//! When Junji dies, choose one —
//! • Each opponent discards two cards and loses 2 life.
//! • Put target non-Dragon creature card from a graveyard onto the
//!   battlefield under your control. You lose 2 life.
//!
//! GAP: the engine has no modal dispatch for TRIGGERED abilities (modal
//! support is spell-ability-only). The dies trigger is wired to the
//! FIRST mode (each opponent discards two and loses 2 life — fully
//! expressible and non-targeted). The second mode (reanimate a
//! non-Dragon creature card under your control, lose 2 life) and the
//! "choose one" selection are GAP'd for a human to route once trigger
//! modal dispatch exists.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Junji, the Midnight Sky");
    let dragon = reg.interner_mut().intern("Dragon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_punish_opponents,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Dies trigger (first mode) — each opponent discards two cards and
/// loses 2 life.
fn dies_punish_opponents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for p in script::opponents(state, trig.controller) {
        effects.push(Effect::Discard {
            player: p,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        });
        effects.push(Effect::LoseLife { player: p, amount: 2 });
    }
    vec![Effect::Sequence(effects)]
}
