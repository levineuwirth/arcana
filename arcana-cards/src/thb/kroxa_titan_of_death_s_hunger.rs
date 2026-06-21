//! Kroxa, Titan of Death's Hunger — `{B}{R}` 6/6 Legendary Elder Giant.
//! "When Kroxa enters, sacrifice it unless it escaped." — GAP: there is
//! no way to read whether the spell was cast for its escape cost, so the
//! conditional self-sacrifice is omitted (firing it unconditionally would
//! be wrong).
//! "Whenever Kroxa enters or attacks, each opponent discards a card,
//! then each opponent who didn't discard a nonland card this way loses 3
//! life." — modeled as two triggers (enters / attacks). The discard is
//! wired for each opponent; the "who didn't discard a nonland card" life
//! loss rider is GAP'd (no per-opponent discarded-card-type accessor).
//! "Escape—{B}{B}{R}{R}, Exile five other cards from your graveyard." —
//! GAP: Escape is not an available KeywordAbility variant.

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Kroxa, Titan of Death's Hunger");
    let elder = reg.interner_mut().intern("Elder");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![], // GAP: Escape unmodeled
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "When Kroxa enters, sacrifice it unless it escaped" —
            // escaped-state is unreadable, so this trigger is omitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: each_opponent_discards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: each_opponent_discards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_opponent_discards(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each opponent discards a card" — one Discard per opponent.
    // GAP: the "then each opponent who didn't discard a nonland card this
    // way loses 3 life" rider needs a per-opponent discarded-card-type
    // accessor that isn't available; the discard half is wired.
    let opps = script::opponents(state, trig.controller);
    opps.into_iter()
        .map(|p| Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect()
}
