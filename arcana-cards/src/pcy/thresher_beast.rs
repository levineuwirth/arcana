//! Thresher Beast — `{3}{G}{G}` 4/4 green Beast.
//! "Whenever this creature becomes blocked, defending player sacrifices a land
//! of their choice."
//! GAP: trigger — no variant for "becomes blocked"; using SelfAttacks as
//! closest approximation.
//! GAP: effect — "defending player sacrifices a land" — Sacrifice effect uses
//! ObjectFilter::creature() by default; land sacrifice by a specific player
//! not fully expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thresher Beast");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_blocked(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: trigger — "becomes blocked" not in catalog; fires on attack instead.
    // GAP: effect — defending player sacrifices a land; using opponent sacrifice land filter.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter {
            types_any: Some(TypeLine(TypeLine::LAND)),
            ..Default::default()
        },
        count: 1,
    }]
}
