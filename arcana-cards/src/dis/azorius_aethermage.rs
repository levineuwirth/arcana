//! Azorius Aethermage — `{1}{W}{U}` 1/1 Human Wizard.
//! "Whenever a permanent is returned to your hand, you may pay {1}.
//! If you do, draw a card."
//!
//! GAP: trigger condition "whenever a permanent is returned to your
//! hand" — ZoneChange with to: Zone::Hand and from: Zone::Battlefield
//! filtering to permanents you control. The optional-pay-{1} condition
//! is also not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Azorius Aethermage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet(ColorSet::WHITE | ColorSet::BLUE),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "permanent is returned to your hand";
                // Zone::Hand form unknown from catalog (only Graveyard(0)
                // and Battlefield shown). Using ZoneChange from Battlefield
                // with best-effort Zone::Hand(0).
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: arcana_core::targets::ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Hand(0),
                },
                intervening_if: None,
                effect: on_permanent_returned,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_permanent_returned(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {1}. If you do, draw a card" — optional mana
    // payment conditioning the draw is not expressible.
    // Emitting unconditional draw as closest approximation.
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
