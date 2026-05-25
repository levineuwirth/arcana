//! Ballroom Brawlers — `{3}{W}{W}` 3/5 white Creature — Human Warrior.
//! "Whenever this creature attacks, this creature and up to one other target
//! creature you control both gain your choice of first strike or lifelink
//! until end of turn."
//!
//! GAP: "your choice of first strike or lifelink" is a modal keyword grant
//! requiring the controller to choose between two keyword options at resolution.
//! No engine API exists for modal keyword choice. We emit the trigger with the
//! correct targeting structure and grant FirstStrike as a best-effort stub.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ballroom Brawlers");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: grant_keyword_choice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn grant_keyword_choice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "your choice of first strike or lifelink" — no modal keyword-choice API.
    // Granting FirstStrike to both self and the optional target as a best-effort stub.
    let mut effects = vec![
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        },
    ];
    if let Some(target) = trig.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::FirstStrike,
                duration: Duration::EndOfTurn,
            });
        }
    }
    effects
}
