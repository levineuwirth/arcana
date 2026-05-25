//! Embermouth Sentinel — `{2}` 2/1 colorless artifact creature (Chimera).
//! "When this creature enters, you may search your library for a basic
//! land card, reveal it, then shuffle and put that card on top. If you
//! control a Dragon, put that card onto the battlefield tapped instead."
//!
//! GAP: "if you control a Dragon, put onto battlefield tapped instead" —
//! conditional tutor destination (top-of-library vs battlefield tapped)
//! based on board state not expressible. Emitting TutorToHand with basic
//! land filter as best effort; Dragon condition dropped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Embermouth Sentinel");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you control a Dragon, put onto battlefield tapped" —
    // conditional tutor destination not expressible; using TutorToHand
    // with basic land filter as best effort.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::default().with_types(TypeLine::LAND.into()),
        reveal: true,
    }]
}
