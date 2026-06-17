//! Deathpact Angel — `{3}{W}{B}{B}` 5/5 Angel with Flying.
//! "When this creature dies, create a 1/1 white and black Cleric creature
//! token. It has '{3}{W}{B}{B}, {T}, Sacrifice this token: Return a card named
//! Deathpact Angel from your graveyard to the battlefield.'"

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathpact Angel");
    let angel = reg.interner_mut().intern("Angel");
    let _cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cleric = reg.interner().lookup("Cleric").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cleric);
    // GAP: the token's printed activated ability ("{3}{W}{B}{B}, {T},
    // Sacrifice this token: return a card named Deathpact Angel from your
    // graveyard to the battlefield") can't be attached — TokenDefinition only
    // carries triggered abilities, not activated ones.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: cleric,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
