//! Hand of Emrakul — `{9}` 7/7 Creature — Eldrazi.
//!
//! Oracle:
//! * "You may sacrifice four Eldrazi Spawn rather than pay this spell's
//!   mana cost." — an alternative cost. GAP: alternative-cost casts via
//!   sacrifice are not expressible with the demonstrated API
//!   (OptionalPaymentKind only covers Mana/Life; this is a cast-time
//!   alternative cost, not an activated/triggered ability).
//! * "Annihilator 1" — Annihilator is NOT in the usable KeywordAbility
//!   surface, so we transcribe its reminder text as a SelfAttacks
//!   trigger: "Whenever this creature attacks, defending player
//!   sacrifices a permanent of their choice."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hand of Emrakul");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    // GAP: alternative cost "sacrifice four Eldrazi Spawn rather than pay
    // mana cost" is a cast-time alternative cost (not expressible).
    reg.register(
        CardDefinition::new(name, chars)
            // Annihilator 1: "Whenever this creature attacks, defending
            // player sacrifices a permanent of their choice."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: annihilator_sac,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn annihilator_sac(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    vec![Effect::Sacrifice {
        player: p,
        filter: ObjectFilter::permanent(),
        count: 1,
    }]
}
