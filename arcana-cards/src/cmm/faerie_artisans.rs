//! Faerie Artisans — `{3}{U}` 2/2 Creature — Faerie Artificer. Mono-blue.
//!
//! Oracle:
//! - Flying — keyword.
//! - "Whenever a nontoken creature an opponent controls enters, create a token
//!   that's a copy of that creature except it's an artifact in addition to its
//!   other types. Then exile all other tokens created with this creature." —
//!   ZoneChange trigger (nontoken creature an opponent controls → battlefield)
//!   minting a token copy of the entering creature via `CopyPermanent`.
//!   GAP: the "except it's an artifact" rider and the "exile all other tokens
//!   created with this creature" provenance cleanup are not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Faerie Artisans");
    let faerie = reg.interner_mut().intern("Faerie");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .nontoken()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: copy_entering_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_entering_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else { return Vec::new(); };
    // GAP: "except it's an artifact in addition to its other types" rider, and
    // "Then exile all other tokens created with this creature" provenance
    // cleanup, are not expressible — emit the plain token copy.
    vec![Effect::CopyPermanent { target: id }]
}
