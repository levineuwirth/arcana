//! Dream-Thief's Bandana — `{2}` artifact — Equipment.
//! "Whenever equipped creature deals combat damage to a player, look at
//! the top card of their library, then exile it face down. For as long
//! as it remains exiled, you may play it, and mana of any type can be
//! spent to cast that spell. Equip {1}"
//!
//! GAP: the source filter cannot say "equipped creature" (approximated
//! as any creature, harmless since the resolution is a no-op), and the
//! steal-the-top-card impulse from an opponent's library with a
//! persistent play permission is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dream-Thief's Bandana");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{1}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "equipped creature deals combat damage"
                // has no equipped-creature source filter; approximated
                // with a broad creature source filter.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: steal_top_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…look at the top card of their library, then exile it face down.
/// For as long as it remains exiled, you may play it…"
fn steal_top_card(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exiling the top of ANOTHER player's library face down with a
    // persistent cross-player play permission (plus any-type mana) is
    // not expressible — ImpulseExile only works on the named player's
    // own library with an end-of-turn permission. Resolution is a no-op.
    Vec::new()
}
