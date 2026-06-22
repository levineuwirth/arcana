//! Tiller Engine — `{2}` 1/3 Artifact Creature — Construct.
//! "Whenever a land you control enters tapped, choose one — Untap that
//! land. / Tap target nonland permanent an opponent controls."
//!
//! Modeled as one triggered ability that fires when a land you control
//! enters the battlefield and untaps it (the first mode). The engine's
//! triggered-ability API does not carry a modal "choose one" selector
//! (modal is a spell-ability-only construct here), and there is no
//! "enters tapped" trigger qualifier, so the tapped precondition and the
//! second mode (tap target opponent's nonland permanent) are gapped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tiller Engine");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "enters TAPPED" — no enters-tapped trigger qualifier;
            // fires on any land you control entering.
            // GAP: modal "choose one" — triggered abilities have no modal
            // selector here; only the "untap that land" mode is wired, and
            // the "tap target nonland permanent an opponent controls" mode
            // is omitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: untap_that_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn untap_that_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    vec![Effect::Untap { target: id }]
}
