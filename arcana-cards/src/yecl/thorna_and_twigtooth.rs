//! Thorna and Twigtooth — `{1}{R}{W}{B}` 6/7 Legendary Giant Treefolk.
//! Enters with two -1/-1 counters on it.
//! Whenever it attacks, remove all counters from target creature you
//! control. Each opponent loses X life, you gain X life, and the topmost
//! creature card in your library perpetually gets +X/+X, where X is the
//! number of counters removed this way.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thorna and Twigtooth");
    let giant = reg.interner_mut().intern("Giant");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(treefolk);

    // GAP: "enters with two -1/-1 counters on it" — no enters-with primitive in
    // the documented effect/trigger surface for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_remove_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn attack_remove_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "remove ALL counters" (no all-kinds removal primitive), the dynamic
    // X = "counters removed this way" (not readable from any accessor), and the
    // "topmost creature card perpetually gets +X/+X" rider (no perpetual /
    // library-top buff effect). The entire payoff is unexpressible.
    Vec::new()
}
