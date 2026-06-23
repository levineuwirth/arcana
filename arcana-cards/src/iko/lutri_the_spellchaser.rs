//! Lutri, the Spellchaser — `{1}{U/R}{U/R}` 3/2 Legendary Elemental Otter.
//! Companion (deckbuilding restriction — not a KeywordAbility; GAP).
//! Flash.
//! When Lutri enters, if you cast it, copy target instant or sorcery
//! spell you control. You may choose new targets for the copy.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lutri, the Spellchaser");
    let elemental = reg.interner_mut().intern("Elemental");
    let otter = reg.interner_mut().intern("Otter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(otter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Companion is a deckbuilding/outside-game keyword, not a
        // KeywordAbility variant.
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: "if you cast it" cast-condition gate has no conditions::
            // predicate; firing on every ETB rather than baking a fake check.
            intervening_if: None,
            effect: copy_spell,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn copy_spell(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::CopySpell { target: *id }]
}
