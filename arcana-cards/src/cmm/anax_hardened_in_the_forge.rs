//! Anax, Hardened in the Forge — `{1}{R}{R}` */3 Legendary Enchantment
//! Creature — Demigod.
//! "Anax's power is equal to your devotion to red." — GAP (characteristic-
//!   defining static; power modeled as `*`).
//! "Whenever Anax or another nontoken creature you control dies, create a
//!   1/1 red Satyr creature token with 'This token can't block.' If the
//!   creature had power 4 or greater, create two of those tokens instead."
//!
//! The death trigger fires on a nontoken creature you control dying
//! (battlefield→graveyard). The token count is dynamic: two tokens if the
//! dying creature had power 4+, else one.
//!
//! GAP: the token's "This token can't block." ability can't be attached
//! via TokenDefinition; the bare 1/1 red Satyr is created.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anax, Hardened in the Forge");
    let demigod = reg.interner_mut().intern("Demigod");
    // Pre-intern the token subtype so the effect fn's read-only lookup works.
    let _satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demigod);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: "power equal to your devotion to red" — a CDA; modeled as `*`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .nontoken(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: make_satyrs,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_satyrs(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let satyr = reg.interner().lookup("Satyr").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);

    let make = |subtypes: SubtypeSet| Effect::CreateToken {
        controller: trig.controller,
        // GAP: token's "This token can't block." ability unattachable.
        token: TokenDefinition {
            name: satyr,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    };

    let dying = trig.dying_object().unwrap_or(trig.source);
    let pw = script::power_of(state, dying);
    if pw >= 4 {
        vec![make(subtypes.clone()), make(subtypes)]
    } else {
        vec![make(subtypes)]
    }
}
