//! Neriv, Crackling Vanguard — `{2}{R}{W}{B}` 4/4 Legendary red-white-black
//! Spirit Dragon.
//!
//! Oracle:
//! * Flying, deathtouch.
//! * "When Neriv enters, create two 1/1 red Goblin creature tokens." — an
//!   ETB trigger minting two 1/1 red Goblin tokens (two CreateToken).
//! * "Whenever Neriv attacks, exile a number of cards from the top of your
//!   library equal to the number of differently named tokens you control.
//!   During any turn you attacked with a commander, you may play those
//!   cards." — a SelfAttacks trigger. GAP (effect body): the dynamic count
//!   ("differently named tokens you control") has no script helper to count
//!   distinct token names, so the impulse-exile amount is not computable;
//!   the commander-attack play-permission window is likewise unmodeled.
//!   Emitting a literal count would be unfaithful, so the effect is omitted.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Neriv, Crackling Vanguard");
    let spirit = reg.interner_mut().intern("Spirit");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_goblins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_impulse_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_goblins(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let token = TokenDefinition {
        name: goblin,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn attack_impulse_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile cards equal to the number of differently named tokens you
    // control" — no script helper counts distinct token names, so the
    // ImpulseExile count is not computable; the commander-attack play window
    // is also unmodeled. Effect omitted rather than emit a literal count.
    Vec::new()
}
