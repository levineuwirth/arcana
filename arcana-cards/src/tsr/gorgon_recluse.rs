//! Gorgon Recluse — `{3}{B}{B}` 2/4 Gorgon.
//!
//! Oracle:
//! * "Whenever this creature blocks or becomes blocked by a nonblack
//!   creature, destroy that creature at end of combat." — modeled with
//!   `SelfBlocksOrBecomesBlocked`, reading the other combatant via
//!   `trig.other_combatant()` and destroying it. GAPs: the "nonblack"
//!   restriction (the trigger condition is a unit, no filter), and the "at
//!   end of combat" delay (no end-of-combat DelayedWhen) — destruction is
//!   applied on resolution instead.
//! * Madness {B}{B} — not in the usable keyword surface; GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Gorgon Recluse");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);

    // GAP: Madness {B}{B} — not in the usable keyword surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: destroy_combatant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn destroy_combatant(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "nonblack" creature restriction (no filter on the condition) and
    // "at end of combat" delay (no end-of-combat DelayedWhen) — destroyed on
    // resolution.
    let Some(id) = trig.other_combatant() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: id }]
}
