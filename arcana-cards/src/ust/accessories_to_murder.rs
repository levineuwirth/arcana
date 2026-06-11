//! Accessories to Murder — artifact — Contraption (Un-set, no mana
//! cost). "Whenever you crank this Contraption, target creature gets
//! +X/+0 until end of turn, where X is the number of creatures you
//! control."
//!
//! GAP: trigger — "whenever you crank this Contraption" is unmodeled
//! (no crank event); `SelfBecomesTapped` is the closest stand-in. The
//! pump itself is faithful: X is computed at resolution.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Accessories to Murder");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — crank is unmodeled; SelfBecomesTapped is
                // the closest stand-in.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: murder_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

/// "…target creature gets +X/+0 until end of turn, where X is the
/// number of creatures you control."
fn murder_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::Pump {
        target: *id,
        power: n as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
