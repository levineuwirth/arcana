//! Depala, Pilot Exemplar — `{1}{R}{W}` 3/3 Legendary Dwarf Pilot (R/W).
//! Two anthem statics (Other Dwarves +1/+1; each Vehicle +1/+1 while a creature) are
//! GAP'd. The becomes-tapped "pay {X}, reveal X, take Dwarf/Vehicle cards" trigger has
//! no X-payment / reveal-N-by-type primitive in the surface, so its body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Depala, Pilot Exemplar");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(pilot);

    // GAP: static anthem "Other Dwarves you control get +1/+1."
    // GAP: static "Each Vehicle you control gets +1/+1 as long as it's a creature."

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTapped,
            intervening_if: None,
            effect: becomes_tapped,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn becomes_tapped(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}. If you do, reveal the top X cards of your library, put
    // all Dwarf and Vehicle cards into your hand, rest on bottom" — OptionalPaymentKind
    // has no {X} variant and there is no reveal-N-and-take-by-type effect.
    Vec::new()
}
