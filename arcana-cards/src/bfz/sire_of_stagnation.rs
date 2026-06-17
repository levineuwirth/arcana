//! Sire of Stagnation — `{4}{U}{B}` 5/7 Eldrazi with Devoid (colorless).
//! Whenever a land an opponent controls enters, that player exiles the top
//! two cards of their library and you draw two cards.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sire of Stagnation");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        // Devoid: this card has no color.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::Opponent),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: on_opponent_land_enters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_opponent_land_enters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" = the entering land's controller.
    let Some(them) = trig
        .entering_object()
        .and_then(|id| state.objects.get(id))
        .map(|o| o.controller)
    else {
        return Vec::new();
    };
    // GAP: oracle exiles the top two cards; no exile-from-top-of-library
    // primitive exists — Mill (to graveyard) is the closest available.
    vec![
        Effect::Mill { player: them, count: 2 },
        Effect::DrawCards { player: trig.controller, count: 2 },
    ]
}
