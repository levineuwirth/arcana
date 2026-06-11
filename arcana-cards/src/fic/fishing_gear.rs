//! Fishing Gear — `{3}` artifact — Equipment (Bloomburrow).
//! "Whenever equipped creature deals combat damage to a player, exile
//! the top card of that player's library. If it's a permanent card,
//! you may put it onto the battlefield under your control. If you
//! don't, create a 1/1 blue Fish creature token. Equip {2}"

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fishing Gear");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "equipped creature" — ObjectFilter cannot express
                // "the creature this Equipment is attached to";
                // approximated as any creature you control (over-fires
                // when other creatures you control connect).
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: go_fishing,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…exile the top card of that player's library. If it's a permanent
/// card, you may put it onto the battlefield under your control. If
/// you don't, create a 1/1 blue Fish creature token."
fn go_fishing(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exiling the top card of ANOTHER player's library, branching
    // on whether it is a permanent card, and the may-steal-or-Fish
    // choice are not expressible with the catalog (ImpulseExile is
    // self-library only; Effect::Conditional has no card-type-of-
    // exiled-card condition).
    Vec::new()
}
