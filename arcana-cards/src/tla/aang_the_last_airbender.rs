//! Aang, the Last Airbender — `{3}{W}` 3/2 Legendary Creature — Human Avatar Ally.
//!
//! * Flying (keyword).
//! * "When Aang enters, airbend up to one other target nonland permanent. (Exile
//!   it. While it's exiled, its owner may cast it for {2} …)" — modeled as an
//!   ETB trigger that EXILES the chosen nonland permanent. GAP (fidelity): the
//!   "owner may cast it for {2}" alternate-cast permission (the Airbend mechanic)
//!   is not expressible; only the exile is modeled.
//! * "Whenever you cast a Lesson spell, Aang gains lifelink until end of turn." —
//!   a `SpellCast` (you, Lesson subtype) trigger granting lifelink EOT.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aang, the Last Airbender");
    let human = reg.interner_mut().intern("Human");
    let avatar = reg.interner_mut().intern("Avatar");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(avatar);
    subtypes.0.insert(ally);

    let lesson_filter = script::subtype_filter(reg, "Lesson");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: airbend_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(lesson_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_lifelink,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn airbend_exile(
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
    // GAP (fidelity): Airbend's "owner may cast it for {2}" alternate-cast
    // permission is unmodeled; we exile the permanent only.
    vec![Effect::ExilePermanent { target: *id }]
}

fn gain_lifelink(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Lifelink,
        duration: Duration::EndOfTurn,
    }]
}
