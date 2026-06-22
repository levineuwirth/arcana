//! Mzed, Mercenary Leader — `{3}{R}` 4/2 Legendary Human Mercenary.
//!
//! Mono Eminence — ... creatures you control have haste. (GAP — a
//!   conditional static anthem keyed on command-zone presence and deck
//!   color identity; no primitive.)
//! At the beginning of your combat step, create a 1/1 red Mercenary
//!   creature token with "This creature can't block."
//! Partner. (GAP — not a modeled KeywordAbility variant.)

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mzed, Mercenary Leader");
    let human = reg.interner_mut().intern("Human");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::BeginCombat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: make_mercenary_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_mercenary_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let merc = reg.interner().lookup("Mercenary").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merc);
    // GAP: the token's printed "This creature can't block" static is not
    // expressible — the token is minted without that restriction.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: merc,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
