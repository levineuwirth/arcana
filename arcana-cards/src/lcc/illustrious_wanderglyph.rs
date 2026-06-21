//! Illustrious Wanderglyph — `{4}{W}` 2/2 Artifact Creature — Golem.
//! Ascend — not in the usable keyword surface; GAP'd.
//! "Other artifact creatures you control get +2/+2 as long as you have the
//! city's blessing." — a conditional anthem static with no demonstrated hook;
//! GAP'd.
//! "At the beginning of each upkeep, create a 1/1 colorless Gnome artifact
//! creature token." — wired as an each-upkeep token trigger.

use arcana_core::effects::Effect;
use arcana_core::effects::TokenDefinition;
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
    let name = reg.interner_mut().intern("Illustrious Wanderglyph");
    let golem = reg.interner_mut().intern("Golem");
    let _gnome = reg.interner_mut().intern("Gnome");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ascend — not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: static "Other artifact creatures you control get +2/+2 as long as
    // you have the city's blessing." — conditional anthem not expressible here.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: make_gnome,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_gnome(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let gnome = reg.interner().lookup("Gnome").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: gnome,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
