//! Goldvein Hydra — `{X}{G}` 0/0 green Hydra with Vigilance, Trample, Haste.
//!
//! Oracle:
//! * Vigilance, trample, haste.
//! * This creature enters with X +1/+1 counters on it. (CR 121.6a)
//! * When this creature dies, create a number of tapped Treasure tokens
//!   equal to its power.
//!
//! Keywords land on `characteristics.keywords`. The dies-trigger reads the
//! creature's power at the moment it died and mints that many Treasure
//! tokens via `Effect::CreateCommodityToken`.
//!
//! GAP: "enters with X +1/+1 counters on it" — the enters-with-counters
//! face spec is not part of the documented effect/keyword surface, so the
//! ETB counter placement is omitted. The bones P/T (0/0) are transcribed
//! verbatim.
//! GAP: the Treasures are minted UNtapped — there is no "create tapped
//! Treasure" form in `Effect::CreateCommodityToken`; the count is faithful.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goldvein Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_make_treasures,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// On death, create Treasure tokens equal to this creature's last power.
fn dies_make_treasures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let n = script::power_of(state, id).max(0) as u32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: n,
    }]
}
