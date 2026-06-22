//! Defiler of Vigor — `{3}{G}{G}` 6/6 Phyrexian Wurm.
//! Trample.
//! As an additional cost to cast green permanent spells, you may pay 2 life;
//! those spells cost {G} less if you paid life this way. — GAP: alternative
//! additional-cost / cost-reduction static not expressible.
//! Whenever you cast a green permanent spell, put a +1/+1 counter on each
//! creature you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Defiler of Vigor");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(wurm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: "As an additional cost to cast green permanent spells, you may pay 2
    // life. Those spells cost {G} less if you paid life this way." — an optional
    // additional cost paired with a self-applied cost reduction static is not
    // expressible with the available primitives.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                // "green permanent spell" approximated as a green, non-instant,
                // non-sorcery spell (permanent types only).
                filter: Some(
                    ObjectFilter::new()
                        .with_colors(ColorSet::green())
                        .without_types(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                ),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: counters_on_each_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counters_on_each_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
