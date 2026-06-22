//! A-Nashi, Moon Sage's Scion — `{1}{B}{B}` 3/2 Legendary Rat Ninja (black).
//!
//! Oracle:
//! * "Ninjutsu {2}{B}" — Ninjutsu is not in the usable keyword surface and its
//!   "return an unblocked attacker, put this onto the battlefield tapped and
//!   attacking" cast mechanic has no matching ActivationCost/effect shape.
//!   GAP'd (the keyword is recorded only via this note).
//! * "Whenever Nashi deals combat damage to a player, exile the top card of each
//!   player's library. Until end of turn, you may play one of those cards. If
//!   you cast a spell this way, pay life equal to its mana value rather than
//!   paying its mana cost." — the combat-damage trigger is wired, but the payoff
//!   is GAP'd: `Effect::ImpulseExile` exiles the top N of YOUR library only (not
//!   one card from EACH player's library), and the "pay life equal to mana
//!   value instead of mana cost" alternative cost has no primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Nashi, Moon Sage's Scion");
    let rat = reg.interner_mut().intern("Rat");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(ninja);

    // GAP: keyword "Ninjutsu {2}{B}" — not in the usable keyword surface; the
    // return-unblocked-attacker cast mechanic is not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_impulse_each_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_impulse_each_player(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of each player's library. Until end of turn, you
    // may play one of those cards. If you cast a spell this way, pay life equal
    // to its mana value rather than its mana cost." ImpulseExile only exiles
    // the top N of YOUR library (not one from each player's), and the
    // pay-life-instead-of-mana alternative cost has no primitive.
    Vec::new()
}
