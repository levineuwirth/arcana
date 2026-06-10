//! Desecrated Tomb — {3} artifact (Core Set 2019, 2018).
//! "Whenever one or more creature cards leave your graveyard, create
//! a 1/1 black Bat creature token with flying." Approximated as a
//! graveyard-to-battlefield zone-change trigger (the most common way
//! creature cards leave a graveyard) minting the Bat.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Desecrated Tomb");
    // Pre-intern the token subtype for resolve-time lookup.
    let _bat = reg.interner_mut().intern("Bat");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "whenever one or more creature cards LEAVE your
                // graveyard" — ZoneChange requires a concrete `to` zone;
                // modeled as graveyard -> battlefield only (reanimation),
                // missing exile/hand/library exits, and fires per card
                // rather than once per batch.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: make_bat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn make_bat(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bat = reg.interner().lookup("Bat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: bat,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
