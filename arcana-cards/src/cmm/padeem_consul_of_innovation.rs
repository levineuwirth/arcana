//! Padeem, Consul of Innovation — `{3}{U}` 1/4 Legendary Vedalken Artificer.
//!
//! Oracle:
//! * Artifacts you control have hexproof.
//! * At the beginning of your upkeep, if you control the artifact with the
//!   greatest mana value or tied for the greatest mana value, draw a card.
//!
//! GAPs:
//! * "Artifacts you control have hexproof" is a pure static continuous
//!   ability with no trigger word or cost — not a triggered/activated ability
//!   and not expressible in this card class. Omitted (recorded here).
//! * The upkeep trigger's intervening-if ("if you control the artifact with
//!   the greatest mana value or tied for the greatest") cannot be expressed
//!   with the available `conditions::` predicates (there is no
//!   greatest-mana-value-among-a-type helper), so `intervening_if` is left
//!   `None` (GAP) and the draw fires unconditionally — a fidelity gap, not a
//!   silently-baked check.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Padeem, Consul of Innovation");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "if you control the artifact with the
                // greatest mana value (or tied)" — no conditions:: predicate
                // expresses greatest-mana-value-among-a-type; left None.
                intervening_if: None,
                effect: upkeep_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
