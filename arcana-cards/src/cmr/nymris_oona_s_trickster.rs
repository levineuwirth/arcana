//! Nymris, Oona's Trickster — `{3}{U}{B}` 1/6 Legendary Creature —
//! Faerie Knight with Flash and Flying.
//!
//! Oracle:
//! * Flash, Flying — base keywords.
//! * Whenever you cast your first spell during each opponent's turn, look at the
//!   top two cards of your library. Put one of those cards into your hand and the
//!   other into your graveyard. — modeled as a `SpellCast` (caster: You) trigger
//!   whose effect digs the top two (`DigTopN` count 2, take one to hand, rest to
//!   graveyard). The "first spell during each opponent's turn" gating (only your
//!   first such cast, and only on an opponent's turn) is not expressible — the
//!   `SpellCast` condition has no per-opponent-turn / first-only frequency — so
//!   the trigger over-fires (it fires on each spell you cast); noted inline.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nymris, Oona's Trickster");
    let faerie = reg.interner_mut().intern("Faerie");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "your first spell during each opponent's turn" gating is
                // not expressible (no per-opponent-turn first-only frequency);
                // this fires on each spell you cast.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: dig_top_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Look at the top two cards: put one into your hand, the other into your
/// graveyard.
fn dig_top_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 2,
        filter: None,
        rest: DigRest::Graveyard,
    }]
}
