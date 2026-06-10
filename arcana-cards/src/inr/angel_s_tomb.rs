//! Angel's Tomb — `{3}` artifact (Magic Origins).
//! "Whenever a creature you control enters, you may have this
//! artifact become a 3/3 white Angel artifact creature with flying
//! until end of turn." Animation is AddType(CREATURE) + SetBasePT 3/3
//! + SetColor white + GrantKeyword Flying, all until end of turn.
//!
//! GAP: "you may have" — the animation is modeled as mandatory (no
//! optional-trigger prompt). GAP: gaining the Angel SUBTYPE is not
//! expressible (AddType covers card types only).

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angel's Tomb");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: animate_tomb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn animate_tomb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may have" — modeled as mandatory animation.
    // GAP: "Angel" subtype grant not expressible.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: trig.source,
            colors: ColorSet::white(),
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
    ]
}
