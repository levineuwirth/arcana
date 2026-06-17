//! Ketramose, the New Dawn — `{1}{W}{B}` 4/4 Legendary God.
//! "Menace, lifelink, indestructible.
//!  Ketramose can't attack or block unless there are seven or more cards in
//!  exile. (restriction — GAP'd)
//!  Whenever one or more cards are put into exile from graveyards and/or the
//!  battlefield during your turn, you draw a card and lose 1 life."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ketramose, the New Dawn");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Menace,
            KeywordAbility::Lifelink,
            KeywordAbility::Indestructible,
        ],
        ..Default::default()
    };

    // GAP: "can't attack or block unless seven or more cards in exile" is a
    // conditional combat restriction — not expressible as a triggered/activated
    // ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever one or more cards are put into exile from
                // graveyards and/or the battlefield during your turn" — modeled
                // as a card moving from the battlefield to exile, controlled by
                // you (partial: graveyard-to-exile and the during-your-turn gate
                // are not both expressible in a single ZoneChange).
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::default()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Exile,
                },
                intervening_if: None,
                effect: draw_and_lose,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_and_lose(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}
