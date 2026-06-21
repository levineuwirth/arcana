//! Nashi, Moon Sage's Scion — `{1}{B}{B}` 3/2 Legendary Creature — Rat
//! Ninja. Black.
//! Ninjutsu {3}{B} (GAP'd — no KeywordAbility::Ninjutsu variant in the
//! demonstrated keyword surface; the alternative-cost cast mechanic is
//! not modeled).
//! "Whenever Nashi deals combat damage to a player, exile the top card of
//!  each player's library. Until end of turn, you may play one of those
//!  cards. If you cast a spell this way, pay life equal to its mana value
//!  rather than paying its mana cost."
//!
//! The combat-damage trigger is wired (self source via name filter,
//! player target, combat only). Its effect is GAP'd: there is no Effect
//! that exiles the top card of EACH player's library with a
//! play-one-until-end-of-turn permission and a life-equal-to-mana-value
//! alternative cost. (Effect::ImpulseExile only exiles your own library
//! and grants normal-cost play.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nashi, Moon Sage's Scion");
    let rat = reg.interner_mut().intern("Rat");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(ninja);

    // Self-source filter for the combat-damage trigger, by Nashi's name.
    let self_filter = ObjectFilter {
        name: Some(name),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ninjutsu {3}{B} — no KeywordAbility::Ninjutsu variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: nashi_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn nashi_combat_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile top of EACH player's library + play-one-until-EOT +
    // life-equal-to-mana-value alternative cost — not expressible.
    Vec::new()
}
