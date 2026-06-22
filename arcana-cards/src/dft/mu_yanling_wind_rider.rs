//! Mu Yanling, Wind Rider — `{2}{U}{U}` 2/4 Legendary Human Wizard Pilot.
//!
//! Oracle:
//! * "When Mu Yanling enters, create a 3/2 colorless Vehicle artifact
//!   token with crew 1." — ETB triggered; mints a 3/2 colorless artifact
//!   Vehicle token. (GAP: the token's "crew 1" activated ability — no
//!   Crew keyword/primitive in this API; the bare token is created.)
//! * "Vehicles you control have flying." — pure static anthem; GAP'd.
//! * "Whenever one or more creatures you control with flying deal combat
//!   damage to a player, draw a card." — DamageDealt trigger
//!   (combat-only) from a flying creature you control to a player; draw 1.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mu Yanling, Wind Rider");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let pilot = reg.interner_mut().intern("Pilot");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Vehicles you control have flying" (anthem-style static).

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_vehicle_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_keyword(KeywordAbility::Flying),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_vehicle_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vehicle = reg.interner().lookup("Vehicle").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vehicle);
    // GAP: the token's "crew 1" activated ability is not expressible (no Crew
    // keyword); the 3/2 colorless artifact Vehicle token itself is created.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: vehicle,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
