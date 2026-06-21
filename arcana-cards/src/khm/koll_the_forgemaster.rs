//! Koll, the Forgemaster — `{R}{W}` 2/2 Legendary Creature — Dwarf Warrior.
//!
//! Oracle:
//! * Whenever another nontoken creature you control dies, if it was enchanted or
//!   equipped, return it to its owner's hand. — modeled as a `ZoneChange`
//!   (battlefield → graveyard) trigger over your nontoken creatures, returning
//!   the dying card to its owner's hand. The intervening-if "if it was enchanted
//!   or equipped" is not expressible (no enchanted/equipped condition helper),
//!   so per guidance the gate is left `None` (the trigger over-fires — a known
//!   fidelity gap, noted inline).
//! * Creature tokens you control that are enchanted or equipped get +1/+1. — a
//!   STATIC continuous anthem over your tokens; not a triggered/activated
//!   ability and not expressible here. GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koll, the Forgemaster");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(warrior);

    let nontoken_creature = arcana_core::targets::ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Creature tokens you control that are enchanted or equipped
    // get +1/+1" (board-wide conditional anthem over your tokens).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: intervening-if "if it was enchanted or equipped" left
                // None (no enchanted/equipped condition); trigger over-fires.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: nontoken_creature,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: dying_creature_to_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Return the dying creature card to its owner's hand.
fn dying_creature_to_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: id }]
}
