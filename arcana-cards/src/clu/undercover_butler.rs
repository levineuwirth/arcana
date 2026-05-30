//! Undercover Butler — `{2}{U/B}` 2/3 blue/black Human Rogue.
//! "Whenever this creature attacks the player with the most life or tied for most
//! life, it can't be blocked this turn."
//!
//! GAP: The trigger condition "attacks the player with the most life or tied for
//! most life" is not expressible as a TriggerCondition variant — there is no
//! `SelfAttacksDefender` variant that can filter on the defending player's life
//! total. Using SelfAttacks as the closest available condition and noting the gap.
//! The CantBeBlocked effect on trig.source is faithfully represented.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Undercover Butler");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "attacks the player with the most life or tied for most
                // life" — no TriggerCondition variant can filter on the defending player's
                // life total. Using SelfAttacks as a superset approximation.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: cant_be_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cant_be_blocked(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::EndOfTurn,
    }]
}
