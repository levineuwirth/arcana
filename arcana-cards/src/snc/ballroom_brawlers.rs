//! Ballroom Brawlers — `{3}{W}{W}` 3/5 white Human Warrior.
//! "Whenever this creature attacks, this creature and up to one other
//! target creature you control both gain your choice of first strike or
//! lifelink until end of turn."
//!
//! GAP: effect — "your choice of first strike or lifelink" modal keyword
//! grant is not expressible; granting both as a best effort.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
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
                effect: grant_keyword,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn grant_keyword(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "your choice of first strike or lifelink" modal choice not expressible;
    // granting first strike to self and to the target.
    let mut effects = vec![
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        },
    ];
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
