//! Yuriko, the Tiger's Shadow — `{1}{U}{B}` 1/3 Legendary Human Ninja.
//! "Commander ninjutsu {U}{B}. Whenever a Ninja you control deals
//! combat damage to a player, reveal the top card of your library and
//! put that card into your hand. Each opponent loses life equal to
//! that card's mana value."
//!
//! GAP: Commander ninjutsu is not a `KeywordAbility` variant — the
//! alternate put-into-play mechanic is unmodeled (keywords empty).
//! The combat-damage trigger is wired (a Ninja you control dealing
//! combat damage to a player), but its effect — reveal the top card,
//! put it into hand, then make each opponent lose life equal to that
//! card's mana value — has no expressible primitive (no reveal-top-
//! and-read-mana-value-for-life-loss effect), so the resolver GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yuriko, the Tiger's Shadow");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    // "a Ninja you control" — controlled-by-you, subtype Ninja. Built
    // after all mutable interns (subtype_filter borrows reg immutably).
    let ninja_filter =
        script::subtype_filter(reg, "Ninja").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ninja_filter,
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: ninja_damage_reveal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn ninja_damage_reveal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal top card + put into hand + each opponent loses life
    // equal to that card's mana value is not expressible with the
    // available primitives (no reveal-top-and-read-mv-for-life-loss
    // effect).
    Vec::new()
}
