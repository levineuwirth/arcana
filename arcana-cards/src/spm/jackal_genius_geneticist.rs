//! Jackal, Genius Geneticist — `{G}{U}` 1/1 Legendary Human Scientist Villain
//! with Trample.
//! "Whenever you cast a creature spell with mana value equal to Jackal's power,
//! copy that spell, except the copy isn't legendary. Then put a +1/+1 counter on
//! Jackal."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jackal, Genius Geneticist");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let creature_spell = ObjectFilter::new().with_types(TypeLine::CREATURE.into());

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (condition): "with mana value equal to Jackal's power" cannot be
            // expressed as a static SpellCast filter (compares the cast spell's MV
            // to this creature's current power); fires on any creature spell.
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(creature_spell),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: copy_then_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn copy_then_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy that spell, except the copy isn't legendary" — no accessor exposes
    // the triggering spell's stack id, so CopySpell can't be targeted at it.
    // The "+1/+1 counter on Jackal" rider IS expressible:
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
