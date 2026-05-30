//! Invasion of Gobakhan // Lightshield Array
//!
//! Front face: {1}{W} Battle — Siege with 4 defense counters.
//! ETB: look at target opponent's hand; you may exile a nonland card from it.
//! For as long as that card remains exiled, its owner may play it but it costs
//! {2} more.
//! Back face (Lightshield Array): Enchantment.
//! At the beginning of your end step, put a +1/+1 counter on each creature
//! that attacked this turn.
//! Sacrifice this enchantment: Creatures you control gain hexproof and
//! indestructible until end of turn.
//!
//! GAPs:
//! - ETB "look at hand / exile nonland / owner may play it for {2} more":
//!   the hand-look, optional exile-from-hand, and cost-rider are not expressible
//!   with any catalog Effect variant. Emitting Vec::new() for the ETB resolver.
//! - "Each creature that attacked this turn" trigger: no script API for
//!   'attacked this turn' set. Emitting Vec::new() for that trigger.
//! - "Sacrifice this enchantment: creatures gain hexproof + indestructible" —
//!   activated ability on transform-back face not wired by current engine.
//! - GAP: defeat→cast-back-face not auto-wired (CR 310.11).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Gobakhan");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Lightshield Array — Enchantment
    let back_name = reg.interner_mut().intern("Lightshield Array");
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // ETB trigger targets a player (opponent)
    let etb_target = TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_resolve,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![etb_target],
            })
            .with_transform_back(back_face),
    )
}

fn etb_resolve(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at target opponent's hand; you may exile a nonland card from
    // it; for as long as that card remains exiled, its owner may play it but
    // costs {2} more" — no catalog Effect for hand-look, exile-from-hand, or
    // play-permission with cost modification.
    Vec::new()
}
