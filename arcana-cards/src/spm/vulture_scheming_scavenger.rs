//! Vulture, Scheming Scavenger — `{5}{U/B}` 4/6 Legendary Human Artificer
//! Villain with Flying. "Whenever Vulture attacks, other Villains you
//! control gain flying until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vulture, Scheming Scavenger");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    subtypes.0.insert(villain);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: other_villains_gain_flying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn other_villains_gain_flying(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Villain")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .filter(|&id| id != trig.source)
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
