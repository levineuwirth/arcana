//! Preacher of the Schism — `{2}{B}` 2/4 Creature — Vampire Cleric. B.
//! Deathtouch.
//! "Whenever this creature attacks the player with the most life or tied
//! for most life, create a 1/1 white Vampire creature token with lifelink."
//! "Whenever this creature attacks while you have the most life or are tied
//! for most life, you draw a card and you lose 1 life."
//! Both use SelfAttacks. GAP (fidelity): the "most life / tied for most"
//! intervening conditions have no expressible predicate, so both bodies
//! fire on every attack — documented deviation.

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
    let name = reg.interner_mut().intern("Preacher of the Schism");
    let vampire = reg.interner_mut().intern("Vampire");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP (fidelity): "attacks the player with the most life or tied"
            // gate not expressible — fires on every attack.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_vampire_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP (fidelity): "while you have the most life or are tied" gate
            // not expressible — fires on every attack.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: draw_and_lose,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_vampire_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: vampire,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Lifelink],
            abilities: vec![],
        },
    }]
}

fn draw_and_lose(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}
