//! Gleaming Overseer — `{1}{U}{B}` 1/4 Zombie Wizard.
//! "When this creature enters, amass Zombies 1.
//!  Zombie tokens you control have hexproof and menace."
//!
//! 1. ETB trigger → `Effect::Amass` with count 1 and race "Zombie"
//!    (army subtype "Army"). Expressible.
//! 2. GAP: "Zombie tokens you control have hexproof and menace." — a static
//!    continuous ability granting keywords to a filtered set of OTHER
//!    permanents; this is neither a triggered nor an activated ability and
//!    has no granted-static primitive in the demonstrated API, so it is
//!    GAP'd. (Scryfall's "Amass" keyword entry is the ETB action, not a
//!    static creature keyword — `keywords` stays empty.)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Gleaming Overseer");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    // Pre-intern the Army subtype so the resolver's lookup succeeds (Zombie
    // is already interned above for this card's own subtype).
    let _army = reg.interner_mut().intern("Army");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Zombie tokens you control have hexproof and menace."
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_amass_zombies,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_amass_zombies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Zombie").unwrap_or_default();
    vec![Effect::Amass {
        controller: trig.controller,
        count: 1,
        army_subtype,
        race_subtype,
    }]
}
