//! Octavia, Living Thesis — `{8}{U}{U}` 8/8 Legendary Elemental Octopus.
//! This spell costs {8} less if you have eight+ instant/sorcery cards in
//! your graveyard. (GAP'd — static cost reduction.)
//! Ward {8}.
//! Magecraft — Whenever you cast or copy an instant or sorcery spell,
//! target creature has base power and toughness 8/8 until end of turn.
//! (the "or copy" branch is a fidelity gap — only cast is matched.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Octavia, Living Thesis");
    let elemental = reg.interner_mut().intern("Elemental");
    let octopus = reg.interner_mut().intern("Octopus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(octopus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{8}").expect("valid cost"),
        )],
        // GAP: "costs {8} less if 8+ instant/sorcery cards in graveyard"
        // is a static cost-reduction effect.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                    TypeLine::INSTANT | TypeLine::SORCERY,
                ))),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: set_base_pt,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn set_base_pt(
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
    vec![Effect::SetBasePT {
        target: *id,
        power: 8,
        toughness: 8,
        duration: Duration::EndOfTurn,
    }]
}
