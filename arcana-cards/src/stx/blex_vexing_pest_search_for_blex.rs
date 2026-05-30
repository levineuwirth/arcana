//! Blex, Vexing Pest // Search for Blex — `{2}{G}` Legendary Creature — Pest 3/2.
//!
//! Front face: Other Pests, Bats, Insects, Snakes, and Spiders you control get +1/+1.
//! When Blex dies, you gain 4 life.
//!
//! Back face (Sorcery, `{3}{B}`): Look at the top five cards of your library.
//! You may put any number of them into your hand and the rest into your graveyard.
//! You lose 3 life for each card you put into your hand this way.
//!
//! GAP: "Other Pests, Bats, Insects, Snakes, and Spiders you control get +1/+1" is a
//! static pump effect on multiple subtypes — no static continuous-effect API is available.
//!
//! GAP: Search for Blex — "put any number into your hand, lose 3 life for each" — the
//! variable-pick-any-number shape is not expressible with DigTopN (single-take only).
//! Back face effect returns Vec::new().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blex, Vexing Pest");
    let pest_sub = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face — Search for Blex
    let back_name = reg.interner_mut().intern("Search for Blex");
    let back_chars = Characteristics {
        name: back_name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let back_ability = SpellAbilityDef {
        text: "Look at the top five cards of your library. You may put any number of them into your hand and the rest into your graveyard. You lose 3 life for each card you put into your hand this way.".into(),
        target_requirements: vec![],
        modal: None,
        effect: search_for_blex_resolve,
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: Some(back_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back_face)
            // Triggered ability: when Blex dies, gain 4 life
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: blex_dies_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn blex_dies_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 4,
    }]
}

fn search_for_blex_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put any number into your hand and the rest into graveyard, lose 3 life for each"
    // is not expressible — DigTopN only supports a single-take pick; variable-pick-any-number
    // with a life-loss scaling is not modeled.
    Vec::new()
}
