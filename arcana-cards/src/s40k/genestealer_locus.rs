//! Genestealer Locus — `{3}{U}` 3/3 Creature — Tyranid Human.
//! "Neurotraumal Rod — Whenever a creature attacks you, it gets -1/-0 until end
//! of turn." "Whenever a creature attacks one of your opponents, it gets +0/+1
//! until end of turn."
//!
//! "Neurotraumal Rod" is an ability-word label (not a real keyword), so it is
//! dropped from the keyword vec. The trigger catalog has no defending-player
//! constraint on `CreatureAttacks`, so each ability fires on any creature
//! attacking and gates inside the resolver via `trig.defending_player()`.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Genestealer Locus");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: attacks_you_shrink,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: attacks_opponent_grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_you_shrink(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Only when the attack is against this ability's controller.
    if trig.defending_player() != Some(trig.controller) {
        return Vec::new();
    }
    let Some(id) = trig.attacking_creature() else { return Vec::new(); };
    vec![Effect::Pump {
        target: id,
        power: -1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn attacks_opponent_grow(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Only when the attack is against one of this controller's opponents.
    let Some(def) = trig.defending_player() else { return Vec::new(); };
    if def == trig.controller {
        return Vec::new();
    }
    let Some(id) = trig.attacking_creature() else { return Vec::new(); };
    vec![Effect::Pump {
        target: id,
        power: 0,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
