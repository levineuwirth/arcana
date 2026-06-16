//! Guardian of the Ages — `{7}` 7/7 Artifact Creature — Golem with Defender.
//! "When a creature attacks you or a planeswalker you control, if this creature
//! has defender, it loses defender and gains trample."

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guardian of the Ages");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if this creature has defender" — no condition
            // helper reads a source keyword; left as None (over-fires on later
            // attacks, but the grant is idempotent).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: gain_trample,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_trample(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses defender" — no keyword-removal Effect in the catalog. Only the
    // "gains trample" half is expressed (permanent grant).
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Trample,
        duration: Duration::Permanent,
    }]
}
