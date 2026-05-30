//! Druid of Purification — `{3}{G}` 2/3 green Creature — Human Druid.
//! "When this creature enters, starting with you, each player may choose an artifact or
//! enchantment you don't control. Destroy each permanent chosen this way."
//! GAP: The "starting with you, each player may choose" group-choice loop with per-player
//! "you don't control" constraint is not expressible with the current Effect catalog
//! (no multi-player sequential-choice effect with dynamic controller-constraint filtering).
//! Modeled as best-effort: target up to one artifact-or-enchantment not controlled by you.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Druid of Purification");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy_artifact_or_enchantment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: full oracle is "each player may choose" sequential loop — only one target here
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_destroy_artifact_or_enchantment(
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
    vec![Effect::DestroyPermanent { target: *id }]
}
