//! Hypnotic Swirly Disc — artifact — Contraption (Unstable, 2017).
//! "Whenever you crank this Contraption, target player mills X cards,
//! where X is the number of creatures you control."
//!
//! GAP: the crank/assemble Contraption mechanic is not modeled — the
//! trigger uses the closest available condition (SelfBecomesTapped).
//! The mill amount is dynamic and computed at resolution.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Hypnotic Swirly Disc");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — 'whenever you crank this Contraption'
                // (crank is not modeled; SelfBecomesTapped is the
                // closest available condition).
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: crank_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            },
        ),
    )
}

fn crank_mill(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::Mill { player: *p, count: x }]
}
