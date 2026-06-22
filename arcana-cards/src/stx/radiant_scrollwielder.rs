//! Radiant Scrollwielder — `{2}{R}{W}` 2/4 Creature — Dwarf Cleric.
//! Instant and sorcery spells you control have lifelink.
//! At the beginning of your upkeep, exile an instant or sorcery card at random
//! from your graveyard. You may cast it this turn. If a spell cast this way
//! would be put into your graveyard, exile it instead.
//!
//! GAP (static): "Instant and sorcery spells you control have lifelink" is a
//! pure continuous ability granting lifelink to a class of spells — not a
//! triggered/activated ability and not expressible via the documented surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Radiant Scrollwielder");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_exile_recast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_exile_recast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile an instant or sorcery card at random from your graveyard; you
    // may cast it this turn; if it would be put into your graveyard, exile it
    // instead." There is no exile-random-from-graveyard-with-play-permission
    // effect (ImpulseExile only operates on the top of the library), and the
    // replacement rider ("exile it instead of graveyard") is not expressible.
    Vec::new()
}
