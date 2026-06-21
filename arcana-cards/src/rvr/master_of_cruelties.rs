//! Master of Cruelties — `{3}{B}{R}` 1/4 Demon with first strike and
//! deathtouch.
//!
//! Rules text:
//! * First strike, deathtouch (keyword line).
//! * "This creature can only attack alone." — a static attack
//!   restriction; not expressible as a triggered/activated ability.
//! * "Whenever this creature attacks a player and isn't blocked, that
//!   player's life total becomes 1. This creature assigns no combat
//!   damage this combat." — modeled as a `SelfAttacksUnblocked`
//!   trigger that sets the defending player's life to 1. The
//!   "assigns no combat damage" rider is a combat-replacement that
//!   has no expressible primitive — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Master of Cruelties");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: static "This creature can only attack alone." — no
    // attack-restriction primitive is exposed.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacksUnblocked,
                intervening_if: None,
                effect: attacks_unblocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "that player's life total becomes 1." The "assigns no combat
/// damage this combat" rider is unexpressible and GAP'd.
fn attacks_unblocked(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    // GAP: "This creature assigns no combat damage this combat." — no
    // combat-damage suppression primitive available.
    vec![Effect::SetLifeTotal { player: p, amount: 1 }]
}
