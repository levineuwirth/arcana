//! Weapons Manufacturing — `{1}{R}` enchantment.
//! "Whenever a nontoken artifact you control enters, create a colorless
//! artifact token named Munitions with 'When this token leaves the
//! battlefield, it deals 2 damage to any target.'"

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weapons Manufacturing");
    let _munitions = reg.interner_mut().intern("Munitions");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: mint_munitions,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a colorless artifact token named Munitions …"
fn mint_munitions(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let munitions = reg.interner().lookup("Munitions").unwrap_or_default();
    // GAP: the token's printed triggered ability ("When this token leaves the
    // battlefield, it deals 2 damage to any target") cannot be authored on a
    // TokenDefinition — the bare token is minted without it.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: munitions,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
