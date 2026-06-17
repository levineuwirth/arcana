//! Turncoat Kunoichi — `{2}{W}` 3/2 Mutant Ninja Fox.
//!
//! Sneak {2}{W}{B} — an alternative cast cost; `Sneak` is not a usable
//! `KeywordAbility` in this surface, so `keywords: vec![]` and the sneak
//! cast mode is unmodeled.
//!
//! "When this creature enters, choose target creature an opponent
//! controls. Exile that creature until this creature leaves the
//! battlefield. If this creature's sneak cost was paid, instead exile
//! the chosen creature [permanently]." — modeled as the base
//! O-ring (exile-until-leaves) form. GAP: the "if sneak was paid, exile
//! permanently instead" branch can't be conditioned on the cast mode.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Turncoat Kunoichi");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let fox = reg.interner_mut().intern("Fox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(fox);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exile_until_leaves,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn exile_until_leaves(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "if sneak cost was paid, exile permanently instead" — the
    // cast-mode condition isn't observable; modeled as exile-until-leaves.
    vec![Effect::ExileUntilSourceLeaves {
        source: trig.source,
        target: *id,
    }]
}
