//! Astor, Bearer of Blades — `{2}{R}{W}` 4/4 Legendary Human Warrior.
//! "When Astor enters, look at the top seven cards of your library. You
//!  may reveal an Equipment or Vehicle card from among them and put it
//!  into your hand. Put the rest on the bottom of your library in a
//!  random order.
//!  Equipment you control have equip {1}.
//!  Vehicles you control have crew 1."

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Astor, Bearer of Blades");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    // Pre-intern the dig subtypes for resolution-time filter rebuild.
    let _equipment = reg.interner_mut().intern("Equipment");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: static "Equipment you control have equip {1}" — ability-granting
    //      to other permanents has no expressible Effect on this card class.
    // GAP: static "Vehicles you control have crew 1" — same.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: dig_for_gear,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dig_for_gear(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Equipment") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Vehicle") {
        syms.push(s);
    }
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 7,
        filter: Some(ObjectFilter::new().with_subtypes_any(syms)),
        rest: DigRest::BottomRandom,
    }]
}
