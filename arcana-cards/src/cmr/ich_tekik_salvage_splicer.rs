//! Ich-Tekik, Salvage Splicer — `{4}{G}` 1/1 Legendary Phyrexian Human
//! Artificer.
//! - "When Ich-Tekik enters, create a 3/3 colorless Phyrexian Golem artifact
//!   creature token."
//! - "Whenever an artifact is put into a graveyard from the battlefield, put
//!   a +1/+1 counter on Ich-Tekik and a +1/+1 counter on each Golem you
//!   control."
//! - Partner is not in the usable keyword surface — GAP'd (keywords empty).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Ich-Tekik, Salvage Splicer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let _golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Partner not in usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_golem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: artifact_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: add_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_golem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golem = reg.interner().lookup("Golem").unwrap_or_default();
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(golem);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: golem,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn add_counters(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golems = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Golem").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::ForEach {
            targets: golems,
            effect: Box::new(Effect::AddCounters {
                target: arcana_core::objects::NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
    ]
}
