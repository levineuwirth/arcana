//! Vernal Sovereign — `{4}{G}{W}` 4/4 green/white Elemental Elk. "Whenever
//! this creature enters or attacks, create a green and white Elemental
//! creature token with 'This token's power and toughness are each equal to
//! the number of creatures you control.'"
//!
//! "Enters or attacks" is two trigger conditions, so two `TriggeredAbilityDef`s
//! both create the token. The token's printed self-referential dynamic P/T
//! ("equal to the number of creatures you control") cannot be expressed on a
//! `TokenDefinition` (only fixed P/T) — see GAP in the token builder.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vernal Sovereign");
    let elemental = reg.interner_mut().intern("Elemental");
    let elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(elk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_elemental,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: create_elemental,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_elemental(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg
        .interner()
        .lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    // GAP: the token's printed ability "This token's power and toughness are
    // each equal to the number of creatures you control" is a self-referential
    // characteristic-defining ability; TokenDefinition only carries fixed P/T,
    // so a fixed 0/0 placeholder is used in place of the dynamic */* CDA.
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
