//! Kestia, the Cultivator — `{1}{G}{W}{U}` Legendary 4/4 Enchantment
//! Creature — Nymph.
//! "Bestow {3}{G}{W}{U}" (aura-cast mechanic — not a supported keyword;
//! GAP).
//! "Enchanted creature gets +4/+4." (bestow aura static — GAP).
//! "Whenever an enchanted creature or enchantment creature you control
//! attacks, draw a card." (The "enchanted creature" disjunct is GAP'd in
//! the filter; the enchantment-creature-you-control case is wired.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kestia, the Cultivator");
    let nymph = reg.interner_mut().intern("Nymph");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nymph);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: "Bestow {3}{G}{W}{U}" + the bestow aura static "Enchanted creature
    // gets +4/+4" are not expressible on this creature card class.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "an enchanted creature" disjunct not expressible in the filter;
            // wired to enchantment creatures you control attacking.
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature()
                    .with_types(TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE))
                    .controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
