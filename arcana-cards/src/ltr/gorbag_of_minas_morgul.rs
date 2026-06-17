//! Gorbag of Minas Morgul — `{1}{B}` 2/2 Legendary Orc Soldier.
//!
//! * "Treasure" is not a real keyword on this card (it tags the Treasure-token
//!   payoff) — `keywords` is empty.
//! * Whenever a Goblin or Orc you control deals combat damage to a player, you
//!   may sacrifice it. When you do, choose one — Draw a card / Create a Treasure
//!   token. The trigger condition is expressible (combat damage to a player by a
//!   Goblin-or-Orc you control); the reflexive "sacrifice it, then choose one"
//!   modal payoff is not, so the effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gorbag of Minas Morgul");
    let orc_sub = reg.interner_mut().intern("Orc");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc_sub);
    subtypes.0.insert(soldier);

    // Source filter: a Goblin or Orc creature you control (subtype OR).
    let goblin = reg.interner_mut().intern("Goblin");
    let orc = reg.interner_mut().intern("Orc");
    let source_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![goblin, orc]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: on_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_combat_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice it. When you do, choose one — Draw a card /
    // Create a Treasure token." This is a reflexive (sacrifice-then-modal)
    // trigger: the sacrifice is an optional cost-like step that itself spawns a
    // second "When you do" modal choice. There is no primitive composing an
    // optional sacrifice-of-the-triggering-creature with a follow-up modal
    // dispatch, so the payoff is omitted.
    Vec::new()
}
