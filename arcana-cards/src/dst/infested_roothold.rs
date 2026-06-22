//! Infested Roothold — `{4}{G}` 0/3 Wall.
//!
//! Defender.
//! Protection from artifacts (GAP: Protection is not in the usable
//! keyword surface).
//! Whenever an opponent casts an artifact spell, you may create a 1/1
//! green Insect creature token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Infested Roothold");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);
    // Pre-intern the token subtype.
    let _insect = reg.interner_mut().intern("Insect");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        // GAP: "Protection from artifacts" — Protection not usable here.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types(TypeLine::ARTIFACT.into())),
                caster: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: make_insect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_insect(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let insect = match reg.interner().lookup("Insect") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(insect);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: insect,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
