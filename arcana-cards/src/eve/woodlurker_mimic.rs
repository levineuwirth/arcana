//! Woodlurker Mimic — `{1}{B/G}` 2/1 black-green Shapeshifter.
//! "Whenever you cast a spell that's both black and green, this creature has
//! base power and toughness 4/5 until end of turn and gains wither until end
//! of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Woodlurker Mimic");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter {
                    colors: Some(ColorSet::black() | ColorSet::green()),
                    ..Default::default()
                }),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: on_black_green_spell,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_black_green_spell(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::SetBasePT {
            target: trig.source,
            power: 4,
            toughness: 5,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Wither,
            duration: Duration::EndOfTurn,
        },
    ]
}
