//! Auntie's Snitch — `{2}{B}` 3/1 Goblin Rogue. This creature can't block
//! (self continuous static → GAP). Prowl {1}{B} (alternative-cost keyword,
//! not expressible → GAP). Whenever a Goblin or Rogue you control deals
//! combat damage to a player, if this card is in your graveyard, you may
//! return this card to your hand.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Auntie's Snitch");
    let goblin = reg.interner_mut().intern("Goblin");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Prowl {1}{B} is an alternative-cost keyword (not expressible).
        ..Default::default()
    };

    // GAP: static "This creature can't block" — a continuous self-static,
    // not a trigger/activated ability.

    // Source: a Goblin or Rogue you control (subtype OR).
    let source_filter = arcana_core::targets::ObjectFilter::new()
        .with_subtypes_any(vec![goblin, rogue])
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                // The "if this card is in your graveyard" gate is realised by
                // restricting the trigger's active zone to the graveyard.
                intervening_if: None,
                effect: return_self_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn return_self_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may return this card to your hand." Best-effort: returns this
    // card from the graveyard to hand (the optional "may" is a resolution
    // choice; emitted as the deterministic return).
    vec![Effect::ReturnFromGraveyardToHand { target: trig.source }]
}
